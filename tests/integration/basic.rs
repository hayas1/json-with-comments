use std::borrow::Cow;

use json_with_comment::{self, de::from::from_str_raw, from_str};
use serde::Deserialize;

#[test]
fn test_deserialize_basic_object() {
    #[derive(Deserialize)]
    struct Data {
        schema: String,
        phantom: (),
        trailing_comma: bool,
    }
    let target = r#"
        {
            "schema": "jsonc",
            "phantom": null,
            "trailing_comma": true,
        }
    "#;

    let data: Data = from_str(target).unwrap();
    assert_eq!(data.schema, "jsonc");
    assert_eq!(data.phantom, ());
    assert_eq!(data.trailing_comma, true);
}

#[test]
fn test_deserialize_basic_array() {
    let target = r#"["foo", "bar", "baz"]"#;
    let data: Vec<String> = from_str(target).unwrap();
    assert_eq!(data, ["foo", "bar", "baz"]);
}

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
fn test_deserialize_json() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Event {
        name: String,
        description: String,
        members: Vec<Member>,
        schedule: Schedule,
    }
    #[derive(Deserialize, PartialEq, Debug)]
    struct Member {
        name: String,
        adult: bool,
        height: Option<f64>,
    }
    #[derive(Deserialize, PartialEq, Debug)]
    struct Schedule {
        year: u32,
        month: u16,
        day: u8,
    }
    let target = r#"
        [
            {
                "name": "event🥳",
                "description": "this is party\u0F12\nhappy new year🎉",
                "members": [
                    {
                        "name": "json string",
                        "adult": true,
                        "height": 1.7
                    },
                    {
                        "name": "jsonc string",
                        "adult": false,
                        "height": null
                    }
                ],
                "schedule": {
                    "year": 2024,
                    "month": 1,
                    "day": 1,
                }
            },
            {
                "name": "empty",
                "description": "",
                "members": [],
                "schedule": {
                    "year": 0,
                    "month": 0,
                    "day": 0,
                },
            }
        ]
    "#;
    let events: Vec<Event> = from_str(target).unwrap();
    let expected = vec![
        Event {
            name: "event🥳".to_string(),
            description: "this is party༒\nhappy new year🎉".to_string(),
            members: vec![
                Member { name: "json string".to_string(), adult: true, height: Some(1.7) },
                Member { name: "jsonc string".to_string(), adult: false, height: None },
            ],
            schedule: Schedule { year: 2024, month: 1, day: 1 },
        },
        Event {
            name: "empty".to_string(),
            description: "".to_string(),
            members: vec![],
            schedule: Schedule { year: 0, month: 0, day: 0 },
        },
    ];
    assert_eq!(events, expected);
}

#[test]
fn test_deserialize_json_with_comment() {
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
    let target2 = r#"
        {
            "name": "Debian",  /* built container name is Debian */
            "image": "mcr.microsoft.com/vscode/devcontainers/base:0-bullseye",
            // "remoteUser": "vscode",
            "mounts": null,  /* do not mounts any file */
        }
    "#;
    let setting = from_str::<Setting>(target2).unwrap();
    assert!(matches!(
        setting,
        Setting {
            name: "Debian",
            image: Some("mcr.microsoft.com/vscode/devcontainers/base:0-bullseye"),
            remote_user: None,
            mounts: None
        }
    ));
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
    let people: Vec<Person> = from_str_raw(target).unwrap();
    assert!(matches!(people[0], Person { name: "hayas1", nickname: Some("hayashi"), age: 26, alive: true }));
    assert!(matches!(people[1], Person { name: "nobunaga", nickname: None, age: 47, alive: false }));
    assert!(matches!(people[2], Person { name: "Ω", nickname: Some("\\u03ad"), age: 32, alive: true }));
}
