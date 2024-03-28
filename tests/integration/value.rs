use json_with_comments::{
    from_str, jsonc_generics, to_string,
    value::{number::Number, JsoncValue},
    Value,
};

#[test]
fn test_deserialize_serialize_null_as_value() {
    let target = r#"null"#;
    let value: Value = from_str(target).unwrap();
    assert_eq!(value, JsoncValue::Null);

    let null = to_string(value).unwrap();
    assert_eq!(null, target);
}

#[test]
fn test_deserialize_serialize_bool_as_value() {
    let target = r#"true"#;
    let value: Value = from_str(target).unwrap();
    assert_eq!(value, JsoncValue::Bool(true));

    let tru = to_string(value).unwrap();
    assert_eq!(tru, target);
}

#[test]
fn test_deserialize_serialize_number_as_value() {
    let target = r#"9"#;
    let value: Value = from_str(target).unwrap();
    assert_eq!(value, JsoncValue::Number(Number::Integer(9)));

    let num = to_string(value).unwrap();
    assert_eq!(num, target);
}

#[test]
fn test_deserialize_serialize_string_as_value() {
    let target = r#""string""#;
    let value: Value = from_str(target).unwrap();
    assert_eq!(value, JsoncValue::String("string".to_string()));

    let string = to_string(value).unwrap();
    assert_eq!(string, target);
}

#[test]
fn test_deserialize_serialize_array_as_value() {
    let target = r#"[null,true,"false",10]"#;
    let value: Value = from_str(target).unwrap();
    assert_eq!(value, jsonc_generics!([null, true, "false", 10]));

    let array = to_string(value).unwrap();
    assert_eq!(array, target);
}

#[test]
fn test_deserialize_serialize_object_as_value() {
    let target = r#"{"null":null,"number":10000000000,"float":1.5}"#;
    let value: JsoncValue<i64, f32> = from_str(target).unwrap();
    assert_eq!(
        value,
        jsonc_generics!({
            "null": null,
            "number": 10000000000,
            "float": 1.5
        })
    );

    let object = to_string(value).unwrap();
    assert!([
        target,
        r#"{"null":null,"float":1.5,"number":10000000000}"#,
        r#"{"number":10000000000,"null":null,"float":1.5}"#,
        r#"{"number":10000000000,"float":1.5,"null":null}"#,
        r#"{"float":1.5,"number":10000000000,"null":null}"#,
        r#"{"float":1.5,"null":null,"number":10000000000}"#,
    ]
    .contains(&&object[..]));
}
