use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
};

use json_with_comments::{from_str, from_str_raw, to_string, to_string_pretty};

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
    let jsonc = to_string_pretty(&map, Default::default()).unwrap();
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
    let jsonc = to_string_pretty(&map, Default::default()).unwrap();
    for (tl, jl) in target_number_key.lines().zip(jsonc.lines()) {
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
