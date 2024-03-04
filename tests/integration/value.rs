use json_with_comments::{
    from_str,
    value::{number::NumberValue, string::StringValue, JsoncValue, MapImpl},
};

#[test]
fn test_deserialize_null_as_value() {
    let target = r#"null"#;
    let value: JsoncValue<'_, u8, f32> = from_str(target).unwrap();
    assert_eq!(value, JsoncValue::Null);
}

#[test]
fn test_deserialize_bool_as_value() {
    let target = r#"true"#;
    let value: JsoncValue<'_, u8, f32> = from_str(target).unwrap();
    assert_eq!(value, JsoncValue::Bool(true));
}

#[test]
fn test_deserialize_number_as_value() {
    let target = r#"9"#;
    let value: JsoncValue<'_, u8, f32> = from_str(target).unwrap();
    assert!(matches!(value, JsoncValue::Number(NumberValue::Integer(9))));
}

#[test]
fn test_deserialize_string_as_value() {
    let target = r#""string""#;
    let value: JsoncValue<'_, u8, f32> = from_str(target).unwrap();
    assert!(matches!(value, JsoncValue::String(StringValue::Borrowed("string"))));
}

#[test]
fn test_deserialize_array_as_value() {
    let target = r#"[null, true, "false", 10]"#;
    let value: JsoncValue<'_, u8, f32> = from_str(target).unwrap();
    assert_eq!(
        value,
        JsoncValue::Array(vec![
            JsoncValue::Null,
            JsoncValue::Bool(true),
            JsoncValue::String(StringValue::Owned("false".to_owned())),
            JsoncValue::Number(NumberValue::Integer(10)),
        ])
    );
}

#[test]
fn test_deserialize_object_as_value() {
    let target = r#"{"null": null, "bool": true, "str": "false", "number": 10}"#;
    let value: JsoncValue<'_, u8, f32> = from_str(target).unwrap();
    assert_eq!(
        value,
        JsoncValue::Object(MapImpl::from([
            (StringValue::Borrowed("null"), JsoncValue::Null),
            (StringValue::Borrowed("bool"), JsoncValue::Bool(true)),
            (StringValue::Borrowed("str"), JsoncValue::String(StringValue::Owned("false".to_owned()))),
            (StringValue::Borrowed("number"), JsoncValue::Number(NumberValue::Integer(10))),
        ]))
    );
}
