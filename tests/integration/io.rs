use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Person<'a> {
    name: &'a str,
    address: Address<'a>,
    email: &'a str,
    active: bool,
}
#[derive(Deserialize)]
pub struct Address<'a> {
    street: &'a str,
    number: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OwnedPerson {
    name: String,
    address: OwnedAddress,
    email: String,
    active: bool,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OwnedAddress {
    street: String,
    number: u32,
}

#[test]
fn test_deserialize_from_file_with_path() {
    let path = std::path::Path::new("tests/data/john.json");
    let owned_person: OwnedPerson = json_with_comments::from_path(path).unwrap();
    assert_eq!(owned_person.name, "John Doe");
    assert_eq!(owned_person.address.street, "Main");
    assert_eq!(owned_person.address.number, 42);
    assert_eq!(owned_person.email, "x0h5z@example.com");
    assert_eq!(owned_person.active, true);
}

#[test]
fn test_deserialize_from_file() {
    let file = std::fs::File::open("tests/data/john.json").unwrap();
    let owned_person: OwnedPerson = json_with_comments::from_file(&file).unwrap();
    assert_eq!(owned_person.name, "John Doe");
    assert_eq!(owned_person.address.street, "Main");
    assert_eq!(owned_person.address.number, 42);
    assert_eq!(owned_person.email, "x0h5z@example.com");
    assert_eq!(owned_person.active, true);
}

#[test]
fn test_deserialize_from_file_content() {
    let content = std::fs::read_to_string("tests/data/john.json").unwrap();
    let person: Person = json_with_comments::from_str(&content).unwrap();
    assert!(matches!(
        person,
        Person {
            name: "John Doe",
            address: Address { street: "Main", number: 42 },
            email: "x0h5z@example.com",
            active: true,
        }
    ));
}

#[test]
fn test_serialize_to_file() {
    let path = std::path::Path::new("tests/data/john2.json");
    let person = OwnedPerson {
        name: "John Doe".to_string(),
        address: OwnedAddress { street: "Second".to_string(), number: 21 },
        email: "ynqoC@example.com".to_string(),
        active: true,
    };
    json_with_comments::to_path(&person, path).unwrap();

    let content = std::fs::read_to_string(path).unwrap();
    assert_eq!(
        content,
        r#"{"name":"John Doe","address":{"street":"Second","number":21},"email":"ynqoC@example.com","active":true}"#
    );

    let deserialized: OwnedPerson = json_with_comments::from_path(path).unwrap();
    assert_eq!(person, deserialized);
}
