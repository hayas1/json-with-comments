use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use json_with_comments::{from_str, from_str_raw};
use serde::Deserialize;

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
fn test_deserialize_literal_map_key() {
    let target_bool = r#"{
        "true": 1,
        "false": 0,
        // "null": null,
    }"#;
    let map: HashMap<Option<bool>, Option<i32>> = from_str(target_bool).unwrap();
    assert_eq!(map, HashMap::from([(Some(true), Some(1)), (Some(false), Some(0))]));

    let map: HashMap<bool, i32> = from_str(target_bool).unwrap();
    assert_eq!(map, HashMap::from([(true, 1), (false, 0)]));
}

#[test]
#[should_panic]
fn test_deserialize_null_map_key() {
    let target_bool = r#"{
        "true": 1,
        "false": 0,
        "null": null,
    }"#;
    let map: HashMap<Option<bool>, Option<i32>> = from_str(target_bool).unwrap();
    assert_eq!(map, HashMap::from([(Some(true), Some(1)), (Some(false), Some(0)), (None, None)]));
}

#[test]
fn test_deserialize_numeric_map_key() {
    #[derive(Deserialize)]
    struct Eratosthenes {
        sieve: HashMap<u64, bool>,
        primes: HashSet<u64>,
    }
    let target_eratosthenes = r#"{
        "sieve": {
            "1": false,
            "2": true,
            "3": true,
            "4": false,
            "5": true,
            "6": false,
            "7": true,
            "8": false,
            "9": false,
        },
        "primes": [2,3,5,7],
    }"#;
    let eratosthenes: Eratosthenes = from_str(target_eratosthenes).unwrap();
    assert_eq!(
        eratosthenes.sieve,
        HashMap::from([
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (6, false),
            (7, true),
            (8, false),
            (9, false),
        ]),
    );
    assert_eq!(eratosthenes.primes, HashSet::from([2, 3, 5, 7]),);
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
