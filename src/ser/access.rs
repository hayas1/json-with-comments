pub mod r#enum;
pub mod jsonc;
pub mod map;
pub mod number;
pub mod seq;

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use serde::Serialize;

    use crate::ser::to_str;

    #[test]
    fn test_serialize_literal() {
        assert_eq!(to_str(()).unwrap(), "null");
        assert_eq!(to_str(true).unwrap(), "true");
        assert_eq!(to_str(false).unwrap(), "false");
        assert_eq!(to_str(123).unwrap(), "123");
        assert_eq!(to_str(123.45).unwrap(), "123.45");
        assert_eq!(to_str(6.02214076E23).unwrap(), "602214076000000000000000"); // TODO

        assert_eq!(to_str("string").unwrap(), r#""string""#);
        assert_eq!(to_str("linefeed\n").unwrap(), r#""linefeed\n""#);
        assert_eq!(to_str("linefeed\u{000A}").unwrap(), r#""linefeed\n""#);
        assert_eq!(to_str("null\u{0000}").unwrap(), r#""null\u0000""#);
        assert_eq!(to_str("del\u{007f}").unwrap(), r#""del\u007F""#);
    }

    #[test]
    fn test_serialize_seq() {
        assert_eq!(to_str(vec![1, 2, 3]).unwrap(), "[1,2,3]");
        assert_eq!(to_str(vec!["str", "string"]).unwrap(), r#"["str","string"]"#);
        assert_eq!(to_str(vec![vec![], vec![false], vec![true, false]]).unwrap(), "[[],[false],[true,false]]");

        assert_eq!(to_str(((), true, 2)).unwrap(), "[null,true,2]");
        assert_eq!(to_str(((), true, ((), [()]))).unwrap(), "[null,true,[null,[null]]]");
        assert_eq!(to_str((false, 1, "two")).unwrap(), r#"[false,1,"two"]"#);

        #[derive(Serialize)]
        struct Lattice(usize, usize);
        assert_eq!(to_str(Lattice(1, 2)).unwrap(), "[1,2]");
        assert_eq!(to_str(vec![Lattice(1, 2), Lattice(3, 4)]).unwrap(), "[[1,2],[3,4]]");
    }

    #[test]
    fn test_serialize_map() {
        assert_eq!(to_str(HashMap::<(), ()>::new()).unwrap(), "{}");
        assert_eq!(to_str(HashMap::from([("key", "value")])).unwrap(), r#"{"key":"value"}"#);
        assert!([
            r#"{"one":1,"two":2,"three":3}"#,
            r#"{"one":1,"three":3,"two":2}"#,
            r#"{"two":2,"one":1,"three":3}"#,
            r#"{"two":2,"three":3,"one":1}"#,
            r#"{"three":3,"one":1,"two":2}"#,
            r#"{"three":3,"two":2,"one":1}"#,
        ]
        .contains(&&to_str(HashMap::from([("one", 1), ("two", 2), ("three", 3)])).unwrap()[..]));
        assert_eq!(
            to_str(BTreeMap::from([(
                "map1",
                BTreeMap::from([("map2", BTreeMap::from([("map3", BTreeMap::from([("nest", 3)]))]))])
            )]))
            .unwrap(),
            r#"{"map1":{"map2":{"map3":{"nest":3}}}}"#
        );

        #[derive(Serialize)]
        struct Linked {
            data: i32,
            next: Option<Box<Linked>>,
        }
        assert_eq!(
            to_str(Linked {
                data: 1,
                next: Some(Box::new(Linked { data: 2, next: Some(Box::new(Linked { data: 3, next: None })) }))
            })
            .unwrap(),
            r#"{"data":1,"next":{"data":2,"next":{"data":3,"next":null}}}"#
        );
    }

    #[test]
    fn test_serialize_enum() {
        #[derive(Serialize)]
        enum Lattice {
            D2(usize, usize),
            D3(usize, usize, usize),
        }
        assert_eq!(to_str(Lattice::D2(3, 5)).unwrap(), r#"{"D2":[3,5]}"#);
        assert_eq!(to_str(Lattice::D3(3, 5, 7)).unwrap(), r#"{"D3":[3,5,7]}"#);

        #[derive(Serialize)]
        enum Enum {
            Unit,
            Tuple(usize, isize, String),
            Struct { num: u64, text: String, bool: bool },
        }
        assert_eq!(to_str(Enum::Unit).unwrap(), r#""Unit""#);
        assert_eq!(to_str(Enum::Tuple(1, -2, "three".to_string())).unwrap(), r#"{"Tuple":[1,-2,"three"]}"#);
        assert_eq!(
            to_str(Enum::Struct { num: 1, text: "two".to_string(), bool: true }).unwrap(),
            r#"{"Struct":{"num":1,"text":"two","bool":true}}"#
        );
    }
}
