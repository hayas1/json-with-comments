//! JSON with comments parser for Rust.
//! See [documents](https://hayas1.github.io/json-with-comments/json_with_comments/) also.
//!
//! # Usage
//! in `Cargo.toml`
//! ```toml
//! [dependencies]
//! json_with_comments = { git = "https://github.com/hayas1/json-with-comments" }
//! ```
//!
//! # Parse JSONC as typed struct
//! Any type that implements [`serde::Deserialize`] can be deserialized from JSONC text.
//! ```
//! use serde::Deserialize;
//! #[derive(Deserialize)]
//! struct Person<'a> {
//!     name: &'a str,
//!     address: Address<'a>,
//! }
//! #[derive(Deserialize)]
//! struct Address<'a> {
//!     street: &'a str,
//!     number: u32,
//! }
//!
//! let json = r#"{
//!     "name": "John Doe", // John Doe is a fictional character
//!     "address": {
//!         "street": "Main",
//!         "number": 42, /* trailing comma */
//!     },
//! }"#;
//!
//! let data: Person = json_with_comments::from_str(json).unwrap();
//! assert!(matches!(
//!     data,
//!     Person {
//!         name: "John Doe",
//!         address: Address { street: "Main", number: 42 }
//!     }
//! ));
//! ```
//!
//! # Parse JSONC as any value
//! Any valid JSONC text can be parsed as [`Value`].
//! See [`jsonc!`] macro also.
//! ```
//! use json_with_comments::{from_str, Value, jsonc, value::JsoncValue};
//! use json_with_comments::value::{number::Number, MapImpl};
//!
//! let json = r#"{
//!     "name": "John Doe", // John Doe is a fictional character
//!     "address": {
//!         "street": "Main",
//!         "number": 42, /* trailing comma */
//!     },
//! }"#;
//!
//! let data: json_with_comments::Value = from_str(json).unwrap();
//! assert_eq!(data["name"], JsoncValue::String("John Doe".into()));
//! assert_eq!(data["address"]["street"], JsoncValue::String("Main".into()));
//! assert_eq!(data.query("address.number"), Some(&42.into()));
//! assert_eq!(data, jsonc!({ "name": "John Doe", "address": { "street": "Main", "number": 42 }}));
//! ```
//!
//! # Format struct as JSONC text
//! Any type that implements [`serde::Serialize`] can be serialized into JSONC text.
//! ```
//! use serde::Serialize;
//! #[derive(Serialize)]
//! struct Person<'a> {
//!     name: &'a str,
//!     address: Address<'a>,
//! }
//! #[derive(Serialize)]
//! struct Address<'a> {
//!     street: &'a str,
//!     number: u32,
//! }
//!
//! let person = Person {
//!     name: "John Doe",
//!     address: Address {
//!         street: "Main",
//!         number: 42,
//!     },
//! };
//!
//! let minify = r#"{"name":"John Doe","address":{"street":"Main","number":42}}"#;
//! assert_eq!(json_with_comments::to_string(&person).unwrap(), minify);
//!
//! let pretty = r#"{
//!   "name": "John Doe",
//!   "address": {
//!     "street": "Main",
//!     "number": 42,
//!   },
//! }"#;
//! assert_eq!(json_with_comments::to_string_pretty(&person, Default::default()).unwrap(), pretty);
//! ```
//!
//! # Interconversion of `serde_json::Value` and `json_with_comments::Value`
//! Any type of `T` implements [`serde::Serialize`] and [`serde::Deserialize`] can be
//! serialized to and deserialized from `serde_json::Value`, and `json_with_comments::Value` also.
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use serde_json::json;
//! use json_with_comments::jsonc;
//!
//! let (json, jsonc) = (json!({"name": "John Doe","age": 30}), jsonc!({ "name": "John Doe", "age": 30 }));
//!
//! // serde_json::Value -> json_with_comments::Value
//! assert_eq!(json_with_comments::to_value(&json).unwrap(), jsonc);
//! assert_eq!(serde_json::from_value::<json_with_comments::Value>(json.clone()).unwrap(), jsonc);
//!
//! // json_with_comments::Value -> serde_json::Value
//! assert_eq!(json_with_comments::from_value::<serde_json::Value>(&jsonc).unwrap(), json);
//! assert_eq!(serde_json::to_value(jsonc.clone()).unwrap(), json);
//! ```
//!
//! # Testing
//! Coverage can be checked [https://hayas1.github.io/json-with-comments/tarpaulin-report](https://hayas1.github.io/json-with-comments/tarpaulin-report)
//!
//! # Performance
//! // TODO

pub mod de;
pub mod error;
pub mod ser;
pub mod value;

pub use de::{from_file, from_path, from_read, from_str, from_str_raw, from_value};
pub use error::{JsonWithCommentsError as Error, Result};
pub use ser::{to_file, to_file_pretty, to_path, to_path_pretty, to_string, to_string_pretty, to_value, to_write};

pub use ser::formatter::{
    minify::MinifyFormatter,
    pretty::{PrettyFormatter, PrettySettings},
    JsoncFormatter,
};

/// [`Value`] is type alias for [`value::JsoncValue<i64, f64>`].
pub type Value = value::JsoncValue<i64, f64>;
