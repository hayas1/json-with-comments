use std::io::{self, BufReader};

use serde::de;

use crate::de::{token::Tokenizer, Deserializer};

/// TODO doc
pub fn from_str<'de, T>(s: &'de str) -> crate::Result<T>
where
    T: de::Deserialize<'de>,
{
    from_read(BufReader::new(s.as_bytes()))
}

/// TODO doc
pub fn from_read<'de, R, T>(read: R) -> crate::Result<T>
where
    R: io::Read,
    T: de::Deserialize<'de>,
{
    let mut de = Deserializer::new(Tokenizer::new(read));
    let value = de::Deserialize::deserialize(&mut de)?;
    de.end()?;

    Ok(value)
}
