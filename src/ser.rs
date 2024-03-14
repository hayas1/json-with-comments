mod access;
mod formatter;

use serde::ser;
use std::io;

use self::access::jsonc::JsoncSerializer;

pub fn to_str<S>(value: S) -> crate::Result<String>
where
    S: ser::Serialize,
{
    let mut write = Vec::new();
    to_write(&mut write, formatter::MinifyFormatter {}, value)?;
    Ok(String::from_utf8(write)?)
}

pub fn to_write<W, F, S>(write: W, formatter: F, value: S) -> crate::Result<()>
where
    W: io::Write,
    F: formatter::JsoncFormatter,
    S: ser::Serialize,
{
    let mut ser = JsoncSerializer::new(write, formatter);
    value.serialize(&mut ser)
}
