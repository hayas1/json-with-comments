use json_with_comment::{self, from_str};
use serde::Deserialize;

#[test]
fn test_deserialize_basic_object() {
    #[derive(Deserialize)]
    struct Data {
        schema: String,
        phantom: (),
        trailing_comma: bool,
    }
    let raw = r#"
        {
            "schema": "jsonc",
            "phantom": null,
            "trailing_comma": true,
        }
    "#;

    let data: Data = from_str(raw).unwrap();
    assert_eq!(data.schema, "jsonc");
    assert_eq!(data.phantom, ());
    assert_eq!(data.trailing_comma, true);
}

#[test]
fn test_deserialize_basic_array() {
    let raw = r#"["foo", "bar", "baz"]"#;
    let data: Vec<String> = from_str(raw).unwrap();
    assert_eq!(data, ["foo", "bar", "baz"]);
}
