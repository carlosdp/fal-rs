use std::collections::HashMap;

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
    ) {
        match self {
            Node::Module { children, .. } => {
                if path_parts.is_empty() {
                    children.push(Node::Leaf {
                        endpoint: endpoint.to_string(),
                        fn_name: fn_name.to_string(),
                        definition,
                        output_type,
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
                    child.insert_path(remaining, endpoint, fn_name, definition, output_type);
                } else {
                    let mut new_module = Node::Module {
                        name: current.to_string(),
                        children: Vec::new(),
                    };
                    new_module.insert_path(remaining, endpoint, fn_name, definition, output_type);
                    children.push(new_module);
                }
            }
            Node::Leaf { .. } => panic!("Cannot insert into a leaf node"),
        }
    }

    fn print_tree(&self, indent: usize) {
        match self {
            Node::Module { name, children } => {
                println!("{}Module: {}", " ".repeat(indent), name);
                for child in children {
                    child.print_tree(indent + 2);
                }
            }
            Node::Leaf { fn_name, .. } => {
                println!("{}Function: {}", " ".repeat(indent), fn_name);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut root = Node::Module {
        name: "root".to_string(),
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

    for model in models.iter().take(5) {
        let (owner, endpoint) = model
            .endpoint_id
            .split_once("/")
            .expect(&format!("could not split endpoint: {}", &model.endpoint_id));

        // URL encode the alias to handle special characters in the endpoint
        let parts = endpoint.split("/").collect::<Vec<&str>>();
        let alias = parts[0];

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

        for (path, params) in paths {
            if path == "/health" {
                // skip healthcheck endpoint
                continue;
            }

            let module_parts = if path == "/" {
                vec!["default".to_string()]
            } else {
                path.split("/")
                    .skip(1)
                    .map(|s| s.replace(".", "_").replace("-", "_"))
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
            ];

            let fn_name = module_parts
                .last()
                .unwrap_or(&"default".to_string())
                .to_string();

            // Combine parent and module parts for the full path
            let mut full_path_parts = Vec::new();
            full_path_parts.extend(parent_parts.iter().map(String::as_str));
            full_path_parts.extend(module_parts.iter().map(String::as_str));

            let input_type = params["post"]["requestBody"]["content"]["application/json"]["schema"]
                ["$ref"]
                .as_str()
                .unwrap();
            let output_type_ref = params["post"]["responses"]["200"]["content"]["application/json"]
                ["schema"]["$ref"]
                .as_str()
                .unwrap();

            if !req_res_types.contains_key(input_type) {
                let type_name = input_type.split("/").last().unwrap();
                req_res_types.insert(
                    input_type.to_string(),
                    app_data.metadata.openapi["components"]["schemas"][type_name].clone(),
                );
            }

            let output_type = {
                let type_name = output_type_ref.split("/").last().unwrap();
                app_data.metadata.openapi["components"]["schemas"][type_name].clone()
            };

            // Insert into the tree
            root.insert_path(
                &full_path_parts,
                &model.endpoint_id,
                &fn_name,
                params["post"].clone(),
                output_type,
            );
        }

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    // Print the tree structure
    root.print_tree(0);

    let request_module = generate_request_module(&root, &req_res_types);
    println!("\nGenerated request module:\n{}", request_module);

    let rust_struct_types = req_res_types
        .iter()
        .map(|(_, v)| schema_type_to_rust_type(v))
        .collect::<Vec<String>>()
        .join("\n\n");
    println!("\nGenerated types:\n{}", rust_struct_types);
}

fn generate_request_module(
    node: &Node,
    input_types: &HashMap<String, serde_json::Value>,
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
                    .map(|child| generate_request_module(child, input_types))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
        Node::Leaf {
            endpoint,
            fn_name,
            definition,
            output_type,
        } => {
            let request_type = input_types
                .get(
                    definition["requestBody"]["content"]["application/json"]["schema"]["$ref"]
                        .as_str()
                        .unwrap(),
                )
                .unwrap();
            let request_type_name = request_type["title"].as_str().unwrap();
            let return_type = output_type["title"].as_str().unwrap();

            let output_struct = schema_type_to_rust_type(&output_type);

            format!(
                r#"
                use super::*;

                {output_struct}

                pub async fn {fn_name}(params: {request_type_name}) -> crate::request::FalRequest<{request_type_name}, {return_type}> {{
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

fn schema_type_to_rust_type(info: &serde_json::Value) -> String {
    let type_name = info["title"].as_str().unwrap();
    let params = info["properties"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| {
            format!(
                "pub {k}: {}",
                schema_property_to_rust_type(
                    v,
                    info["required"]
                        .as_array()
                        .map(|a| a.contains(&serde_json::Value::String(k.to_string())))
                        .unwrap_or(false)
                )
            )
        })
        .collect::<Vec<String>>()
        .join(",\n");

    format!(
        r#"
    pub struct {type_name} {{
        {params}
    }}
    "#
    )
}

fn schema_property_to_rust_type(property: &serde_json::Value, required: bool) -> String {
    let type_name = match property["type"].as_str() {
        Some("string") => "String".to_string(),
        Some("number") => "f64".to_string(),
        Some("integer") => "i64".to_string(),
        Some("boolean") => "bool".to_string(),
        Some("object") => {
            if property["additionalProperties"].as_object().is_some() {
                property["title"].as_str().unwrap().to_string()
            } else if let Some(reference) = property["$ref"].as_str() {
                reference.split("/").last().unwrap().to_string()
            } else {
                "HashMap<String, serde_json::Value>".to_string()
            }
        }
        _ => {
            println!("Unsupported type: {:?}", property);
            "serde_json::Value".to_string()
        }
    };

    if required {
        format!("{}", type_name)
    } else {
        format!("Option<{}>", type_name)
    }
}
