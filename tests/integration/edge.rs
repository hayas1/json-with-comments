use std::collections::HashMap;

use json_with_comment::from_str;

#[test]
fn test_deserialize_empty_object() {
    let target = r#"{}"#;
    let data: HashMap<String, String> = from_str(target).unwrap();
    assert_eq!(data, HashMap::new());
}

#[test]
fn test_cannot_deserialize_only_comma_object() {
    let target = r#"{,}"#;
    let data = from_str::<HashMap<String, String>>(target);
    assert!(matches!(data, Err(_)));
}

#[test]
fn test_deserialize_edge_array() {
    let target = r#"[]"#;
    let data: Vec<()> = from_str(target).unwrap();
    assert_eq!(data, vec![]);
}

#[test]
fn test_cannot_deserialize_only_comma_array() {
    let target = r#"[,]"#;
    let data = from_str::<Vec<String>>(target);
    assert!(matches!(data, Err(_)));
}
