[![Workflow Status](https://github.com/hayas1/json-with-comments/workflows/Master/badge.svg)](https://github.com/hayas1/json-with-comments/actions?query=workflow%3A%22Master%22)

# json-with-comments

JSON with comments parser for Rust.
See [documents](https://hayas1.github.io/json-with-comments/json_with_comments/) also.

## Usage
in `Cargo.toml`
```toml
[dependencies]
json_with_comments = { git = "https://github.com/hayas1/json-with-comments" }
```

## Parse JSONC as typed struct
Any type that implements [`serde::Deserialize`] can be deserialized from JSONC text.
```rust
use serde::Deserialize;
#[derive(Deserialize)]
struct Person<'a> {
    name: &'a str,
    address: Address<'a>,
}
#[derive(Deserialize)]
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

## Parse JSONC as any value
Any valid JSONC text can be parsed as [`Value`].
```rust
use json_with_comments::{from_str, Value, value::JsoncValue};
use json_with_comments::value::{number::Number, MapImpl};

let json = r#"{
    "name": "John Doe", // John Doe is a fictional character
    "address": {
        "street": "Main",
        "number": 42, /* trailing comma */
    },
}"#;

let data: json_with_comments::Value = from_str(json).unwrap();
assert_eq!(data["name"], JsoncValue::String("John Doe".into()));
assert_eq!(data["address"]["street"], JsoncValue::String("Main".into()));
assert_eq!(data.query("address.number"), Some(&42.into()));
```

## Testing
Coverage can be checked [https://hayas1.github.io/json-with-comments/tarpaulin-report](https://hayas1.github.io/json-with-comments/tarpaulin-report)

## Performance
// TODO

License: MIT
