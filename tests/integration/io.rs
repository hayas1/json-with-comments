#[derive(serde::Deserialize)]
pub struct Person<'a> {
    name: &'a str,
    address: Address<'a>,
    email: &'a str,
    active: bool,
}
#[derive(serde::Deserialize)]
pub struct Address<'a> {
    street: &'a str,
    number: u32,
}

#[derive(serde::Deserialize)]
pub struct OwnedPerson {
    name: String,
    address: OwnedAddress,
    email: String,
    active: bool,
}
#[derive(serde::Deserialize)]
pub struct OwnedAddress {
    street: String,
    number: u32,
}

#[test]
fn deserialize_from_file() {
    let path = std::path::Path::new("tests/data/john.json");
    let owned_person: OwnedPerson = json_with_comment::from_path(path).unwrap();
    assert_eq!(owned_person.name, "John Doe");
    assert_eq!(owned_person.address.street, "Main");
    assert_eq!(owned_person.address.number, 42);
    assert_eq!(owned_person.email, "x0h5z@example.com");
    assert_eq!(owned_person.active, true);

    let file = std::fs::File::open("tests/data/john.json").unwrap();
    let owned_person: OwnedPerson = json_with_comment::from_file(&file).unwrap();
    assert_eq!(owned_person.name, "John Doe");
    assert_eq!(owned_person.address.street, "Main");
    assert_eq!(owned_person.address.number, 42);
    assert_eq!(owned_person.email, "x0h5z@example.com");
    assert_eq!(owned_person.active, true);

    let content = std::fs::read_to_string("tests/data/john.json").unwrap();
    let person: Person = json_with_comment::from_str(&content).unwrap();
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
