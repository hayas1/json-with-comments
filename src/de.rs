pub mod access;
pub mod position;
pub mod token;

use std::{fs::File, io, path::Path};

use serde::de;

use crate::de::token::{raw::RawTokenizer, read::ReadTokenizer, Tokenizer};

use self::{access::jsonc::JsoncDeserializer, token::str::StrTokenizer};

/// Deserialize a JSON with comments text as type `D`.
///
/// # Examples
/// ```
/// use serde::Deserialize;
/// #[derive(Deserialize)]
/// struct Country {
///     name: String,
///     code: u32,
///     regions: Vec<String>,
/// }
/// let jp = r#"
///     {
///         "name": "Japan",
///         "code": 81,
///         "regions": [
///             "Hokkaido",
///             "Kanto",
///             "Kyushu-Okinawa",
///         ],
///     }
/// "#;
/// let japan: Country = json_with_comments::from_str(jp).unwrap();
/// assert_eq!(japan.name, "Japan");
/// assert_eq!(japan.code, 81);
/// assert_eq!(japan.regions, ["Hokkaido", "Kanto", "Kyushu-Okinawa"]);
/// ```
///
/// # Errors
/// This function can deserialize string as borrowed `&str`.
/// But, if it contain escape sequence such as `"\n"`, cannot deserialize and return `Err`.
/// If you want to deserialize string value as escaped borrowed `&str`, use [`from_str_raw`] instead.
/// ```
/// use std::borrow::Cow;
/// use json_with_comments::from_str;
///
/// let no_escaped = r#"  "string without linefeed"  "#;
/// assert_eq!(from_str::<String>(no_escaped).unwrap(), "string without linefeed");
/// assert_eq!(from_str::<Cow<'_, str>>(no_escaped).unwrap(), "string without linefeed");
/// assert_eq!(from_str::<&str>(no_escaped).unwrap(), "string without linefeed");
///
/// let escaped = r#"  "string with linefeed\n"  "#;
/// assert_eq!(from_str::<String>(escaped).unwrap(), "string with linefeed\n");
/// assert_eq!(from_str::<Cow<'_, str>>(escaped).unwrap(), "string with linefeed\n");
/// assert!(from_str::<&str>(escaped).is_err()); // cannot deserialize as &str because of its lifetime
/// ```
pub fn from_str<'de, D>(s: &'de str) -> crate::Result<D>
where
    D: de::Deserialize<'de>,
{
    from_tokenizer(StrTokenizer::new(s))
}

/// Deserialize a JSON with comments text as type `D`.
/// Deserialized instance may have raw string value that contain escape sequence.
/// This function can deserialize any string value as borrowed `&str`.
///
/// # Examples
/// ```
/// let target = r#"    "\"q\" \\s\/ l\n"    "#;
/// let no_escaped: &str = json_with_comments::from_str_raw(target).unwrap();
/// assert_eq!(no_escaped, r#"\"q\" \\s\/ l\n"#);
///
/// let unescaped: String = json_with_comments::from_str(target).unwrap();
/// assert_eq!(unescaped, "\"q\" \\s/ l\n");
/// ```
///
/// # Notes
/// This function can deserialize any string as borrowed `&str`.
/// But, if it contain escape sequence such as `"\n"`, it be deserialized as it is.
/// If you need to deserialize string value as unescaped owned `String`, use [`from_str`].
/// ```
/// use std::borrow::Cow;
/// use json_with_comments::from_str_raw;
///
/// let no_escaped = r#"  "string without linefeed"  "#;
/// assert_eq!(from_str_raw::<String>(no_escaped).unwrap(), "string without linefeed");
/// assert_eq!(from_str_raw::<Cow<'_, str>>(no_escaped).unwrap(), "string without linefeed");
/// assert_eq!(from_str_raw::<&str>(no_escaped).unwrap(), "string without linefeed");
///
/// let escaped = r#"  "string with linefeed\n"  "#;
/// assert_eq!(from_str_raw::<String>(escaped).unwrap(), "string with linefeed\\n");
/// assert_eq!(from_str_raw::<Cow<'_, str>>(escaped).unwrap(), "string with linefeed\\n");
/// assert_eq!(from_str_raw::<&str>(escaped).unwrap(), "string with linefeed\\n"); // deserialized as it is
/// ```
pub fn from_str_raw<'de, D>(s: &'de str) -> crate::Result<D>
where
    D: de::Deserialize<'de>,
{
    from_raw(s.as_bytes())
}

