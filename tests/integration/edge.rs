use std::collections::{BTreeMap, BTreeSet, HashMap};

use json_with_comments::{error::SyntaxError, from_str, to_string};

#[test]
fn test_cannot_deserialize_empty() {
    // empty text is not valid JSON https://www.json.org/json-en.html
    let target = "";
    let err = from_str::<Option<HashMap<(), ()>>>(target).unwrap_err();
    assert!(matches!(err.into_inner().downcast_ref().unwrap(), SyntaxError::EofWhileStartParsingValue));
}

#[test]
fn test_deserialize_multiple_jsonc() {
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
fn test_serialize_empty_object() {
    let hash = HashMap::<(), ()>::new();
    let object = to_string(hash).unwrap();
    assert_eq!(object, "{}");

    let btree = BTreeMap::<(), ()>::new();
    let object = to_string(btree).unwrap();
    assert_eq!(object, "{}");
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
fn test_serialize_empty_array() {
    let vector = Vec::<()>::new();
    let array = to_string(vector).unwrap();
    assert_eq!(array, "[]");

    let set = BTreeSet::<()>::new();
    let array = to_string(set).unwrap();
    assert_eq!(array, "[]");
}

#[test]
fn test_cannot_deserialize_only_comma_array() {
    let target = "[,]";
    let data = from_str::<Vec<String>>(target);
    assert!(matches!(data, Err(_)));
}

#[test]
fn test_deserialize_recursive_object() {
    #[derive(serde::Deserialize, PartialEq, Eq, Debug)]
    struct Node<V> {
        value: V,
        next: Option<Box<Node<V>>>,
    }
    let target = r#"
        {
            "value": "foo",
            "next": {
                "value": "bar",
                "next": {
                    "value": "baz",
                    "next": null
                }
            }
        }
    "#;
    let root: Node<String> = from_str(target).unwrap();
    assert_eq!(root.value, "foo");

    let next = root.next.unwrap();
    assert_eq!(next.value, "bar");

    let last = next.next.unwrap();
    assert_eq!(last.value, "baz");
    assert_eq!(last.next, None);
}

#[test]
fn test_deserialize_recursive_array() {
    let target = r#"[[],[[]],[[],[[]]]]"#;
    let data: Vec<Vec<Vec<Vec<()>>>> = from_str(target).unwrap();
    assert_eq!(data, vec![vec![], vec![vec![]], vec![vec![], vec![vec![]]]]);
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
fn test_serialize_single_null_literal() {
    let unit = ();
    let null = to_string(unit).unwrap();
    assert_eq!(null, "null");

    let none = None::<u64>;
    let null = to_string(none).unwrap();
    assert_eq!(null, "null");

    let some_unit = Some(());
    let null = to_string(some_unit).unwrap();
    assert_eq!(null, "null");
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
fn test_serialize_single_bool_literal() {
    let bool = true;
    let tru = to_string(bool).unwrap();
    assert_eq!(tru, "true");

    let bool = false;
    let fal = to_string(bool).unwrap();
    assert_eq!(fal, "false");
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
fn test_serialize_single_string_literal() {
    let empty = "";
    let string = to_string(empty).unwrap();
    assert_eq!(string, "\"\"");

    let hello = "hello";
    let string = to_string(hello).unwrap();
    assert_eq!(string, "\"hello\"");
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
fn test_serialize_single_number_literal() {
    let zero = 0;
    let number = to_string(zero).unwrap();
    assert_eq!(number, "0");

    let hundred = 100u32;
    let number = to_string(hundred).unwrap();
    assert_eq!(number, "100");

    let minus_hundred = -100;
    let number = to_string(minus_hundred).unwrap();
    assert_eq!(number, "-100");

    // TODO
    // let hundred_fraction = 100.0;
    // let number = to_string(hundred_fraction).unwrap();
    // assert_eq!(number, "100.0");

    let half = 0.5;
    let number = to_string(half).unwrap();
    assert_eq!(number, "0.5");
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
fn test_cannot_deserialize_string_seq_map_key() {
    let target_seq = r#"{
        "[1, "two", 3]": "123"
    }"#;
    let seq_map: HashMap<(u32, &str, u32), &str> = from_str(target_seq).unwrap();
    assert_eq!(seq_map, HashMap::from([((1, "two", 3), "123")]));
}
