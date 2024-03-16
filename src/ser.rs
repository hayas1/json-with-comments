mod access;
mod formatter;

use serde::ser;
use std::{fs::File, io, path::Path};

use self::access::jsonc::JsoncSerializer;

/// TODO doc
pub fn to_str<S>(value: S) -> crate::Result<String>
where
    S: ser::Serialize,
{
    let mut write = Vec::new();
    to_write(&mut write, formatter::minify::MinifyFormatter, value)?;
    Ok(String::from_utf8(write)?)
}

/// TODO doc
pub fn to_str_pretty<S>(value: S, settings: formatter::pretty::PrettySettings) -> crate::Result<String>
where
    S: ser::Serialize,
{
    let mut write = Vec::new();
    to_write(&mut write, formatter::pretty::PrettyFormatter::new(settings), value)?;
    Ok(String::from_utf8(write)?)
}

/// TODO doc
pub fn to_path<S>(path: &Path, value: S) -> crate::Result<()>
where
    S: ser::Serialize,
{
    let mut file = File::open(path)?;
    to_file(&mut file, value)?;
    Ok(())
}

/// TODO doc
pub fn to_path_pretty<S>(path: &Path, value: S, settings: formatter::pretty::PrettySettings) -> crate::Result<()>
where
    S: ser::Serialize,
{
    let mut file = File::open(path)?;
    to_file_pretty(&mut file, value, settings)?;
    Ok(())
}

/// TODO doc
pub fn to_file<S>(file: &mut File, value: S) -> crate::Result<()>
where
    S: ser::Serialize,
{
    to_write(file, formatter::minify::MinifyFormatter, value)?;
    Ok(())
}

/// TODO doc
pub fn to_file_pretty(
    file: &mut File,
    value: impl ser::Serialize,
    settings: formatter::pretty::PrettySettings,
) -> crate::Result<()> {
    to_write(file, formatter::pretty::PrettyFormatter::new(settings), value)?;
    Ok(())
}

/// TODO doc
pub fn to_write<W, F, S>(write: W, formatter: F, value: S) -> crate::Result<()>
where
    W: io::Write,
    F: formatter::JsoncFormatter,
    S: ser::Serialize,
{
    let mut ser = JsoncSerializer::new(write, formatter);
    value.serialize(&mut ser)
}
