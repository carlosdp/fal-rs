use std::collections::{HashMap, HashSet};

use cargo_manifest::Manifest;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ModelGroup {
    key: String,
    label: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Model {
    id: String,
    title: String,
    category: String,
    #[serde(default)]
    tags: Vec<String>,
    short_description: String,
    thumbnail_url: String,
    model_url: String,
    stream_url: Option<String>,
    date: String,
    machine_type: Option<String>,
    license_type: Option<String>,
    group: Option<ModelGroup>,
    #[serde(default)]
    result_comparison: bool,
    #[serde(default)]
    highlighted: bool,
    pricing_info_override: Option<String>,
    credits_required: Option<i32>,
    endpoint_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AppDataMetadata {
    openapi: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AppData {
    app_name: String,
    metadata: AppDataMetadata,
}

enum Node {
    Module {
        name: String,
        children: Vec<Node>,
    },
    Leaf {
        endpoint: String,
        fn_name: String,
        definition: serde_json::Value,
        output_type: serde_json::Value,
        docs: String,
    },
}

impl Node {
    fn insert_path(
        &mut self,
        path_parts: &[&str],
        endpoint: &str,
        fn_name: &str,
        definition: serde_json::Value,
        output_type: serde_json::Value,
        docs: String,
    ) {
        match self {
            Node::Module { children, .. } => {
                if path_parts.is_empty() {
                    children.push(Node::Leaf {
                        endpoint: endpoint.to_string(),
                        fn_name: fn_name.to_string(),
                        definition,
                        output_type,
                        docs,
                    });
                    return;
                }

                let current = path_parts[0];
                let remaining = &path_parts[1..];

                // Find or create the child module
                if let Some(child) = children.iter_mut().find(|n| match n {
                    Node::Module { name, .. } => name == current,
                    _ => false,
                }) {
                    child.insert_path(remaining, endpoint, fn_name, definition, output_type, docs);
                } else {
                    let mut new_module = Node::Module {
                        name: current.to_string(),
                        children: Vec::new(),
                    };
                    new_module.insert_path(
                        remaining,
                        endpoint,
                        fn_name,
                        definition,
                        output_type,
                        docs,
                    );
                    children.push(new_module);
                }
            }
            Node::Leaf { .. } => panic!("Cannot insert into a leaf node"),
        }
    }

    fn print_tree(&self, indent: usize) {
        match self {
            Node::Module { name, children } => {
                tracing::trace!("{}Module: {}", " ".repeat(indent), name);
                for child in children {
                    child.print_tree(indent + 2);
                }
            }
            Node::Leaf { fn_name, .. } => {
                tracing::trace!("{}Function: {}", " ".repeat(indent), fn_name);
            }
        }
    }
}

fn write_module_to_files(
    node: &Node,
    base_path: &str,
    input_types: &HashMap<String, serde_json::Value>,
    extra_types: &mut HashMap<String, (String, String)>,
    is_root: bool,
) -> std::io::Result<()> {
    match node {
        Node::Module { name, children } => {
            let module_path = if is_root {
                base_path.to_string()
            } else {
                format!("{}/{}", base_path, name)
            };

            // Create directory if it doesn't exist
            std::fs::create_dir_all(&module_path)?;

            // Start with an empty string for mod content
            let mut mod_content = String::new();

            // Get direct child functions and modules
            let (functions, modules): (Vec<_>, Vec<_>) = children
                .iter()
                .partition(|child| matches!(child, Node::Leaf { .. }));

            // Add use declarations if we have direct functions/types
            if !functions.is_empty() {
                mod_content
                    .push_str("#[allow(unused_imports)]\nuse std::collections::HashMap;\n#[allow(unused_imports)]\nuse serde::{Serialize, Deserialize};\n#[allow(unused_imports)]\nuse crate::prelude::*;\n\n");
            }

            // Add pub mod declarations for child modules
            let mut has_child_modules = false;
            for child in &modules {
                if let Node::Module {
                    name: child_name, ..
                } = child
                {
                    has_child_modules = true;

                    let mut path_segments: Vec<String> = module_path
                        .split('/')
                        .map(|s| s.replace("_", "-"))
                        .collect();
                    path_segments.remove(0); // remove "src"
                    path_segments.push(child_name.replace("_", "-")); // add the child module name
                    let features = (1..=path_segments.len())
                        .map(|i| path_segments[0..i].join("_"))
                        .collect::<Vec<String>>();

                    let features_map = features
                        .into_iter()
                        .map(|s| format!("feature = \"{}\"", s))
                        .collect::<Vec<String>>()
                        .join(",");
                    let feature_cfg = format!(
                        "#[cfg(any({features_map}))]\n#[cfg_attr(docsrs, doc(cfg(any({features_map}))))]",
                    );

                    mod_content.push_str(&format!("{}\npub mod {};\n", feature_cfg, child_name));
                }
            }

            // Add a newline after module declarations if we have both modules and content
            if has_child_modules && !functions.is_empty() {
                mod_content.push_str("\n");
            }

            // Add direct functions and types
            if !functions.is_empty() {
                for child in functions {
                    mod_content.push_str(&generate_request_module(child, input_types, extra_types));
                }
            }

            // Write mod.rs
            std::fs::write(format!("{}/mod.rs", module_path), mod_content)?;

            // Recursively handle child modules
            for child in modules {
                if let Node::Module { .. } = child {
                    write_module_to_files(child, &module_path, input_types, extra_types, false)?;
                }
            }
        }
        Node::Leaf { .. } => {
            // Leaf nodes are handled within their parent module's mod.rs
        }
    }

    Ok(())
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

fn generate_request_module(
    node: &Node,
    input_types: &HashMap<String, serde_json::Value>,
    extra_types: &mut HashMap<String, (String, String)>,
) -> String {
    match node {
        Node::Module { name, children } => {
            format!(
                r#"
                pub mod {name} {{
                    use super::*;

                    {}
                }}
                "#,
                children
                    .iter()
                    .map(|child| generate_request_module(child, input_types, extra_types))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
        Node::Leaf {
            endpoint,
            fn_name,
            definition,
            output_type,
            docs,
        } => {
            let request_type_name = definition["requestBody"]["content"]["application/json"]
                ["schema"]["$ref"]
                .as_str()
                .unwrap()
                .split("/")
                .last()
                .unwrap();
            let request_type = input_types.get(request_type_name).unwrap();
            let request_type_name = request_type["title"].as_str().unwrap();
            let return_type = output_type["title"].as_str().unwrap();

            let output_struct = schema_type_to_rust_type(&output_type, extra_types, false);

            let docs = docs
                .trim()
                .split("\n")
                .map(|line| line.trim())
                .map(|line| format!("/// {line}"))
                .collect::<Vec<String>>()
                .join("\n");

            format!(
                r#"
                {output_struct}

                {docs}
                pub fn {fn_name}(params: {request_type_name}) -> FalRequest<{request_type_name}, {return_type}> {{
                    FalRequest::new(
                        "{endpoint}",
                        params
                    )
                }}
                "#,
            )
        }
    }
}

fn schema_type_to_rust_type(
    info: &serde_json::Value,
    extra_types: &mut HashMap<String, (String, String)>,
    input_type: bool,
) -> String {
    let type_name = info["title"].as_str().unwrap().replace(" ", "");
    let params = info["properties"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| {
            let is_required = info["required"]
                .as_array()
                .map(|a| a.contains(&serde_json::Value::String(k.to_string())))
                .unwrap_or(false);

            let prefix = if k == "type" {
                "#[serde(rename = \"type\")]\npub ty:".to_owned()
            } else if is_rust_keyword(k) {
                format!("#[serde(rename = \"{k}\")]\npub r#{k}:")
            } else {
                let prop_name = to_snake_case(&k);
                format!("pub {prop_name}:")
            };

            let serialization_attr = if !is_required {
                "#[serde(skip_serializing_if = \"Option::is_none\")]\n"
            } else {
                ""
            };

            let description = v["description"].as_str().unwrap_or("");
            let examples = v["examples"].as_array();

            let docs = if !description.is_empty() {
                let description = description
                    .trim()
                    .split("\n")
                    .map(|line| line.trim())
                    .map(|line| format!("/// {line}"))
                    .collect::<Vec<String>>()
                    .join("\n");

                let examples = if let Some(examples) = examples {
                    let examples = examples
                        .iter()
                        .map(|e| serde_json::to_string(e).unwrap())
                        .map(|line| format!("/// {line}"))
                        .collect::<Vec<String>>()
                        .join("\n");

                    format!("{description}{examples}\n")
                } else {
                    "".to_string()
                };

                format!("{description}{examples}\n")
            } else {
                "".to_string()
            };

            format!(
                "{docs}{serialization_attr}{prefix} {}",
                schema_property_to_rust_type(v, is_required, extra_types,)
            )
        })
        .collect::<Vec<String>>()
        .join(",\n");

    let default_derive = if input_type { ", Default" } else { "" };

    format!(
        r#"
    #[derive(Debug, Serialize, Deserialize{default_derive})]
    pub struct {type_name} {{
        {params}
    }}
    "#
    )
}

fn schema_property_to_rust_type(
    property: &serde_json::Value,
    required: bool,
    extra_types: &mut HashMap<String, (String, String)>,
) -> String {
    let type_name = match property["type"].as_str() {
        Some("string") => "String".to_string(),
        Some("number") => "f64".to_string(),
        Some("integer") => "i64".to_string(),
        Some("boolean") => "bool".to_string(),
        Some("object") => {
            if property["additionalProperties"].as_object().is_some() {
                if let Some(title) = property["title"].as_str() {
                    get_or_build_object_type(title, property, extra_types)
                } else {
                    tracing::warn!("[additionalProperties] no title for object: {:?}", property);
                    "HashMap<String, serde_json::Value>".to_string()
                }
            } else if let Some(reference) = property["$ref"].as_str() {
                if hardcoded_struct(reference) {
                    reference.split("/").last().unwrap().to_string()
                } else {
                    "HashMap<String, serde_json::Value>".to_string()
                }
            } else {
                "HashMap<String, serde_json::Value>".to_string()
            }
        }
        Some("array") => {
            format!(
                "Vec<{}>",
                schema_property_to_rust_type(&property["items"], required, extra_types)
            )
        }
        _ => {
            if let Some(reference) = property["$ref"].as_str() {
                reference.split("/").last().unwrap().to_string()
            } else if let Some(all_of) = property["allOf"].as_array() {
                // fal API uses these to rename types usually, we should be able to just grab the first (only) element
                if all_of.len() != 1 {
                    tracing::warn!("allOf != 1 element: {:?}", all_of);
                }

                let first_element = all_of.first().unwrap();
                schema_property_to_rust_type(first_element, required, extra_types)
            } else if let Some(one_of) = property["oneOf"].as_array() {
                if one_of.len() == 1 {
                    schema_property_to_rust_type(one_of.first().unwrap(), required, extra_types)
                } else {
                    if let Some(title) = property["title"].as_str() {
                        get_or_build_enum(title, one_of, extra_types)
                    } else {
                        tracing::warn!("no title for oneOf: {:?}", property);
                        "serde_json::Value".to_string()
                    }
                }
            } else if let Some(any_of) = property["anyOf"].as_array() {
                if any_of.len() == 1 {
                    schema_property_to_rust_type(any_of.first().unwrap(), required, extra_types)
                } else {
                    if let Some(title) = property["title"].as_str() {
                        // technically this is not correct per OpenAPI spec,
                        // but I don't think this is being used correctly in fal anyways
                        get_or_build_enum(title, any_of, extra_types)
                    } else {
                        tracing::warn!("no title for anyOf: {:?}", property);
                        "serde_json::Value".to_string()
                    }
                }
            } else {
                tracing::warn!("Unsupported type: {:?}", property);
                "serde_json::Value".to_string()
            }
        }
    };

    if required {
        format!("{}", type_name)
    } else {
        format!("Option<{}>", type_name)
    }
}

// handle oneOf
fn get_or_build_enum(
    title: &str,
    options: &[serde_json::Value],
    extra_types: &mut HashMap<String, (String, String)>,
) -> String {
    let mut enum_name = title.to_string();
    enum_name = enum_name
        .replace(".", "_")
        .replace("-", "_")
        .replace(" ", "");
    enum_name = format!("{}Property", enum_name);

    if let Some((existing_type, _)) = extra_types.get(&enum_name) {
        existing_type.clone()
    } else {
        let variants = options
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if let Some(enum_options) = v["enum"].as_array() {
                    enum_options
                        .iter()
                        .enumerate()
                        .map(|(j, op)| {
                            let mut variant_name =
                                snake_to_upper_camel(op.as_str().unwrap()).replace(":", "_");
                            if variant_name
                                .chars()
                                .next()
                                .map_or(false, |c| c.is_digit(10))
                            {
                                let prefix = v["title"]
                                    .as_str()
                                    .map(|s| snake_to_upper_camel(s))
                                    .unwrap_or("Property".to_string());
                                variant_name = format!("{prefix}_{variant_name}");
                            }
                            let original_name = op.as_str().unwrap();
                            let default_marker = if i == 0 && j == 0 { "#[default]\n" } else { "" };

                            format!("{default_marker}#[serde(rename=\"{original_name}\")]\n{variant_name}")
                        })
                        .collect()
                } else {
                    let variant_name =
                        if let Some(title) = v["title"].as_str() {
                            title.to_string()
                        } else if let Some(reference) = v["$ref"].as_str() {
                            reference.split("/").last().unwrap().to_string()
                        } else {
                            tracing::warn!("no title for oneOf: {:?}", v);
                            snake_to_upper_camel(v["type"].as_str().expect(
                                "if you don't have a title, you have to have a basic type =(",
                            ))
                        };
                    let variant_name = variant_name
                        .replace(".", "_")
                        .replace("-", "_")
                        .replace(" ", "");

                    let variant_type = schema_property_to_rust_type(v, true, extra_types);
                    let default_marker = if i == 0 { "#[default]\n" } else { "" };

                    vec![format!("{default_marker}{variant_name}({variant_type})")]
                }
            })
            .flatten()
            .collect::<Vec<String>>()
            .join(",\n");

        let enum_type = format!(
            "#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]\n#[allow(non_camel_case_types)]\npub enum {enum_name}\n{{\n{variants}\n}}"
        );

        extra_types.insert(enum_name.clone(), (enum_name.clone(), enum_type.clone()));

        enum_name
    }
}

fn get_or_build_object_type(
    title: &str,
    property: &serde_json::Value,
    extra_types: &mut HashMap<String, (String, String)>,
) -> String {
    let mut struct_name = title.to_string();
    struct_name = struct_name
        .replace(".", "_")
        .replace("-", "_")
        .replace(" ", "");

    if let Some((name, _)) = extra_types.get(&struct_name) {
        name.clone()
    } else {
        let mut property = property.clone();
        property["properties"] = property["additionalProperties"].clone();
        let struct_impl = schema_type_to_rust_type(&property, extra_types, true);
        extra_types.insert(struct_name.clone(), (struct_name.clone(), struct_impl));
        struct_name
    }
}

fn docs_from(model: &Model, openapi_params: &serde_json::Value) -> String {
    let openapi_description = openapi_params["post"]["description"].as_str().unwrap_or("");

    let title = &model.title;
    let category = &model.category;
    let machine_type = model
        .machine_type
        .as_ref()
        .map(|s| format!("Machine Type: {s}"))
        .unwrap_or("".to_string());
    let license_type = model
        .license_type
        .as_ref()
        .map(|s| format!("License Type: {s}"))
        .unwrap_or("".to_string());

    format!(
        "
    {title}

    Category: {category}
    {machine_type}
    {license_type}

    {openapi_description}
    "
    )
}

fn hardcoded_struct(reference: &str) -> bool {
    matches!(reference, "File" | "Image")
}

fn snake_to_upper_camel(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let mut prev_is_uppercase = false;

    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_is_uppercase {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_is_uppercase = true;
        } else {
            result.push(if c.is_whitespace() || c == '-' {
                '_'
            } else {
                c
            });
            prev_is_uppercase = false;
        }
    }

    result
}

fn is_rust_keyword(word: &str) -> bool {
    const KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", // Reserved for future use
        "abstract", "async", "await", "become", "box", "do", "final", "macro", "override", "priv",
        "try", "typeof", "unsized", "virtual", "yield", // Strict mode keywords
        "dyn",
    ];

    KEYWORDS.contains(&word)
}
