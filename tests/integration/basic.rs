use json_with_comments::{from_str, from_str_raw, to_string, to_string_pretty};
use serde::{Deserialize, Serialize};

#[test]
fn test_basic_object() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Data {
        schema: String,
        phantom: (),
        trailing_comma: bool,
    }
    let target = r#"{
        "schema": "jsonc",
        "phantom": null,
        "trailing_comma": true,
    }"#;

    // deserialize
    let data: Data = from_str(target).unwrap();
    assert_eq!(data.schema, "jsonc");
    assert_eq!(data.phantom, ());
    assert_eq!(data.trailing_comma, true);

    // serialize
    let jsonc = to_string_pretty(data, Default::default()).unwrap();
    for (tl, jl) in jsonc.lines().zip(target.lines()) {
        assert_eq!(tl.trim(), jl.trim());
    }
}

#[test]
fn test_json() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Event {
        name: String,
        description: String,
        members: Vec<Member>,
        schedule: Schedule,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Member {
        name: String,
        adult: bool,
        height: Option<f64>,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Schedule {
        year: u32,
        month: u16,
        day: u8,
    }
    let target = r#"[
        {
            "name": "event🥳",
            "description": "this is party\u0F12\nhappy new year🎉",
            "members": [
                {
                    "name": "json string",
                    "adult": true,
                    "height": 1.7,
                },
                {
                    "name": "jsonc string",
                    "adult": false,
                    "height": null,
                },
            ],
            "schedule": {
                "year": 2024,
                "month": 1,
                "day": 1,
            },
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
        },
    ]"#;

    // deserialize
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

    // serialize
    let jsonc = to_string(&events).unwrap();
    let re: Vec<Event> = from_str(&jsonc).unwrap();
    assert_eq!(events, re);

    // let jsonc = to_string_pretty(events, Default::default()).unwrap();
    // for (tl, jl) in jsonc.lines().zip(target.lines()) {
    //     assert_eq!(tl.trim(), jl.trim());
    // }
}

#[test]
fn test_json_with_comment() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Setting<'a> {
        name: &'a str,
        image: Option<&'a str>,
        remote_user: Option<&'a str>,
        mounts: Option<Vec<&'a str>>,
    }
    let target = r#"{
        "name": "Debian",
        "image": "mcr.microsoft.com/vscode/devcontainers/base:0-bullseye",
        "remoteUser": "vscode",
        "mounts": null,
    }"#;

    // deserialize
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

    // serialize
    let jsonc = to_string_pretty(setting, Default::default()).unwrap();
    for (tl, jl) in jsonc.lines().zip(target.lines()) {
        assert_eq!(tl.trim(), jl.trim());
    }

    let target2 = r#"
        {
            "name": "Debian",  /* built container name is Debian */
            "image": "mcr.microsoft.com/vscode/devcontainers/base:0-bullseye",
            // "remoteUser": "vscode",
            "mounts": null,  /* do not mounts any file */
        }
    "#;

    // deserialize
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

    // serialize
    let jsonc = to_string(&setting).unwrap();
    let re: Setting = from_str(&jsonc).unwrap();
    assert_eq!(setting, re);
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
fn test_deserialize_enum() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Server {
        name: String,
        ip: std::net::IpAddr,
        port: u16,
        kind: Kind,
        host: Host,
        machine: Machine,
    }
    #[derive(Deserialize, PartialEq, Debug)]
    enum Kind {
        Web,
        Api(String),
        Db { dbms: String },
    }
    #[derive(Deserialize, PartialEq, Debug)]
    enum Host {
        Local,
        OnPremises(),
        Cloud(String, u32),
    }
    #[derive(Deserialize, PartialEq, Debug)]
    enum Machine {
        Local,
        VirtualMachine {},
        Container { runtime: String, engine: String },
    }

    let target_web_server = r#"{
        "name": "web",
        "port": 8080,
        "ip": "127.0.0.1",
        "kind": "Web",
        "host": "Local",
        "machine": "Local",
    }"#;
    let server = from_str::<Server>(target_web_server).unwrap();
    assert_eq!(
        server,
        Server {
            name: "web".to_string(),
            ip: std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
            kind: Kind::Web,
            host: Host::Local,
            machine: Machine::Local,
        }
    );

    let target_api_server = r#"{
        "name": "api",
        "port": 8080,
        "ip": "127.0.0.1",
        "kind": {"Api": "gRPC"},
        "host": {"OnPremises": []},
        "machine": {"VirtualMachine": {}},
    }"#;
    let server = from_str::<Server>(target_api_server).unwrap();
    assert_eq!(
        server,
        Server {
            name: "api".to_string(),
            ip: std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
            kind: Kind::Api("gRPC".to_string()),
            host: Host::OnPremises(),
            machine: Machine::VirtualMachine {},
        }
    );

    let target_db_server = r#"{
        "name": "db",
        "port": 8080,
        "ip": "127.0.0.1",
        "kind": {"Db": {"dbms": "MySQL"}},
        "host": {"Cloud": ["Google Cloud Platform", 465]},
        "machine": {"Container": {"runtime": "docker", "engine": "Google Kubernetes Engine"}},
    }"#;
    let server = from_str::<Server>(target_db_server).unwrap();
    assert_eq!(
        server,
        Server {
            name: "db".to_string(),
            ip: std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
            kind: Kind::Db { dbms: "MySQL".to_string() },
            host: Host::Cloud("Google Cloud Platform".to_string(), 465),
            machine: Machine::Container {
                runtime: "docker".to_string(),
                engine: "Google Kubernetes Engine".to_string()
            },
        }
    );
}
