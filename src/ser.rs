pub mod access;
pub mod formatter;

use serde::ser;
use std::{fs::File, io, path::Path};

use crate::Value;

use self::access::jsonc::JsoncSerializer;

/// Serialize struct `S` as minified JSON with comments text.
/// If you want to serialize as pretty formatted JSONC text, use [`to_string_pretty`] instead.
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
/// let jp = json_with_comments::to_string(japan).unwrap();
/// assert_eq!(jp, r#"{"name":"Japan","code":81,"regions":["Hokkaido","Kanto","Kyushu-Okinawa"]}"#);
/// ```
pub fn to_string<S>(value: S) -> crate::Result<String>
where
    S: ser::Serialize,
{
    let mut write = Vec::new();
    to_write(value, &mut write, formatter::minify::MinifyFormatter)?;
    Ok(unsafe { String::from_utf8_unchecked(write) }) // TODO maybe safe
}

/// Serialize struct `S` as pretty formatted JSON with comments text.
/// If you want to serialize as minified JSONC text, use [`to_string`] instead.
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
/// let jp = json_with_comments::to_string_pretty(japan, Default::default()).unwrap();
/// let pretty = r#"{
///   "name": "Japan",
///   "code": 81,
///   "regions": [
///     "Hokkaido",
///     "Kanto",
///     "Kyushu-Okinawa",
///   ],
/// }"#;
/// assert_eq!(jp, pretty);
/// ```
pub fn to_string_pretty<S>(value: S, settings: formatter::pretty::PrettySettings) -> crate::Result<String>
where
    S: ser::Serialize,
{
    let mut write = Vec::new();
    to_write(value, &mut write, formatter::pretty::PrettyFormatter::new(settings))?;
    Ok(unsafe { String::from_utf8_unchecked(write) }) // TODO maybe safe
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
    to_file(value, &mut file)
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
/// let path = std::path::Path::new("tests/data/product_pretty.jsonc");
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
    to_file_pretty(value, &mut file, settings)
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
    to_write(value, file, formatter::minify::MinifyFormatter)
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
/// let path = std::path::Path::new("tests/data/product_pretty.jsonc");
/// if path.exists() {
///     std::fs::remove_file(path).unwrap();
/// }
/// let mut file = std::fs::File::create(path).unwrap();
/// let product = Product { name: "candy".to_string(), price: 100 };
/// json_with_comments::to_file_pretty(product, &mut file, Default::default()).unwrap();
/// let pretty = r#"{
///   "name": "candy",
///   "price": 100,
/// }"#;
/// assert_eq!(std::fs::read_to_string(path).unwrap(), pretty);
/// ```
pub fn to_file_pretty(
    value: impl ser::Serialize,
    file: &mut File,
    settings: formatter::pretty::PrettySettings,
) -> crate::Result<()> {
    to_write(value, file, formatter::pretty::PrettyFormatter::new(settings))
}

/// Serialize struct `S` as a JSON with comments text of the given writer.
///
/// # Examples
/// ```
/// use serde::Serialize;
/// #[derive(Serialize)]
/// struct Product {
///     name: String,
///     price: u32,
/// }
/// let mut write = Vec::new();
/// let product = Product { name: "candy".to_string(), price: 100 };
/// json_with_comments::to_write(product, &mut write, json_with_comments::MinifyFormatter).unwrap();
/// assert_eq!(String::from_utf8(write).unwrap(), r#"{"name":"candy","price":100}"#);
/// ```
pub fn to_write<W, F, S>(value: S, write: W, formatter: F) -> crate::Result<()>
where
    W: io::Write,
    F: formatter::JsoncFormatter,
    S: ser::Serialize,
{
    let mut ser = JsoncSerializer::new(write, formatter);
    value.serialize(&mut ser)
}

/// Serialize `T` to [`JsoncValue`]
///
/// # Example
/// ```
/// use serde::Serialize;
/// #[derive(Serialize)]
/// struct Product {
///     name: String,
///     price: u32,
/// }
/// let target = Product { name: "candy".to_string(), price: 100 };
/// let product = json_with_comments::to_value(target).unwrap();
/// assert_eq!(product, json_with_comments::jsonc!({ "name": "candy", "price": 100 }));
/// ```
pub fn to_value<T>(value: T) -> crate::Result<Value>
where
    T: ser::Serialize,
{
    Value::from_serialize(value)
}
#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use serde::Serialize;

    use crate::ser::{to_string, to_string_pretty};

    #[test]
    fn test_serialize_literal() {
        assert_eq!(to_string(()).unwrap(), "null");
        assert_eq!(to_string(false).unwrap(), "false");
        assert_eq!(to_string(true).unwrap(), "true");
    }

    #[test]
    fn test_serialize_string() {
        assert_eq!(to_string("str").unwrap(), r#""str""#);
        assert_eq!(to_string("string".to_string()).unwrap(), r#""string""#);

        assert_eq!(to_string("linefeed\n").unwrap(), r#""linefeed\n""#);
        assert_eq!(to_string("linefeed\u{000A}").unwrap(), r#""linefeed\n""#);
        assert_eq!(to_string("null\u{0000}").unwrap(), r#""null\u0000""#);
        assert_eq!(to_string("del\u{007f}").unwrap(), r#""del\u007F""#);
    }

    #[test]
    fn test_serialize_number() {
        assert_eq!(to_string(123).unwrap(), "123");
        assert_eq!(to_string(123.45).unwrap(), "123.45");
        assert_eq!(to_string(-119).unwrap(), "-119");
        assert_eq!(to_string(100.0).unwrap(), "100.0");
        assert_eq!(to_string(6.02214076E23).unwrap(), "6.02214076e23");
        assert_eq!(to_string(0.0000000000000001).unwrap(), "1e-16");
    }

    #[test]
    fn test_serialize_seq() {
        assert_eq!(to_string(vec![1, 2, 3]).unwrap(), "[1,2,3]");
        assert_eq!(to_string(vec!["str", "string"]).unwrap(), r#"["str","string"]"#);
        assert_eq!(to_string(vec![vec![], vec![false], vec![true, false]]).unwrap(), "[[],[false],[true,false]]");

        assert_eq!(to_string(((), true, 2)).unwrap(), "[null,true,2]");
        assert_eq!(to_string(((), true, ((), [()]))).unwrap(), "[null,true,[null,[null]]]");
        assert_eq!(to_string((false, 1, "two")).unwrap(), r#"[false,1,"two"]"#);
    }

    #[test]
    fn test_serialize_map() {
        assert_eq!(to_string(HashMap::<(), ()>::new()).unwrap(), "{}");
        assert_eq!(to_string(HashMap::from([("key", "value")])).unwrap(), r#"{"key":"value"}"#);
        assert!([
            r#"{"one":1,"two":2,"three":3}"#,
            r#"{"one":1,"three":3,"two":2}"#,
            r#"{"two":2,"one":1,"three":3}"#,
            r#"{"two":2,"three":3,"one":1}"#,
            r#"{"three":3,"one":1,"two":2}"#,
            r#"{"three":3,"two":2,"one":1}"#,
        ]
        .contains(&&to_string(HashMap::from([("one", 1), ("two", 2), ("three", 3)])).unwrap()[..]));
        assert_eq!(
            to_string(BTreeMap::from([(
                "map1",
                BTreeMap::from([("map2", BTreeMap::from([("map3", BTreeMap::from([("nest", 3)]))]))])
            )]))
            .unwrap(),
            r#"{"map1":{"map2":{"map3":{"nest":3}}}}"#
        );

        #[derive(Serialize)]
        struct Linked {
            data: i32,
            next: Option<Box<Linked>>,
        }
        assert_eq!(
            to_string(Linked {
                data: 1,
                next: Some(Box::new(Linked { data: 2, next: Some(Box::new(Linked { data: 3, next: None })) }))
            })
            .unwrap(),
            r#"{"data":1,"next":{"data":2,"next":{"data":3,"next":null}}}"#
        );
    }

    #[test]
    fn test_serialize_struct() {
        #[derive(Serialize)]
        struct UnitStruct;
        assert_eq!(to_string(UnitStruct).unwrap(), "null");

        #[derive(Serialize)]
        struct Lattice(usize, usize);
        assert_eq!(to_string(Lattice(1, 2)).unwrap(), "[1,2]");
        assert_eq!(to_string(vec![Lattice(1, 2), Lattice(3, 4)]).unwrap(), "[[1,2],[3,4]]");

        #[derive(Serialize)]
        struct Person {
            name: String,
            age: Option<u32>,
        }
        assert_eq!(
            to_string(Person { name: "Alice".to_string(), age: None }).unwrap(),
            r#"{"name":"Alice","age":null}"#
        );
        assert_eq!(to_string(Person { name: "Bob".to_string(), age: Some(42) }).unwrap(), r#"{"name":"Bob","age":42}"#);
    }

    #[test]
    fn test_serialize_enum() {
        #[derive(Serialize)]
        enum Lattice {
            D2(usize, usize),
            D3(usize, usize, usize),
        }
        assert_eq!(to_string(Lattice::D2(3, 5)).unwrap(), r#"{"D2":[3,5]}"#);
        assert_eq!(to_string(Lattice::D3(3, 5, 7)).unwrap(), r#"{"D3":[3,5,7]}"#);

        #[derive(Serialize)]
        enum Enum {
            Unit,
            Tuple(usize, isize, String),
            Struct { num: u64, text: String, bool: bool },
        }
        assert_eq!(to_string(Enum::Unit).unwrap(), r#""Unit""#);
        assert_eq!(to_string(Enum::Tuple(1, -2, "three".to_string())).unwrap(), r#"{"Tuple":[1,-2,"three"]}"#);
        assert_eq!(
            to_string(Enum::Struct { num: 1, text: "two".to_string(), bool: true }).unwrap(),
            r#"{"Struct":{"num":1,"text":"two","bool":true}}"#
        );
    }

    #[test]
    fn test_serialize_pretty() {
        assert_eq!(
            to_string_pretty(vec!["string", "string", "string", "string", "string", "string"], Default::default())
                .unwrap(),
            [
                r#"["#,
                r#"  "string","#,
                r#"  "string","#,
                r#"  "string","#,
                r#"  "string","#,
                r#"  "string","#,
                r#"  "string","#,
                r#"]"#
            ]
            .join("\n")
        );

        assert_eq!(
            to_string_pretty(
                ((), 1, "two", HashMap::from([("hoge", HashMap::from([("fuga", "piyo")]))]), vec![true, false]),
                Default::default()
            )
            .unwrap(),
            [
                r#"["#,
                r#"  null,"#,
                r#"  1,"#,
                r#"  "two","#,
                r#"  {"#,
                r#"    "hoge": {"#,
                r#"      "fuga": "piyo","#,
                r#"    },"#,
                r#"  },"#,
                r#"  ["#,
                r#"    true,"#,
                r#"    false,"#,
                r#"  ],"#,
                r#"]"#,
            ]
            .join("\n")
        )
    }
}
