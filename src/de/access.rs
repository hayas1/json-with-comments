pub mod r#enum;
pub mod jsonc;
pub mod map;
pub mod number;
pub mod seq;
pub mod string;

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use crate::from_str;

    #[test]
    fn test_deserialize_literal() {
        assert_eq!(from_str::<bool>("true").unwrap(), true);
        assert_eq!(from_str::<bool>("false").unwrap(), false);
        assert_eq!(from_str::<()>("null").unwrap(), ());
    }

    #[test]
    fn test_deserialize_string() {
        assert_eq!(from_str::<String>(r#""hello world""#).unwrap(), "hello world".to_string());
        assert_eq!(from_str::<&str>(r#""12345""#).unwrap(), "12345");
        assert_eq!(from_str::<String>(r#""🥒💯""#).unwrap(), "🥒💯".to_string());

        assert_eq!(from_str::<String>(r#""linefeed\n""#).unwrap(), "linefeed\n");
        assert_eq!(from_str::<String>(r#""tab\tspace""#).unwrap(), "tab\tspace");
        assert_eq!(from_str::<String>(r#""linefeed\u000A""#).unwrap(), "linefeed\n");
        assert_eq!(from_str::<String>(r#""null\u0000""#).unwrap(), "null\u{0000}");
        assert_eq!(from_str::<String>(r#""del\u007f""#).unwrap(), "del\u{007F}");
    }

    #[test]
    fn test_deserialize_number() {
        assert_eq!(from_str::<u64>("57").unwrap(), 57);
        assert_eq!(from_str::<i128>("-99999999999999999").unwrap(), -99999999999999999);
        assert_eq!(from_str::<f32>("3.1415926535").unwrap(), 3.1415926535);
        assert_eq!(from_str::<f64>("6.02214076e23").unwrap(), 6.02214076E23);
    }

    #[test]
    fn test_deserialize_seq() {
        assert_eq!(from_str::<Vec<()>>("[]").unwrap(), vec![]);
        assert_eq!(from_str::<Vec<i32>>("[1,2,3]").unwrap(), vec![1, 2, 3]);
        assert_eq!(
            from_str::<((), bool, String)>(r#"[null, true, "string"]"#).unwrap(),
            ((), true, "string".to_string())
        );
        assert_eq!(from_str::<((), Vec<bool>)>(r#"[null, [false, true]]"#).unwrap(), ((), vec![false, true]));
    }

    #[test]
    fn test_deserialize_map() {
        assert_eq!(from_str::<HashMap<(), ()>>("{}").unwrap(), HashMap::new());
        assert_eq!(
            from_str::<HashMap<String, String>>(r#"{"key":"value"}"#).unwrap(),
            HashMap::from([("key".to_string(), "value".to_string())])
        );
        assert_eq!(
            from_str::<BTreeMap<i64, &str>>(r#"{"1": "one", "2": "two", "3": "three"}"#).unwrap(),
            BTreeMap::from([(1, "one"), (2, "two"), (3, "three")])
        );
        assert_eq!(
            from_str::<BTreeMap<&str, HashMap<&str, &str>>>(r#"{"hoge":{"fuga":"piyo"},"foo":{"bar":"baz"}}"#).unwrap(),
            BTreeMap::from([("hoge", HashMap::from([("fuga", "piyo")])), ("foo", HashMap::from([("bar", "baz")]))])
        )
    }

    #[test]
    fn test_deserialize_struct() {
        #[derive(serde::Deserialize)]
        struct Person<'a> {
            name: &'a str,
            age: Option<u32>,
            family: Family<'a>,
        }
        #[derive(serde::Deserialize)]
        enum Family<'a> {
            Single,
            Parent(&'a str),
            Children { brother: &'a str, sister: &'a str },
        }

        assert!(matches!(
            from_str(r#"{"name":"John","age":30,"family":"Single"}"#),
            Ok(Person { name: "John", age: Some(30), family: Family::Single })
        ));
        assert!(matches!(
            from_str(r#"{"name":"Jin","age":null,"family":{"Parent": "Jane"}}"#),
            Ok(Person { name: "Jin", age: None, family: Family::Parent("Jane") })
        ));
        assert!(matches!(
            from_str(r#"{"name":"John","age":55,"family":{"Children": {"brother": "Jim", "sister": "Kate"}}}"#),
            Ok(Person { name: "John", age: Some(55), family: Family::Children { brother: "Jim", sister: "Kate" } })
        ));
    }
}
