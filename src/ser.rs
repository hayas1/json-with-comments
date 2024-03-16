mod access;
mod formatter;

use serde::ser;
use std::{fs::File, io, path::Path};

use self::access::jsonc::JsoncSerializer;

/// Serialize struct `S` as minified JSON with comments text.
/// If you want to serialize as pretty formatted JSONC text, use [`to_str_pretty`] instead.
///
/// # Examples
/// ```
/// use serde::Serialize;
/// #[derive(Serialize)]
/// struct Country {
///     name: String,
///     code: u32,
///     regions: Vec<String>,
/// }
/// let japan = Country {
///     name: "Japan".to_string(),
///     code: 81,
///     regions: vec!["Hokkaido".to_string(), "Kanto".to_string(), "Kyushu-Okinawa".to_string()],
/// };
/// let jp = json_with_comments::to_str(japan).unwrap();
/// assert_eq!(jp, r#"{"name":"Japan","code":81,"regions":["Hokkaido","Kanto","Kyushu-Okinawa"]}"#);
/// ```
pub fn to_str<S>(value: S) -> crate::Result<String>
where
    S: ser::Serialize,
{
    let mut write = Vec::new();
    to_write(value, &mut write, formatter::minify::MinifyFormatter)?;
    Ok(String::from_utf8(write)?)
}

/// Serialize struct `S` as pretty formatted JSON with comments text.
/// If you want to serialize as minified JSONC text, use [`to_str`] instead.
///
/// # Examples
/// ```
/// use serde::Serialize;
/// #[derive(Serialize)]
/// struct Country<'a> {
///     name: &'a str,
///     code: u32,
///     regions: Vec<&'a str>,
/// }
/// let japan = Country {
///     name: "Japan",
///     code: 81,
///     regions: vec!["Hokkaido", "Kanto", "Kyushu-Okinawa"],
/// };
/// let jp = json_with_comments::to_str_pretty(japan, Default::default()).unwrap();
/// assert_eq!(jp, vec![
///     r#"{"#,
///     r#"  "name": "Japan","#,
///     r#"  "code": 81,"#,
///     r#"  "regions": ["#,
///     r#"    "Hokkaido","#,
///     r#"    "Kanto","#,
///     r#"    "Kyushu-Okinawa","#,
///     r#"  ],"#,
///     r#"}"#].join("\n"));
/// ```
pub fn to_str_pretty<S>(value: S, settings: formatter::pretty::PrettySettings) -> crate::Result<String>
where
    S: ser::Serialize,
{
    let mut write = Vec::new();
    to_write(value, &mut write, formatter::pretty::PrettyFormatter::new(settings))?;
    Ok(String::from_utf8(write)?)
}

/// Serialize struct `S` as a minified JSON with comments text of the given path.
/// If you want to serialize as pretty formatted JSONC text, use [`to_path_pretty`] instead.
///
/// # Examples
/// ```
/// use serde::Serialize;
/// #[derive(Serialize)]
/// struct Product {
///     name: String,
///     price: u32,
/// }
///
/// // {"name":"candy","price":100}
/// let path = std::path::Path::new("tests/data/product_minify.jsonc");
/// let before = std::fs::read_to_string(path).unwrap();
///
/// if path.exists() {
///     std::fs::remove_file(path).unwrap();
/// }
///
/// let product = Product {
///     name: "candy".to_string(),
///     price: 100,
/// };
/// json_with_comments::to_path(product, path).unwrap();
/// let after = std::fs::read_to_string(path).unwrap();
/// assert_eq!(before, after);
/// ```
pub fn to_path<S>(value: S, path: &Path) -> crate::Result<()>
where
    S: ser::Serialize,
{
    let mut file = File::create(path)?;
    to_file(value, &mut file)?;
    Ok(())
}

/// Serialize struct `S` as a pretty formatted JSON with comments text of the given path.
/// If you want to serialize as minified JSONC text, use [`to_path`] instead.
///
/// # Examples
/// ```
/// use serde::Serialize;
/// #[derive(Serialize)]
/// struct Product {
///     name: String,
///     price: u32,
/// }
///
/// // {
/// //  "name": "candy",
/// //  "price": 100,
/// // }
/// let path = std::path::Path::new("tests/data/product.jsonc");
/// let before = std::fs::read_to_string(path).unwrap();
///
/// if path.exists() {
///     std::fs::remove_file(path).unwrap();
/// }
///
/// let product = Product {
///     name: "candy".to_string(),
///     price: 100,
/// };
/// json_with_comments::to_path_pretty(product, path, Default::default()).unwrap();
/// let after = std::fs::read_to_string(path).unwrap();
/// assert_eq!(before, after);
/// ```
pub fn to_path_pretty<S>(value: S, path: &Path, settings: formatter::pretty::PrettySettings) -> crate::Result<()>
where
    S: ser::Serialize,
{
    let mut file = File::create(path)?;
    to_file_pretty(value, &mut file, settings)?;
    Ok(())
}

/// Serialize struct `S` as a minified JSON with comments text of the given file.
/// If you want to serialize as pretty formatted JSONC text, use [`to_file_pretty`] instead.
///
/// # Examples
/// ```
/// use serde::Serialize;
/// #[derive(Serialize)]
/// struct Product {
///     name: String,
///     price: u32,
/// }
/// let path = std::path::Path::new("tests/data/product_minify.jsonc");
/// if path.exists() {
///     std::fs::remove_file(path).unwrap();
/// }
/// let mut file = std::fs::File::create(path).unwrap();
/// let product = Product { name: "candy".to_string(), price: 100 };
/// json_with_comments::to_file(product, &mut file).unwrap();
/// assert_eq!(std::fs::read_to_string(path).unwrap(), r#"{"name":"candy","price":100}"#);
/// ```
pub fn to_file<S>(value: S, file: &mut File) -> crate::Result<()>
where
    S: ser::Serialize,
{
    to_write(value, file, formatter::minify::MinifyFormatter)?;
    Ok(())
}

/// Serialize struct `S` as a pretty formatted JSON with comments text of the given file.
/// If you want to serialize as minified JSONC text, use [`to_file`] instead.
///
/// # Examples
/// ```
/// use serde::Serialize;
/// #[derive(Serialize)]
/// struct Product {
///     name: String,
///     price: u32,
/// }
/// let path = std::path::Path::new("tests/data/product.jsonc");
/// if path.exists() {
///     std::fs::remove_file(path).unwrap();
/// }
/// let mut file = std::fs::File::create(path).unwrap();
/// let product = Product { name: "candy".to_string(), price: 100 };
/// json_with_comments::to_file_pretty(product, &mut file, Default::default()).unwrap();
/// assert_eq!(std::fs::read_to_string(path).unwrap(), vec![
///     r#"{"#,
///     r#"  "name": "candy","#,
///     r#"  "price": 100,"#,
///     r#"}"#].join("\n"));
/// ```
pub fn to_file_pretty(
    value: impl ser::Serialize,
    file: &mut File,
    settings: formatter::pretty::PrettySettings,
) -> crate::Result<()> {
    to_write(value, file, formatter::pretty::PrettyFormatter::new(settings))?;
    Ok(())
}

/// TODO doc
pub fn to_write<W, F, S>(value: S, write: W, formatter: F) -> crate::Result<()>
where
    W: io::Write,
    F: formatter::JsoncFormatter,
    S: ser::Serialize,
{
    let mut ser = JsoncSerializer::new(write, formatter);
    value.serialize(&mut ser)
}
