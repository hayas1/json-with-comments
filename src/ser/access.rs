pub mod r#enum;
pub mod jsonc;
pub mod map;
pub mod number;
pub mod seq;

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use serde::Serialize;

    use crate::ser::{to_string, to_string_pretty};

    #[test]
    fn test_serialize_literal() {
        assert_eq!(to_string(()).unwrap(), "null");
        assert_eq!(to_string(false).unwrap(), "false");
        assert_eq!(to_string(true).unwrap(), "true");
    }

    #[test]
    fn test_serialize_string() {
        assert_eq!(to_string("str").unwrap(), r#""str""#);
        assert_eq!(to_string("string".to_string()).unwrap(), r#""string""#);

        assert_eq!(to_string("linefeed\n").unwrap(), r#""linefeed\n""#);
        assert_eq!(to_string("linefeed\u{000A}").unwrap(), r#""linefeed\n""#);
        assert_eq!(to_string("null\u{0000}").unwrap(), r#""null\u0000""#);
        assert_eq!(to_string("del\u{007f}").unwrap(), r#""del\u007F""#);
    }

    #[test]
    fn test_serialize_number() {
        assert_eq!(to_string(123).unwrap(), "123");
        assert_eq!(to_string(123.45).unwrap(), "123.45");
        assert_eq!(to_string(-119).unwrap(), "-119");
        assert_eq!(to_string(100.0).unwrap(), "100.0");
        assert_eq!(to_string(6.02214076E23).unwrap(), "6.02214076e23");
        assert_eq!(to_string(0.0000000000000001).unwrap(), "1e-16");
    }

    #[test]
    fn test_serialize_seq() {
        assert_eq!(to_string(vec![1, 2, 3]).unwrap(), "[1,2,3]");
        assert_eq!(to_string(vec!["str", "string"]).unwrap(), r#"["str","string"]"#);
        assert_eq!(to_string(vec![vec![], vec![false], vec![true, false]]).unwrap(), "[[],[false],[true,false]]");

        assert_eq!(to_string(((), true, 2)).unwrap(), "[null,true,2]");
        assert_eq!(to_string(((), true, ((), [()]))).unwrap(), "[null,true,[null,[null]]]");
        assert_eq!(to_string((false, 1, "two")).unwrap(), r#"[false,1,"two"]"#);
    }

    #[test]
    fn test_serialize_map() {
        assert_eq!(to_string(HashMap::<(), ()>::new()).unwrap(), "{}");
        assert_eq!(to_string(HashMap::from([("key", "value")])).unwrap(), r#"{"key":"value"}"#);
        assert!([
            r#"{"one":1,"two":2,"three":3}"#,
            r#"{"one":1,"three":3,"two":2}"#,
            r#"{"two":2,"one":1,"three":3}"#,
            r#"{"two":2,"three":3,"one":1}"#,
            r#"{"three":3,"one":1,"two":2}"#,
            r#"{"three":3,"two":2,"one":1}"#,
        ]
        .contains(&&to_string(HashMap::from([("one", 1), ("two", 2), ("three", 3)])).unwrap()[..]));
        assert_eq!(
            to_string(BTreeMap::from([(
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
            to_string(Linked {
                data: 1,
                next: Some(Box::new(Linked { data: 2, next: Some(Box::new(Linked { data: 3, next: None })) }))
            })
            .unwrap(),
            r#"{"data":1,"next":{"data":2,"next":{"data":3,"next":null}}}"#
        );
    }

    #[test]
    fn test_serialize_struct() {
        #[derive(Serialize)]
        struct UnitStruct;
        assert_eq!(to_string(UnitStruct).unwrap(), "null");

        #[derive(Serialize)]
        struct Lattice(usize, usize);
        assert_eq!(to_string(Lattice(1, 2)).unwrap(), "[1,2]");
        assert_eq!(to_string(vec![Lattice(1, 2), Lattice(3, 4)]).unwrap(), "[[1,2],[3,4]]");

        #[derive(Serialize)]
        struct Person {
            name: String,
            age: Option<u32>,
        }
        assert_eq!(
            to_string(Person { name: "Alice".to_string(), age: None }).unwrap(),
            r#"{"name":"Alice","age":null}"#
        );
        assert_eq!(to_string(Person { name: "Bob".to_string(), age: Some(42) }).unwrap(), r#"{"name":"Bob","age":42}"#);
    }

    #[test]
    fn test_serialize_enum() {
        #[derive(Serialize)]
        enum Lattice {
            D1(usize),
            D2(usize, usize),
            D3(usize, usize, usize),
        }
        assert_eq!(to_string(Lattice::D1(3)).unwrap(), r#"{"D1":3}"#);
        assert_eq!(to_string(Lattice::D2(3, 5)).unwrap(), r#"{"D2":[3,5]}"#);
        assert_eq!(to_string(Lattice::D3(3, 5, 7)).unwrap(), r#"{"D3":[3,5,7]}"#);

        #[derive(Serialize)]
        enum Enum {
            Unit,
            Tuple(usize, isize, String),
            Struct { num: u64, text: String, bool: bool },
        }
        assert_eq!(to_string(Enum::Unit).unwrap(), r#""Unit""#);
        assert_eq!(to_string(Enum::Tuple(1, -2, "three".to_string())).unwrap(), r#"{"Tuple":[1,-2,"three"]}"#);
        assert_eq!(
            to_string(Enum::Struct { num: 1, text: "two".to_string(), bool: true }).unwrap(),
            r#"{"Struct":{"num":1,"text":"two","bool":true}}"#
        );
    }

    #[test]
    fn test_serialize_pretty() {
        assert_eq!(
            to_string_pretty(vec!["string", "string", "string", "string", "string", "string"]).unwrap(),
            [
                r#"["#,
                r#"  "string","#,
                r#"  "string","#,
                r#"  "string","#,
                r#"  "string","#,
                r#"  "string","#,
                r#"  "string","#,
                r#"]"#
            ]
            .join("\n")
        );

        assert_eq!(
            to_string_pretty((
                (),
                1,
                "two",
                HashMap::from([("hoge", HashMap::from([("fuga", "piyo")]))]),
                vec![true, false]
            ),)
            .unwrap(),
            [
                r#"["#,
                r#"  null,"#,
                r#"  1,"#,
                r#"  "two","#,
                r#"  {"#,
                r#"    "hoge": {"#,
                r#"      "fuga": "piyo","#,
                r#"    },"#,
                r#"  },"#,
                r#"  ["#,
                r#"    true,"#,
                r#"    false,"#,
                r#"  ],"#,
                r#"]"#,
            ]
            .join("\n")
        )
    }
}
