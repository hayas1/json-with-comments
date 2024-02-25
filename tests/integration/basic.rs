use std::borrow::Cow;

use json_with_comment::{self, from_str};
use serde::Deserialize;

#[test]
fn test_deserialize_basic_object() {
    #[derive(Deserialize)]
    struct Data {
        schema: String,
        phantom: (),
        trailing_comma: bool,
    }
    let raw = r#"
        {
            "schema": "jsonc",
            "phantom": null,
            "trailing_comma": true,
        }
    "#;

    let data: Data = from_str(raw).unwrap();
    assert_eq!(data.schema, "jsonc");
    assert_eq!(data.phantom, ());
    assert_eq!(data.trailing_comma, true);
}

#[test]
fn test_deserialize_basic_array() {
    let raw = r#"["foo", "bar", "baz"]"#;
    let data: Vec<String> = from_str(raw).unwrap();
    assert_eq!(data, ["foo", "bar", "baz"]);
}

#[test]
fn test_cow_str() {
    #[derive(Deserialize)]
    struct Data<'a> {
        name: Cow<'a, str>,
    }
    let raw = r#"{"name": "cow"}"#;
    let data: Data = from_str(raw).unwrap();
    assert_eq!(data.name, "cow");
}

#[test]
fn test_deserialize_json() {
    #[derive(Deserialize, PartialEq, Eq, Debug)]
    struct Event {
        name: String,
        description: String,
        members: Vec<Member>,
        schedule: Schedule,
    }
    #[derive(Deserialize, PartialEq, Eq, Debug)]
    struct Member {
        first_name: Option<String>,
        last_name: String,
        adult: bool,
    }
    #[derive(Deserialize, PartialEq, Eq, Debug)]
    struct Schedule {
        year: u32,
        month: u16,
        day: u8,
    }
    let raw = r#"
        [
            {
                "name": "event🥳",
                "description": "this is party\u0F12\nhappy new year🎉",
                "members": [
                    {
                        "first_name": "string",
                        "last_name": "json",
                        "adult": true,
                    },
                    {
                        "first_name": null,
                        "last_name": "jsonc",
                        "adult": false,
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
    let events: Vec<Event> = from_str(raw).unwrap();
    let expected = vec![
        Event {
            name: "event🥳".to_string(),
            description: "this is party༒\nhappy new year🎉".to_string(),
            members: vec![
                Member { first_name: Some("string".to_string()), last_name: "json".to_string(), adult: true },
                Member { first_name: None, last_name: "jsonc".to_string(), adult: false },
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
    // TODO
}
