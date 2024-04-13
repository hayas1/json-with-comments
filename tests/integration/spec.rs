use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
};

use json_with_comments::{from_str, from_str_raw, to_string, to_string_pretty};
use serde::{Deserialize, Serialize};

#[test]
fn test_deserialize_str() {
    let unescaped = r#""string without linefeed""#;
    assert_eq!(from_str::<String>(unescaped).unwrap(), "string without linefeed");
    assert_eq!(from_str::<Cow<'_, str>>(unescaped).unwrap(), "string without linefeed");
    assert_eq!(from_str::<&str>(unescaped).unwrap(), "string without linefeed");
    assert_eq!(from_str_raw::<String>(unescaped).unwrap(), r#"string without linefeed"#);
    assert_eq!(from_str_raw::<Cow<'_, str>>(unescaped).unwrap(), r#"string without linefeed"#);
    assert_eq!(from_str_raw::<&str>(unescaped).unwrap(), r#"string without linefeed"#);

    let escaped = r#""string with linefeed\n""#;
    assert_eq!(from_str::<String>(escaped).unwrap(), "string with linefeed\n");
    assert_eq!(from_str::<Cow<'_, str>>(escaped).unwrap(), "string with linefeed\n");
    assert!(from_str::<&str>(escaped).is_err(), "borrowed string that has escape cannot be deserialized (lifetime)");
    assert_eq!(from_str_raw::<String>(escaped).unwrap(), r#"string with linefeed\n"#);
    assert_eq!(from_str_raw::<Cow<'_, str>>(escaped).unwrap(), r#"string with linefeed\n"#);
    assert_eq!(from_str_raw::<&str>(escaped).unwrap(), r#"string with linefeed\n"#);
}

#[test]
fn test_null() {
    let none = None::<()>;
    let string = to_string(none).unwrap();
    assert_eq!(string, "null");
    let re: Option<()> = from_str(&string).unwrap();
    assert_eq!(re, None);

    let some_unit = Some(());
    let string = to_string(some_unit).unwrap();
    assert_eq!(string, "null");
    let re: Option<()> = from_str(&string).unwrap();
    // assert_eq!(re, Some(()));
    assert_eq!(re, None);
}

#[test]
fn test_not_string_map_key() {
    let target_bool = r#"{
        "false": 0,
        "true": 1,
    }"#;
    let map: BTreeMap<bool, i32> = from_str(target_bool).unwrap();
    assert_eq!(map, BTreeMap::from([(true, 1), (false, 0)]));
    let jsonc = to_string_pretty(&map).unwrap();
    for (tl, jl) in target_bool.lines().zip(jsonc.lines()) {
        assert_eq!(tl.trim(), jl.trim());
    }

    let target_number_key = r#"{
        "1": false,
        "2": true,
        "3": true,
        "4": false,
        "5": true,
    }"#;
    let map: BTreeMap<u64, bool> = from_str(target_number_key).unwrap();
    assert_eq!(map, BTreeMap::from([(1, false), (2, true), (3, true), (4, false), (5, true)]));
    let jsonc = to_string_pretty(&map).unwrap();
    for (tl, jl) in target_number_key.lines().zip(jsonc.lines()) {
        assert_eq!(tl.trim(), jl.trim());
    }

    let target_unit_key = r#"{
        "null": false,
    }"#;
    let map: HashMap<(), bool> = from_str(target_unit_key).unwrap();
    assert_eq!(map, HashMap::from([((), false)]));
    let jsonc = to_string_pretty(&map).unwrap();
    for (tl, jl) in target_unit_key.lines().zip(jsonc.lines()) {
        assert_eq!(tl.trim(), jl.trim());
    }
}

#[test]
#[should_panic]
fn test_cannot_deserialize_null_map_key() {
    let target_bool = r#"{
        "true": 1,
        "false": 0,
        "null": null,
    }"#;
    let map: HashMap<Option<bool>, Option<i32>> = from_str(target_bool).unwrap();
    assert_eq!(map, HashMap::from([(Some(true), Some(1)), (Some(false), Some(0)), (None, None)]));
}

#[test]
fn test_can_deserialize_duplicated_map_key() {
    let target = r#"{
        "hello": "world",
        "hello": "world!"
    }"#;

    let map: HashMap<&str, &str> = from_str(target).unwrap();
    assert_eq!(
        map,
        HashMap::from([
            // ("hello", "world"),
            ("hello", "world!"),
        ])
    );

    let jsonc = to_string(&map).unwrap();
    assert_eq!(jsonc, r#"{"hello":"world!"}"#);
}

#[test]
#[should_panic]
fn test_cannot_deserialize_seq_map_key() {
    let target_reversi = r#"{
        "['D', 4]": true,
        "['E', 4]": false,
        "['D', 5]": false,
        "['E', 5]": true,
    }"#;
    let reversi: HashMap<(char, usize), bool> = from_str(target_reversi).unwrap();
    assert_eq!(reversi, HashMap::from([(('D', 4), true), (('E', 4), false), (('D', 5), false), (('E', 5), true)]));
}

#[test]
fn test_roundtrip_unit() {
    let target = ();
    let jsonc = to_string(target).unwrap();
    assert_eq!(jsonc, "null");
    let unit = from_str::<()>(&jsonc).unwrap();
    assert_eq!(unit, target);
}

#[test]
fn test_roundtrip_bool() {
    let target = true;
    let jsonc = to_string(target).unwrap();
    assert_eq!(jsonc, "true");
    let tru = from_str::<bool>(&jsonc).unwrap();
    assert_eq!(tru, target);

    let target = false;
    let jsonc = to_string(target).unwrap();
    assert_eq!(jsonc, "false");
    let fal = from_str::<bool>(&jsonc).unwrap();
    assert_eq!(fal, target);
}

#[test]
fn test_roundtrip_number() {
    let target = 42;
    let jsonc = to_string(target).unwrap();
    assert_eq!(jsonc, "42");
    let unsigned = from_str::<i32>(&jsonc).unwrap();
    assert_eq!(unsigned, target);

    let target = -42;
    let jsonc = to_string(target).unwrap();
    assert_eq!(jsonc, "-42");
    let signed = from_str::<i32>(&jsonc).unwrap();
    assert_eq!(signed, target);

    let target = 42.75;
    let jsonc = to_string(target).unwrap();
    assert_eq!(jsonc, "42.75");
    let fraction = from_str::<f32>(&jsonc).unwrap();
    assert_eq!(fraction, target);

    let target = 6.02E23;
    let jsonc = to_string(target).unwrap();
    assert_eq!(jsonc, "6.02e23");
    let exponent = from_str::<f64>(&jsonc).unwrap();
    assert_eq!(exponent, target);
}

#[test]
fn test_roundtrip_string() {
    let target = "hello";
    let jsonc = to_string(target).unwrap();
    assert_eq!(jsonc, r#""hello""#);
    let string = from_str::<String>(&jsonc).unwrap();
    assert_eq!(string, target);

    let target = "hello\nworld";
    let jsonc = to_string(target).unwrap();
    assert_eq!(jsonc, r#""hello\nworld""#);
    let string = from_str::<String>(&jsonc).unwrap();
    assert_eq!(string, target);
}

#[test]
fn test_roundtrip_seq() {
    let target = vec!["hoge", "fuga", "piyo"];
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, r#"["hoge","fuga","piyo"]"#);
    let metasyntactic = from_str::<Vec<&str>>(&jsonc).unwrap();
    assert_eq!(metasyntactic, target);

    let target = (false, 1, ((), vec![()]));
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, r#"[false,1,[null,[null]]]"#);
    let natural = from_str::<(bool, i32, ((), Vec<()>))>(&jsonc).unwrap();
    assert_eq!(natural, target);
}

#[test]
fn test_roundtrip_map() {
    let target = HashMap::from([("foo", "bar"), ("baz", "qux")]);
    let jsonc = to_string(&target).unwrap();
    assert!(jsonc == r#"{"foo":"bar","baz":"qux"}"# || jsonc == r#"{"baz":"qux","foo":"bar"}"#);
    let foobar = from_str::<HashMap<&str, &str>>(&jsonc).unwrap();
    assert_eq!(foobar, target);

    let target = BTreeMap::from([("one", 1), ("two", 2), ("three", 3)]);
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, r#"{"one":1,"three":3,"two":2}"#);
    let ott = from_str::<BTreeMap<&str, i32>>(&jsonc).unwrap();
    assert_eq!(ott, target);
}

#[test]
fn test_roundtrip_option() {
    let target = Some(42);
    let jsonc = to_string(target).unwrap();
    assert_eq!(jsonc, "42");
    let some = from_str::<Option<i32>>(&jsonc).unwrap();
    assert_eq!(some, target);

    let target = None;
    let jsonc = to_string(target).unwrap();
    assert_eq!(jsonc, "null");
    let none = from_str::<Option<i32>>(&jsonc).unwrap();
    assert_eq!(none, target);
}

#[test]
fn test_roundtrip_struct() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Marker;
    let target = Marker;
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, "null");
    let marker = from_str::<Marker>(&jsonc).unwrap();
    assert_eq!(marker, target);

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Coordinate(isize, isize);
    let target = Coordinate(1, 2);
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, r#"[1,2]"#);
    let coordinate = from_str::<Coordinate>(&jsonc).unwrap();
    assert_eq!(coordinate, target);

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Person {
        name: String,
        age: i32,
    }
    let target = Person { name: "John".to_string(), age: 21 };
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, r#"{"name":"John","age":21}"#);
    let person = from_str::<Person>(&jsonc).unwrap();
    assert_eq!(person, target);

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct User {
        marker: Option<Marker>,
        coordinate: Coordinate,
        person: Person,
    }
    let target =
        User { marker: None, coordinate: Coordinate(1, 2), person: Person { name: "John".to_string(), age: 21 } };
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, r#"{"marker":null,"coordinate":[1,2],"person":{"name":"John","age":21}}"#);
    let user = from_str::<User>(&jsonc).unwrap();
    assert_eq!(user, target);
}

#[test]
fn test_roundtrip_enum() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Marker {
        Foo,
        Bar(i32),
        Baz(i32, i32),
        Qux { hoge: String, fuga: bool },
        Quux(Box<Marker>),
    }
    let target = Marker::Foo;
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, r#""Foo""#);
    let foo = from_str::<Marker>(&jsonc).unwrap();
    assert_eq!(foo, target);

    let target = Marker::Bar(42);
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, r#"{"Bar":42}"#);
    let bar = from_str::<Marker>(&jsonc).unwrap();
    assert_eq!(bar, target);

    let target = Marker::Baz(1, 2);
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, r#"{"Baz":[1,2]}"#);
    let baz = from_str::<Marker>(&jsonc).unwrap();
    assert_eq!(baz, target);

    let target = Marker::Qux { hoge: "hoge".to_string(), fuga: true };
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, r#"{"Qux":{"hoge":"hoge","fuga":true}}"#);
    let qux = from_str::<Marker>(&jsonc).unwrap();
    assert_eq!(qux, target);

    let target = Marker::Quux(Box::new(Marker::Qux { hoge: "hoge".to_string(), fuga: true }));
    let jsonc = to_string(&target).unwrap();
    assert_eq!(jsonc, r#"{"Quux":{"Qux":{"hoge":"hoge","fuga":true}}}"#);
    let quux = from_str::<Marker>(&jsonc).unwrap();
    assert_eq!(quux, target);
}

#[test]
fn test_roundtrip_compound() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Linked<T> {
        Next { value: T, next: Box<Linked<T>> },
        End,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Compound {
        name: String,
        value: Linked<u32>,
    }

    let target = Compound {
        name: "values".to_string(),
        value: Linked::Next {
            value: 32,
            next: Box::new(Linked::Next {
                value: 64,
                next: Box::new(Linked::Next { value: 128, next: Box::new(Linked::End) }),
            }),
        },
    };
    let jsonc = to_string(&target).unwrap();
    assert_eq!(
        jsonc,
        r#"{"name":"values","value":{"Next":{"value":32,"next":{"Next":{"value":64,"next":{"Next":{"value":128,"next":"End"}}}}}}}"#
    );
    let compound = from_str::<Compound>(&jsonc).unwrap();
    assert_eq!(compound, target);
}
