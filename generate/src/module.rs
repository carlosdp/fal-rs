use std::collections::HashMap;

use crate::types::schema_type_to_rust_type;

pub enum Node {
    Module {
        name: String,
        children: Vec<Node>,
    },
    Leaf {
        endpoint: String,
        fn_name: String,
        definition: serde_json::Value,
        input_types: Vec<serde_json::Value>,
        output_type: serde_json::Value,
        docs: String,
    },
}

impl Node {
    pub fn insert_path(
        &mut self,
        path_parts: &[&str],
        endpoint: &str,
        fn_name: &str,
        definition: serde_json::Value,
        input_types: Vec<serde_json::Value>,
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
                        input_types,
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
                    child.insert_path(
                        remaining,
                        endpoint,
                        fn_name,
                        definition,
                        input_types,
                        output_type,
                        docs,
                    );
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
                        input_types,
                        output_type,
                        docs,
                    );
                    children.push(new_module);
                }
            }
            Node::Leaf { .. } => panic!("Cannot insert into a leaf node"),
        }
    }

    pub fn print_tree(&self, indent: usize) {
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

pub fn write_module_to_files(
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

pub fn generate_request_module(
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
            input_types,
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
            let return_type = output_type["title"].as_str().unwrap();

            let input_structs = input_types
                .iter()
                .map(|input_type| schema_type_to_rust_type(input_type, extra_types, false))
                .collect::<Vec<String>>()
                .join("\n");
            // let output_struct = schema_type_to_rust_type(&output_type, extra_types, false);

            let docs = docs
                .trim()
                .split("\n")
                .map(|line| line.trim())
                .map(|line| format!("/// {line}"))
                .collect::<Vec<String>>()
                .join("\n");

            format!(
                r#"
                {input_structs}

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
