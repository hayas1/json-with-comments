use std::collections::BTreeMap;

use json_with_comments::{
    from_str, jsonc_generics, to_string,
    value::{number::Number, JsoncValue},
    Value,
};
use serde::{Deserialize, Serialize};

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

#[test]
fn test_roundtrip_unit_and_value() {
    let target = ();
    let value = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
    assert_eq!(value, jsonc_generics!(null));
    let unit: () = value.into_deserialize().unwrap();
    assert_eq!(unit, target);
}

#[test]
fn test_roundtrip_bool_and_value() {
    let target = true;
    let value = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
    assert_eq!(value, jsonc_generics!(true));
    let tru: bool = value.into_deserialize().unwrap();
    assert_eq!(tru, target);

    let target = false;
    let value = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
    assert_eq!(value, jsonc_generics!(false));
    let fal: bool = value.into_deserialize().unwrap();
    assert_eq!(fal, target);
}

#[test]
fn test_roundtrip_number_and_value() {
    let target_integer = 123usize;
    let value = JsoncValue::<i64, f64>::from_serialize(target_integer).unwrap();
    assert_eq!(value, jsonc_generics!(123));
    let num: usize = value.into_deserialize().unwrap();
    assert_eq!(num, target_integer);

    let target_float = 123.45f64;
    let value = JsoncValue::<i64, f64>::from_serialize(target_float).unwrap();
    assert_eq!(value, jsonc_generics!(123.45));
    let num: f64 = value.into_deserialize().unwrap();
    assert_eq!(num, target_float);
}

#[test]
fn test_roundtrip_string_and_value() {
    let target = "string";
    let value = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
    assert_eq!(value, jsonc_generics!("string"));
    let string: String = value.into_deserialize().unwrap();
    assert_eq!(string, target);
}

#[test]
fn test_roundtrip_seq_and_value() {
    let target_vec = [0, 1, 2];
    let value = JsoncValue::<i64, f64>::from_serialize(target_vec).unwrap();
    assert_eq!(value, jsonc_generics!([0, 1, 2]));
    let array: [i32; 3] = value.into_deserialize().unwrap();
    assert_eq!(array, target_vec);

    let target_tuple = (false, 1, "two");
    let value = JsoncValue::<i64, f64>::from_serialize(target_tuple).unwrap();
    assert_eq!(value, jsonc_generics!([false, 1, "two"]));
    let tuple: (bool, i32, &str) = value.into_deserialize().unwrap();
    assert_eq!(tuple, target_tuple);
}

#[test]
fn test_roundtrip_map_and_value() {
    let target = BTreeMap::from([("0".to_string(), true), ("1".to_string(), false)]);
    let value = JsoncValue::<i64, f64>::from_serialize(&target).unwrap();
    assert_eq!(value, jsonc_generics!({ "0": true, "1": false }));
    let object: BTreeMap<String, bool> = value.into_deserialize().unwrap();
    assert_eq!(object, target);
}

#[test]
fn test_roundtrip_option_and_value() {
    let target = Some(true);
    let value = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
    assert_eq!(value, jsonc_generics!(true));
    let opt: Option<bool> = value.into_deserialize().unwrap();
    assert_eq!(opt, target);

    let target = None;
    let value = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
    assert_eq!(value, jsonc_generics!(null));
    let opt: Option<bool> = value.into_deserialize().unwrap();
    assert_eq!(opt, target);
}

#[test]
fn test_roundtrip_struct_and_value() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Unit;
    let target = Unit;
    let value = JsoncValue::<i64, f64>::from_serialize(&target).unwrap();
    assert_eq!(value, jsonc_generics!(null));
    let unit: Unit = value.into_deserialize().unwrap();
    assert_eq!(unit, target);

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Point(i32, i32);
    let target = Point(0, 1);
    let value = JsoncValue::<i64, f64>::from_serialize(&target).unwrap();
    assert_eq!(value, jsonc_generics!([0, 1]));
    let point: Point = value.into_deserialize().unwrap();
    assert_eq!(point, target);

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u8,
    }
    let target = Person { name: "John".to_string(), age: 21 };
    let value = JsoncValue::<i64, f64>::from_serialize(&target).unwrap();
    assert_eq!(
        value,
        jsonc_generics!({
            "name": "John",
            "age": 21
        })
    );
    let object: Person = value.into_deserialize().unwrap();
    assert_eq!(object, target);
}

#[test]
fn test_roundtrip_enum_and_value() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Metasyntactic {
        Foo,
        Bar(i32),
        Baz(i32, i32),
        Qux { hoge: i32, fuga: i32 },
    }
    let target = Metasyntactic::Foo;
    let value = JsoncValue::<i64, f64>::from_serialize(&target).unwrap();
    assert_eq!(value, jsonc_generics!("Foo"));
    let metasyntactic: Metasyntactic = value.into_deserialize().unwrap();
    assert_eq!(metasyntactic, target);

    let target = Metasyntactic::Bar(10);
    let value = JsoncValue::<i64, f64>::from_serialize(&target).unwrap();
    assert_eq!(value, jsonc_generics!({"Bar": 10}));
    let metasyntactic: Metasyntactic = value.into_deserialize().unwrap();
    assert_eq!(metasyntactic, target);

    let target = Metasyntactic::Baz(10, 20);
    let value = JsoncValue::<i64, f64>::from_serialize(&target).unwrap();
    assert_eq!(value, jsonc_generics!({"Baz": [10, 20]}));
    let metasyntactic: Metasyntactic = value.into_deserialize().unwrap();
    assert_eq!(metasyntactic, target);

    let target = Metasyntactic::Qux { hoge: 10, fuga: 20 };
    let value = JsoncValue::<i64, f64>::from_serialize(&target).unwrap();
    assert_eq!(
        value,
        jsonc_generics!({
            "Qux": {
                "hoge": 10,
                "fuga": 20,
            }
        })
    );
    let metasyntactic: Metasyntactic = value.into_deserialize().unwrap();
    assert_eq!(metasyntactic, target);
}
