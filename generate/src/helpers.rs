use serde_json::{Map, Value};

pub fn hardcoded_struct(reference: &str) -> bool {
    matches!(reference, "File" | "Image")
}

pub fn snake_to_upper_camel(input: &str) -> String {
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

pub fn to_snake_case(input: &str) -> String {
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

pub fn is_rust_keyword(word: &str) -> bool {
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

/// Compares two serde_json::Value instances for equality, ignoring array order.
///
/// This function recursively compares JSON structures. For objects, it checks
/// if they have the same key/value pairs regardless of order (standard behavior).
/// For arrays, it treats them as multisets (order doesn't matter, but duplicates count).
///
/// # Arguments
///
/// * `a` - First JSON value to compare
/// * `b` - Second JSON value to compare
///
/// # Returns
///
/// * `bool` - True if values are semantically equal (ignoring array order)
pub fn json_eq_ignore_array_order(a: &Value, b: &Value) -> bool {
    match (a, b) {
        // For null, boolean, number, and string types, use standard equality
        (Value::Null, Value::Null) => true,
        (Value::Bool(a_val), Value::Bool(b_val)) => a_val == b_val,
        (Value::Number(a_val), Value::Number(b_val)) => a_val == b_val,
        (Value::String(a_val), Value::String(b_val)) => a_val == b_val,

        // For objects, recursively compare all key-value pairs
        (Value::Object(a_obj), Value::Object(b_obj)) => objects_equal(a_obj, b_obj),

        // For arrays, compare as multisets (ignoring order)
        (Value::Array(a_arr), Value::Array(b_arr)) => arrays_equal_unordered(a_arr, b_arr),

        // Different types are never equal
        _ => false,
    }
}

/// Helper function to compare objects recursively
fn objects_equal(a: &Map<String, Value>, b: &Map<String, Value>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for (key, a_val) in a {
        match b.get(key) {
            Some(b_val) => {
                if !json_eq_ignore_array_order(a_val, b_val) {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}

/// Helper function to compare arrays as multisets
fn arrays_equal_unordered(a: &[Value], b: &[Value]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    // We can't directly use HashSet for Values because they might not implement Hash
    // Instead, we'll use a "matching" approach

    // Create a copy of b that we'll modify
    let mut b_remaining: Vec<&Value> = b.iter().collect();

    for a_val in a {
        // Try to find a matching element in b_remaining
        let mut found_index = None;

        for (i, b_val) in b_remaining.iter().enumerate() {
            if json_eq_ignore_array_order(a_val, b_val) {
                found_index = Some(i);
                break;
            }
        }

        match found_index {
            Some(index) => {
                // Remove the matched element
                b_remaining.remove(index);
            }
            None => {
                // No match found
                return false;
            }
        }
    }

    // If we've matched everything, b_remaining should be empty
    b_remaining.is_empty()
}

// Example usage:
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_values() {
        assert!(json_eq_ignore_array_order(&json!(null), &json!(null)));
        assert!(json_eq_ignore_array_order(&json!(true), &json!(true)));
        assert!(json_eq_ignore_array_order(&json!(42), &json!(42)));
        assert!(json_eq_ignore_array_order(&json!("hello"), &json!("hello")));

        assert!(!json_eq_ignore_array_order(&json!(null), &json!(false)));
        assert!(!json_eq_ignore_array_order(&json!(42), &json!(42.0))); // Note: This behavior depends on serde_json::Number equality
        assert!(!json_eq_ignore_array_order(
            &json!("hello"),
            &json!("world")
        ));
    }

    #[test]
    fn test_objects() {
        // Objects with same key/values in different order
        assert!(json_eq_ignore_array_order(
            &json!({"name": "Alice", "age": 30}),
            &json!({"age": 30, "name": "Alice"})
        ));

        // Objects with different values
        assert!(!json_eq_ignore_array_order(
            &json!({"name": "Alice", "age": 30}),
            &json!({"name": "Bob", "age": 30})
        ));

        // Nested objects
        assert!(json_eq_ignore_array_order(
            &json!({"person": {"name": "Alice", "age": 30}}),
            &json!({"person": {"age": 30, "name": "Alice"}})
        ));
    }

    #[test]
    fn test_arrays() {
        // Arrays with same elements in different order
        assert!(json_eq_ignore_array_order(
            &json!([1, 2, 3]),
            &json!([3, 2, 1])
        ));

        // Arrays with different elements
        assert!(!json_eq_ignore_array_order(
            &json!([1, 2, 3]),
            &json!([1, 2, 4])
        ));

        // Arrays with duplicates
        assert!(json_eq_ignore_array_order(
            &json!([1, 2, 2, 3]),
            &json!([3, 2, 1, 2])
        ));

        assert!(!json_eq_ignore_array_order(
            &json!([1, 2, 2]),
            &json!([1, 2])
        ));
    }

    #[test]
    fn test_complex_structures() {
        // Objects containing arrays
        assert!(json_eq_ignore_array_order(
            &json!({"name": "Alice", "scores": [85, 90, 95]}),
            &json!({"name": "Alice", "scores": [95, 85, 90]})
        ));

        // Arrays containing objects
        assert!(json_eq_ignore_array_order(
            &json!([
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25}
            ]),
            &json!([
                {"age": 25, "name": "Bob"},
                {"age": 30, "name": "Alice"}
            ])
        ));

        // Deeply nested structures
        assert!(json_eq_ignore_array_order(
            &json!({
                "team": "Developers",
                "members": [
                    {"name": "Alice", "skills": ["Rust", "Python"]},
                    {"name": "Bob", "skills": ["JavaScript", "TypeScript"]}
                ]
            }),
            &json!({
                "members": [
                    {"skills": ["TypeScript", "JavaScript"], "name": "Bob"},
                    {"skills": ["Python", "Rust"], "name": "Alice"}
                ],
                "team": "Developers"
            })
        ));
    }
}
