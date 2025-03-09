mod fal_data;
mod helpers;
mod module;
mod types;

use fal_data::*;
use helpers::*;
use module::*;
use types::*;

use std::collections::{HashMap, HashSet};

use cargo_manifest::Manifest;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let base_module_path = "src/endpoints";

    let mut root = Node::Module {
        name: "endpoints".to_string(),
        children: Vec::new(),
    };
    let mut req_res_types: HashMap<String, serde_json::Value> = HashMap::new();

    let client = reqwest::Client::new();

    let models: Vec<Model> = client
        .get("https://fal.ai/api/models")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    // De-duplicate, because the /models endpoint returns multiple endpoints within an alias
    let mut visited_aliases = HashSet::new();

    for model in &models {
        let (owner, endpoint) = model
            .endpoint_id
            .split_once("/")
            .expect(&format!("could not split endpoint: {}", &model.endpoint_id));

        // URL encode the alias to handle special characters in the endpoint
        let parts = endpoint.split("/").collect::<Vec<&str>>();
        let alias = parts[0];

        if visited_aliases.contains(alias) {
            continue;
        }

        visited_aliases.insert(alias);

        let encoded_alias = utf8_percent_encode(alias, NON_ALPHANUMERIC).to_string();

        let app_data: AppData = client
            .get(format!(
                "https://fal.ai/api/models/app-data?owner={owner}&alias={encoded_alias}"
            ))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
        let paths = app_data.metadata.openapi["paths"].as_object().unwrap();

        for (reference, schema) in app_data.metadata.openapi["components"]["schemas"]
            .as_object()
            .unwrap()
            .iter()
        {
            // TODO: here, we should check if an existing schema is equal, and if not, we need to disambiguate
            // Create a new name based on what we can infer here, (maybe even use LLM to come up with a name lol)
            // Then, we need to remap the $refs in this object that reference this.
            // Actually, we need to use a multi value hashmap, because another endpoint might use the same, alternate type
            // and we want to be able to look up the existing matching type
            // Use a Hashmap<String, Vec<(String, serde_json::Value)>
            req_res_types.insert(reference.to_string(), schema.clone());
        }

        for (path, params) in paths {
            if path == "/health" {
                // skip healthcheck endpoint
                continue;
            }

            let module_parts = if path == "/" {
                vec![]
            } else {
                path.split("/")
                    .skip(1)
                    .map(|s| s.replace(".", "_").replace("-", "_"))
                    .map(|s| {
                        if s.chars().next().unwrap().is_digit(10) {
                            format!("v{s}")
                        } else {
                            s.to_string()
                        }
                    })
                    .collect::<Vec<String>>()
            };

            let parent_parts = vec![
                owner.replace(".", "_").replace("-", "_"),
                alias
                    .split("/")
                    .collect::<Vec<&str>>()
                    .join("::")
                    .replace(".", "_")
                    .replace("-", "_"),
            ]
            .into_iter()
            .map(|s| {
                if s.chars().next().unwrap().is_digit(10) {
                    format!("v{s}")
                } else {
                    s.to_string()
                }
            })
            .collect::<Vec<_>>();

            let fn_name = if module_parts.is_empty() {
                parent_parts
                    .last()
                    .unwrap_or(&"default".to_string())
                    .to_string()
            } else {
                module_parts.last().unwrap().to_string()
            };

            // Combine parent and module parts for the full path
            let mut full_path_parts = Vec::new();
            // Don't include the root module name since it's just the container
            full_path_parts.extend(parent_parts.iter().map(String::as_str));
            full_path_parts.extend(module_parts.iter().map(String::as_str));

            let Some(output_type_ref) = params["post"]["responses"]["200"]["content"]
                ["application/json"]["schema"]["$ref"]
                .as_str()
            else {
                tracing::error!("no output type ref for {}", &model.endpoint_id);
                tracing::error!("params: {:?}", params);
                continue;
            };

            let output_type = {
                let type_name = output_type_ref.split("/").last().unwrap();
                app_data.metadata.openapi["components"]["schemas"][type_name].clone()
            };

            let docs = docs_from(&model, params);

            let endpoint = format!("{owner}/{alias}{}", if path == "/" { "" } else { path });

            // Insert into the tree
            root.insert_path(
                &full_path_parts,
                &endpoint,
                &fn_name,
                params["post"].clone(),
                output_type,
                docs,
            );
        }

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }

    // Print the tree structure
    root.print_tree(0);

    let mut extra_types: HashMap<String, (String, String)> = HashMap::new();

    // Write the module tree to files
    write_module_to_files(
        &root,
        base_module_path,
        &req_res_types,
        &mut extra_types,
        true,
    )
    .expect("Failed to write module files");

    let mut manifest = Manifest::from_path("Cargo.toml").unwrap();

    // Generate and write common types
    let rust_struct_types = req_res_types
        .iter()
        .filter(|(k, _)| !hardcoded_struct(k))
        .map(|(_, v)| schema_type_to_rust_type(v, &mut extra_types, true))
        .collect::<Vec<String>>()
        .join("\n\n");

    let extra_types_str = extra_types
        .iter()
        .map(|(_, (_, item))| item.clone())
        .collect::<Vec<String>>()
        .join("\n\n");

    // Write common types to types.rs
    let types_content = format!(
        "use std::collections::HashMap;\n\nuse serde::{{Serialize, Deserialize}};\nuse crate::prelude::*;\n\n{}\n\n{}",
        rust_struct_types, extra_types_str
    );
    std::fs::write(format!("{}/types.rs", base_module_path), types_content)
        .expect("Failed to write types file");

    // Write initial root mod.rs with module declarations
    let root_mod_content = format!(
        "{}\n\nmod types;\npub use types::*;\n",
        match &root {
            Node::Module { children, .. } => children
                .iter()
                .filter_map(|child| {
                    if let Node::Module { name, .. } = child {
                        let feature_name = format!("endpoints_{}", name.replace("_", "-"));
                        Some(format!("#[cfg(any(feature = \"endpoints\", feature = \"{feature_name}\"))]\n#[cfg_attr(docsrs, doc(cfg(any(feature = \"endpoints\", feature = \"{feature_name}\"))))]\npub mod {};", name))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("\n"),
            _ => String::new(),
        }
    );
    std::fs::write(format!("{}/mod.rs", base_module_path), root_mod_content)
        .expect("Failed to write root mod.rs");

    update_manifest_features(&mut manifest, &root, "");

    let new_manifest = toml::to_string_pretty(&manifest).unwrap();
    std::fs::write("Cargo.toml", new_manifest).unwrap();
}

fn update_manifest_features(manifest: &mut Manifest, node: &Node, parent_path: &str) {
    // Remove all features that start with "endpoint"
    if let Some(features) = &mut manifest.features {
        features.retain(|name, _| !name.starts_with("endpoint"));
    }

    // Initialize features if not present
    if manifest.features.is_none() {
        manifest.features = Some(std::collections::BTreeMap::new());
    }

    // Add base endpoints feature
    if let Some(features) = &mut manifest.features {
        features.insert("endpoints".to_string(), vec![]);
    }

    // Recursively add features for each module
    add_module_features(manifest, node, parent_path);
}

fn add_module_features(manifest: &mut Manifest, node: &Node, parent_path: &str) {
    match node {
        Node::Module { name, children } => {
            let current_path = if parent_path.is_empty() {
                name.clone()
            } else {
                format!("{}_{}", parent_path, name.replace("_", "-"))
            };

            // Add feature for current module
            if let Some(features) = &mut manifest.features {
                let feature_name = current_path.clone();
                features.insert(feature_name, vec![]);
            }

            // Recurse into children
            for child in children {
                add_module_features(manifest, child, &current_path);
            }
        }
        Node::Leaf { .. } => {
            // Leaf nodes don't need features
        }
    }
}
