use std::borrow::Cow;

use json_with_comment::{from_str, from_str_raw};
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
fn test_deserialize_literal_matchable() {
    #[derive(Deserialize)]
    struct Person<'a> {
        name: &'static str,
        nickname: Option<&'a str>,
        age: u8,
        alive: bool,
    }
    let target = r#"[
        {
            "name": "hayas1",
            "nickname": "hayashi",
            "age": 26,
            "alive": true
        },
        {
            "name": "nobunaga",
            "nickname": null,
            "age": 47,
            "alive": false
        },
        {
            "name": "Ω",
            "nickname": "\u03ad",
            "age": 32,
            "alive": true
        }
    ]"#;
    let people: (Person, Person, Person) = from_str_raw(target).unwrap();
    assert!(matches!(
        people,
        (
            Person { name: "hayas1", nickname: Some("hayashi"), age: 26, alive: true },
            Person { name: "nobunaga", nickname: None, age: 47, alive: false },
            Person { name: "Ω", nickname: Some("\\u03ad"), age: 32, alive: true }
        )
    ));
}

#[test]
fn test_deserialize_ignored() {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Setting {
        name: &'static str,
        image: Option<&'static str>,
        remote_user: Option<&'static str>,
        mounts: Option<Vec<&'static str>>,
    }
    let target = r#"
        {
            "name": "Debian",
            "image": "mcr.microsoft.com/vscode/devcontainers/base:0-bullseye",
            "remoteUser": "vscode",
            "mounts": null,
            "customizations": {}, // this field is not defined in struct
            "features": {}, /* this field is not defined in struct */
        }
    "#;
    let setting = from_str::<Setting>(target).unwrap();
    assert!(matches!(
        setting,
        Setting {
            name: "Debian",
            image: Some("mcr.microsoft.com/vscode/devcontainers/base:0-bullseye"),
            remote_user: Some("vscode"),
            mounts: None
        }
    ));
}
