use json_with_comments::{from_str, jsonc, to_string, Value};

#[test]
fn test_roundtrip_preserve_order_str() {
    let target = r#"{"hoge":1,"fuga":2,"piyo":3}"#;
    let value: Value = from_str(target).unwrap();
    assert_eq!(value, jsonc!({"hoge": 1, "fuga": 2, "piyo": 3}));

    let jsonc = to_string(value).unwrap();
    assert_eq!(jsonc, target);
}

#[test]
fn test_roundtrip_preserve_order_value() {
    let target = jsonc!({
        "hoge": false,
        "fuga": 1,
        "piyo": {
            "foo": 0,
            "bar": "one",
            "baz": [[],[[]]],
        },
    });
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, r#"{"hoge":false,"fuga":1,"piyo":{"foo":0,"bar":"one","baz":[[],[[]]]}}"#);

    let value: Value = from_str(&jsonc).unwrap();
    assert_eq!(value, target);
}
