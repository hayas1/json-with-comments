use std::collections::HashMap;

use json_with_comment::from_str;

#[test]
fn test_deserialize_empty_object() {
    let raw = r#"{}"#;
    let data: HashMap<String, String> = from_str(raw).unwrap();
    assert_eq!(data, HashMap::new());
}

#[test]
fn test_cannot_deserialize_only_comma_object() {
    let raw = r#"{,}"#;
    let data = from_str::<HashMap<String, String>>(raw);
    assert!(matches!(data, Err(_)));
}

#[test]
fn test_deserialize_edge_array() {
    let raw = r#"[]"#;
    let data: Vec<()> = from_str(raw).unwrap();
    assert_eq!(data, vec![]);
}

#[test]
fn test_cannot_deserialize_only_comma_array() {
    let raw = r#"[,]"#;
    let data = from_str::<Vec<String>>(raw);
    assert!(matches!(data, Err(_)));
}