/// Deserialize a JSON with comments text of the given path as type `D`.
///
/// # Examples
/// ```
/// use serde::Deserialize;
/// #[derive(Deserialize)]
/// struct Product {
///     name: String,
///     price: u32,
/// }
///
/// // {
/// //     "name": "candy",
/// //     "price": 100
/// // }
/// let path = std::path::Path::new("tests/data/product.json");
/// let product: Product = json_with_comments::from_path(path).unwrap();
/// assert_eq!(product.name, "candy");
/// assert_eq!(product.price, 100);
/// ```
///
/// # Errors
/// This function cannot deserialize string value as borrowed `&str`.
/// It cause compile time error, same as [`from_file`].
/// ```compile_fail
/// use serde::Deserialize;
/// #[derive(Deserialize)]
/// struct Product<'a> {
///     name: &'a str,
///     price: u32,
/// }
/// // {
/// //     "name": "candy",
/// //     "price": 100
/// // }
/// let path = std::path::Path::new("tests/data/product.json");
/// let product: Product = json_with_comments::from_path(path).unwrap(); // implementation of `Deserialize` is not general enough
/// assert_eq!(product.name, "candy");
/// assert_eq!(product.price, 100);
/// ```
pub fn from_path<D>(p: &Path) -> crate::Result<D>
where
    D: de::DeserializeOwned,
{
    from_file(&File::open(p)?)
}

/// Deserialize a JSON with comments text of the given file as type `D`.
///
/// # Examples
/// ```
/// use serde::Deserialize;
/// #[derive(Deserialize)]
/// struct Product {
///     name: String,
///     price: u32,
/// }
///
/// // {
/// //     "name": "candy",
/// //     "price": 100
/// // }
/// let file = std::fs::File::open("tests/data/product.json").unwrap();
/// let product: Product = json_with_comments::from_file(&file).unwrap();
/// assert_eq!(product.name, "candy");
/// assert_eq!(product.price, 100);
/// ```
///
/// # Errors
/// This function cannot deserialize string value as borrowed `&str`.
/// It cause compile time error, same as [`from_file`].
/// ```compile_fail
/// use serde::Deserialize;
/// #[derive(Deserialize)]
/// struct Product<'a> {
///     name: &'a str,
///     price: u32,
/// }
/// // {
/// //     "name": "candy",
/// //     "price": 100
/// // }
/// let file = std::fs::File::open("tests/data/product.json").unwrap();
/// let product: Product = json_with_comments::from_file(&file).unwrap(); // implementation of `Deserialize` is not general enough
/// assert_eq!(product.name, "candy");
/// assert_eq!(product.price, 100);
/// ```
pub fn from_file<D>(f: &File) -> crate::Result<D>
where
    D: de::DeserializeOwned,
{
    from_read(f)
}

/// Deserialize a JSON with comments text from the given reader as type `D`.
///
/// # Examples
/// ```
/// use serde::Deserialize;
/// #[derive(Deserialize)]
/// struct Product {
///     name: String,
///     price: u32,
/// }
///
/// let read = r#"
/// {
///     "name": "candy",
///     "price": 100
/// }
/// "#.trim().as_bytes();
/// let product: Product = json_with_comments::from_read(read).unwrap();
/// assert_eq!(product.name, "candy");
/// assert_eq!(product.price, 100);
/// ```
///
/// # Errors
/// This function cannot deserialize string value as borrowed `&str`.
/// It cause compile time error.
/// If you want to deserialize string value as escaped borrowed `&str`, use [`from_str_raw`] instead.
/// If you want to deserialize string value as unescaped owned `String`, use [`from_str`] instead.
/// ```compile_fail
/// use serde::Deserialize;
/// #[derive(Deserialize)]
/// struct Product<'a> {
///     name: &'a str,
///     price: u32,
/// }
/// // {
/// //     "name": "candy",
/// //     "price": 100
/// // }
/// let read = std::fs::File::open("tests/data/product.json").unwrap();
/// let product: Product = json_with_comments::from_read(&read).unwrap(); // implementation of `Deserialize` is not general enough
/// assert_eq!(product.name, "candy");
/// assert_eq!(product.price, 100);
/// ```
pub fn from_read<R, D>(read: R) -> crate::Result<D>
where
    R: io::Read,
    D: de::DeserializeOwned,
{
    from_tokenizer(ReadTokenizer::new(read))
}

/// TODO doc
pub fn from_raw<'de, D>(s: &'de [u8]) -> crate::Result<D>
where
    D: de::Deserialize<'de>,
{
    from_tokenizer(RawTokenizer::new(s))
}

/// TODO doc
pub fn from_tokenizer<'de, T, D>(tokenizer: T) -> crate::Result<D>
where
    T: 'de + Tokenizer<'de>,
    D: de::Deserialize<'de>,
{
    let mut de = JsoncDeserializer::new(tokenizer);
    let value = de::Deserialize::deserialize(&mut de)?;
    de.finish()?;

    Ok(value)
}
