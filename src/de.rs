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
/// {
///     "name": "Japan",
///     "code": 81,
///     "regions": [
///         "Hokkaido",
///         "Kanto",
///         "Kyushu-Okinawa",
///     ],
/// }"#;
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
/// }"#.as_bytes();
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

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use super::*;

    #[test]
    fn test_deserialize_literal() {
        assert_eq!(from_str::<bool>("true").unwrap(), true);
        assert_eq!(from_str::<bool>("false").unwrap(), false);
        assert_eq!(from_str::<()>("null").unwrap(), ());
    }

    #[test]
    fn test_deserialize_string() {
        assert_eq!(from_str::<String>(r#""hello world""#).unwrap(), "hello world".to_string());
        assert_eq!(from_str::<&str>(r#""12345""#).unwrap(), "12345");
        assert_eq!(from_str::<String>(r#""🥒💯""#).unwrap(), "🥒💯".to_string());

        assert_eq!(from_str::<String>(r#""linefeed\n""#).unwrap(), "linefeed\n");
        assert_eq!(from_str::<String>(r#""tab\tspace""#).unwrap(), "tab\tspace");
        assert_eq!(from_str::<String>(r#""linefeed\u000A""#).unwrap(), "linefeed\n");
        assert_eq!(from_str::<String>(r#""null\u0000""#).unwrap(), "null\u{0000}");
        assert_eq!(from_str::<String>(r#""del\u007f""#).unwrap(), "del\u{007F}");
    }

    #[test]
    fn test_deserialize_number() {
        assert_eq!(from_str::<u64>("57").unwrap(), 57);
        assert_eq!(from_str::<i128>("-99999999999999999").unwrap(), -99999999999999999);
        assert_eq!(from_str::<f32>("3.1415926535").unwrap(), 3.1415926535);
        assert_eq!(from_str::<f64>("6.02214076e23").unwrap(), 6.02214076E23);
    }

    #[test]
    fn test_deserialize_seq() {
        assert_eq!(from_str::<Vec<()>>("[]").unwrap(), vec![]);
        assert_eq!(from_str::<Vec<i32>>("[1,2,3]").unwrap(), vec![1, 2, 3]);
        assert_eq!(
            from_str::<((), bool, String)>(r#"[null, true, "string"]"#).unwrap(),
            ((), true, "string".to_string())
        );
        assert_eq!(from_str::<((), Vec<bool>)>(r#"[null, [false, true]]"#).unwrap(), ((), vec![false, true]));
    }

    #[test]
    fn test_deserialize_map() {
        assert_eq!(from_str::<HashMap<(), ()>>("{}").unwrap(), HashMap::new());
        assert_eq!(
            from_str::<HashMap<String, String>>(r#"{"key":"value"}"#).unwrap(),
            HashMap::from([("key".to_string(), "value".to_string())])
        );
        assert_eq!(
            from_str::<BTreeMap<i64, &str>>(r#"{"1": "one", "2": "two", "3": "three"}"#).unwrap(),
            BTreeMap::from([(1, "one"), (2, "two"), (3, "three")])
        );
        assert_eq!(
            from_str::<BTreeMap<&str, HashMap<&str, &str>>>(r#"{"hoge":{"fuga":"piyo"},"foo":{"bar":"baz"}}"#).unwrap(),
            BTreeMap::from([("hoge", HashMap::from([("fuga", "piyo")])), ("foo", HashMap::from([("bar", "baz")]))])
        )
    }

    #[test]
    fn test_deserialize_struct_and_enum() {
        #[derive(serde::Deserialize)]
        struct Person<'a> {
            name: &'a str,
            age: Option<u32>,
            family: Family<'a>,
        }
        #[derive(serde::Deserialize)]
        enum Family<'a> {
            Single,
            Parent(&'a str),
            Children { brother: &'a str, sister: &'a str },
        }

        assert!(matches!(
            from_str(r#"{"name": "John", "age": 30, "family": "Single"}"#),
            Ok(Person { name: "John", age: Some(30), family: Family::Single })
        ));
        assert!(matches!(
            from_str(r#"{"name": "Jin", "age": null, "family": {"Parent": "Jane"}}"#),
            Ok(Person { name: "Jin", age: None, family: Family::Parent("Jane") })
        ));
        assert!(matches!(
            from_str(r#"{"name":"John","age":55,"family":{"Children": {"brother": "Jim", "sister": "Kate"}}}"#),
            Ok(Person { name: "John", age: Some(55), family: Family::Children { brother: "Jim", sister: "Kate" } })
        ));
    }

    #[test]
    fn test_deserialize_with_comments() {
        let target = r#"{
            "name": "JSON with comments", // JSON with comments allow JavaScript style comments.
            "keywords": [
                "JSON",
                "JSONC",
                "trailing comma", /* JSON with comments allow trailing comma */
            ],
        }"#;

        #[derive(serde::Deserialize, Debug, PartialEq)]
        struct Jsonc<'a> {
            name: &'a str,
            keywords: Vec<&'a str>,
        }
        assert_eq!(
            from_str::<Jsonc>(target).unwrap(),
            Jsonc { name: "JSON with comments", keywords: vec!["JSON", "JSONC", "trailing comma"] }
        );
    }

    #[test]
    fn test_deserialize_ignored() {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Setting {
            name: &'static str,
            image: Option<&'static str>,
            remote_user: Option<&'static str>,
            mounts: Option<Vec<&'static str>>,
        }
        let target = r#"{
            "name": "Debian",
            "image": "mcr.microsoft.com/vscode/devcontainers/base:0-bullseye",
            "remoteUser": "vscode",
            "mounts": null,
            "customizations": {}, // this field is not defined in struct
            "features": {}, /* this field is not defined in struct */
        }"#;
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
}
