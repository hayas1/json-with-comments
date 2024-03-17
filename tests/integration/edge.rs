use std::collections::HashMap;

use json_with_comments::{error::SyntaxError, from_str};

#[test]
fn test_cannot_deserialize_empty() {
    // empty text is not valid JSON https://www.json.org/json-en.html
    let target = "";
    let err = from_str::<Option<HashMap<(), ()>>>(target).unwrap_err();
    assert!(matches!(err.into_inner().downcast_ref().unwrap(), SyntaxError::EofWhileStartParsingValue));
}

#[test]
fn test_multiple_jsonc() {
    let target = r#"
        {
            "hoge": "fuga"
        }
        {
            "foo": "bar"
        }
    "#;
    let err = from_str::<HashMap<String, String>>(target).unwrap_err();
    assert!(matches!(err.into_inner().downcast_ref().unwrap(), SyntaxError::ExpectedEof { .. }));
}

#[test]
fn test_deserialize_empty_object() {
    let target = "{}";
    let data: HashMap<(), ()> = from_str(target).unwrap();
    assert_eq!(data, HashMap::new());

    let target2 = "{  }";
    let data: HashMap<(), ()> = from_str(target2).unwrap();
    assert_eq!(data, HashMap::new());
}

#[test]
fn test_cannot_deserialize_only_comma_object() {
    let target = "{,}";
    let data = from_str::<HashMap<String, String>>(target);
    assert!(matches!(data, Err(_)));
}

#[test]
fn test_deserialize_empty_array() {
    let target = "[]";
    let data: Vec<()> = from_str(target).unwrap();
    assert_eq!(data, vec![]);

    let target2 = "[  ]";
    let data: Vec<()> = from_str(target2).unwrap();
    assert_eq!(data, vec![]);
}

#[test]
fn test_cannot_deserialize_only_comma_array() {
    let target = "[,]";
    let data = from_str::<Vec<String>>(target);
    assert!(matches!(data, Err(_)));
}

#[test]
fn test_deserialize_single_string_literal() {
    let target = "\"\"";
    let data: String = from_str(target).unwrap();
    assert_eq!(data, "");

    let target_hello = "\"hello\"";
    let data: &str = from_str(target_hello).unwrap();
    assert_eq!(data, "hello");
}

#[test]
fn test_deserialize_single_number_literal() {
    let target0 = "0";
    let data: u32 = from_str(target0).unwrap();
    assert_eq!(data, 0);

    let target100 = "100";
    let data: u32 = from_str(target100).unwrap();
    assert_eq!(data, 100);

    let target_100 = "-100";
    let data: i32 = from_str(target_100).unwrap();
    assert_eq!(data, -100);

    let target100_0 = "100.0";
    let data: f64 = from_str(target100_0).unwrap();
    assert_eq!(data, 100.0);
}

#[test]
fn test_deserialize_single_bool_literal() {
    let target_true = "true";
    let data: bool = from_str(target_true).unwrap();
    assert_eq!(data, true);

    let target_false = "false";
    let data: bool = from_str(target_false).unwrap();
    assert_eq!(data, false);
}

#[test]
fn test_deserialize_single_null_literal() {
    let target_null = "null";
    let data: () = from_str(target_null).unwrap();
    assert_eq!(data, ());

    let target_null2 = "null";
    let data: Option<u64> = from_str(target_null2).unwrap();
    assert_eq!(data, None);

    let target_null3 = "null";
    let data: Option<()> = from_str(target_null3).unwrap();
    // assert_eq!(data, Some(()));
    assert_eq!(data, None);
}

#[test]
fn test_cannot_deserialize_empty_comment() {
    // empty text is not valid JSON https://www.json.org/json-en.html
    let target_slash = "//";
    let err = from_str::<Option<HashMap<(), ()>>>(target_slash).unwrap_err();
    assert!(matches!(err.into_inner().downcast_ref().unwrap(), SyntaxError::EofWhileStartParsingValue));

    let target_asterisk = "/* this JSON will be empty */";
    let err = from_str::<Option<HashMap<(), ()>>>(target_asterisk).unwrap_err();
    assert!(matches!(err.into_inner().downcast_ref().unwrap(), SyntaxError::EofWhileStartParsingValue));
}

#[test]
fn test_deserialize_comment_inside_string() {
    let target_slash = "\"// this is not comment, but string\"";
    let data: String = from_str(target_slash).unwrap();
    assert_eq!(data, "// this is not comment, but string");

    let target_asterisk = "\"/* this is not comment, but string */\"";
    let data: String = from_str(target_asterisk).unwrap();
    assert_eq!(data, "/* this is not comment, but string */");
}

#[test]
fn test_deserialize_comment_with_eof() {
    let target_slash_empty = "1 //";
    let data: i32 = from_str(target_slash_empty).unwrap();
    assert_eq!(data, 1);

    let target_slash = "1 // one";
    let data: i32 = from_str(target_slash).unwrap();
    assert_eq!(data, 1);

    let target_asterisk = "1 /* one */";
    let data: i32 = from_str(target_asterisk).unwrap();
    assert_eq!(data, 1);
}

#[test]
fn test_cannot_deserialize_eof_while_comment() {
    let target_empty = "12 /* comment not terminate";
    let err = from_str::<i32>(target_empty).unwrap_err();
    assert!(matches!(err.into_inner().downcast_ref().unwrap(), SyntaxError::UnterminatedComment));

    let target_asterisk = "12 /* comment not terminate *";
    let err = from_str::<i32>(target_asterisk).unwrap_err();
    assert!(matches!(err.into_inner().downcast_ref().unwrap(), SyntaxError::UnterminatedComment));
}

#[test]
fn test_deserialize_special_comment() {
    let target_too_many_slash = "\"too many slash\"/////////////////////////////////////////////////////";
    let data: &str = from_str(target_too_many_slash).unwrap();
    assert_eq!(data, "too many slash");

    let target_too_many_asterisk = "\"too many asterisk\"/*****************************************************/";
    let data: &str = from_str(target_too_many_asterisk).unwrap();
    assert_eq!(data, "too many asterisk");

    let target_contain_control_character = "\"control character\" // terminator \u{009C} null \u{0000} escape \x1b";
    let data: &str = from_str(target_contain_control_character).unwrap();
    assert_eq!(data, "control character");
}

#[test]
#[should_panic]
fn test_cannot_deserialize_optional_string_map_key() {
    let target_option = r#"{
        ""ok"": null
    }"#;
    let map: HashMap<Option<&str>, Option<i32>> = from_str(target_option).unwrap();
    assert_eq!(map, HashMap::from([(Some("ok"), None)]));
}

#[test]
#[should_panic]
fn test_cannot_deserialize_string_seq_map_key() {
    let target_seq = r#"{
        "[1, "two", 3]": "123"
    }"#;
    let seq_map: HashMap<(u32, &str, u32), &str> = from_str(target_seq).unwrap();
    assert_eq!(seq_map, HashMap::from([((1, "two", 3), "123")]));
}
