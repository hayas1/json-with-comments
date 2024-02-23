use json_with_comment::from_str;
use serde::Deserialize;

#[test]
fn test_deserialize_recursive_object() {
    #[derive(Deserialize, PartialEq, Eq, Debug)]
    struct Node<V> {
        value: V,
        next: Option<Box<Node<V>>>,
    }
    let raw = r#"
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
    let root: Node<String> = from_str(raw).unwrap();
    assert_eq!(root.value, "foo");

    let next = root.next.unwrap();
    assert_eq!(next.value, "bar");

    let last = next.next.unwrap();
    assert_eq!(last.value, "baz");
    assert_eq!(last.next, None);
}

#[test]
fn test_deserialize_recursive_array() {
    let raw = r#"[[[], [], []], [], []]"#;
    let data: Vec<Vec<Vec<()>>> = from_str(raw).unwrap();
    assert_eq!(data, vec![vec![vec![], vec![], vec![]], vec![], vec![]]);
}
