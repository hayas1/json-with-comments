use json_with_comments::{
    from_str,
    value::{number::NumberValue, JsoncValue, MapImpl},
    Value,
};

#[test]
fn test_deserialize_null_as_value() {
    let target = r#"null"#;
    let value: Value = from_str(target).unwrap();
    assert_eq!(value, JsoncValue::Null);
}

#[test]
fn test_deserialize_bool_as_value() {
    let target = r#"true"#;
    let value: Value = from_str(target).unwrap();
    assert_eq!(value, JsoncValue::Bool(true));
}

#[test]
fn test_deserialize_number_as_value() {
    let target = r#"9"#;
    let value: Value = from_str(target).unwrap();
    assert_eq!(value, JsoncValue::Number(NumberValue::Integer(9)));
}

#[test]
fn test_deserialize_string_as_value() {
    let target = r#""string""#;
    let value: Value = from_str(target).unwrap();
    assert_eq!(value, JsoncValue::String("string".to_string()));
}

#[test]
fn test_deserialize_array_as_value() {
    let target = r#"[null, true, "false", 10]"#;
    let value: Value = from_str(target).unwrap();
    assert_eq!(
        value,
        JsoncValue::Array(vec![
            JsoncValue::Null,
            JsoncValue::Bool(true),
            JsoncValue::String("false".to_string()),
            JsoncValue::Number(NumberValue::Integer(10)),
        ])
    );
}

#[test]
fn test_deserialize_object_as_value() {
    let target = r#"{"null": null, "bool": true, "str": "false", "number": 10000000000, "float": 1.5}"#;
    let value: JsoncValue<i128, f32> = from_str(target).unwrap();
    assert_eq!(
        value,
        JsoncValue::Object(MapImpl::from([
            ("null".to_string(), JsoncValue::Null),
            ("bool".to_string(), JsoncValue::Bool(true)),
            ("str".to_string(), JsoncValue::String("false".to_string())),
            ("number".to_string(), JsoncValue::Number(NumberValue::Integer(10000000000))),
            ("float".to_string(), JsoncValue::Number(NumberValue::Float(1.5))),
        ]))
    );
}
