use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

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
        "kind": {"Web": null},
        "host": {"Local": null},
        "machine": {"Local": null},
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
