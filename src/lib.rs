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
//! ```rust
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
//! # Testing
//! Coverage can be checked [https://hayas1.github.io/json-with-comments/tarpaulin-report](https://hayas1.github.io/json-with-comments/tarpaulin-report)
//!
//! # Performance
//! // TODO

pub mod de;
pub mod error;
pub mod value;

pub use de::{from_file, from_path, from_read, from_str, from_str_raw};
pub use error::{JsonWithCommentsError as Error, Result};
