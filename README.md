# json-with-comments

JSON with comments parser for Rust.

## Usage
in `Cargo.toml`
```toml
[dependencies]
json_with_comments = { git = "https://github.com/hayas1/json-with-comments" }
```

## Parse JSONC as typed struct
Any type that implements [`serde::Deserialize`] can be deserialized from JSONC text.
```rust
#[derive(serde::Deserialize)]
struct Person<'a> {
    name: &'a str,
    address: Address<'a>,
}
#[derive(serde::Deserialize)]
struct Address<'a> {
    street: &'a str,
    number: u32,
}

let json = r#"{
    "name": "John Doe", // John Doe is a fictional character
    "address": {
        "street": "Main",
        "number": 42, /* trailing comma */
    },
}"#;

let data: Person = json_with_comments::from_str(json).unwrap();
assert!(matches!(
    data,
    Person {
        name: "John Doe",
        address: Address { street: "Main", number: 42 }
    }
));
```

## Performance
// TODO

License: MIT
