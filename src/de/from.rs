use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use serde::de;

use crate::de::{token::byte::ByteTokenizer, Deserializer};

use super::token::Tokenizer;

/// TODO doc
pub fn from_str<'de, D>(s: &'de str) -> crate::Result<D>
where
    D: de::Deserialize<'de>,
{
    from_read(BufReader::new(s.as_bytes()))
}

/// TODO doc
pub fn from_path<'de, D>(p: &'de Path) -> crate::Result<D>
where
    D: de::Deserialize<'de>,
{
    // TODO handling io error
    from_read(File::open(p).or_else(|e| Err(crate::Error::new(e.to_string())))?)
}

/// TODO doc
pub fn from_file<'de, D>(f: &'de File) -> crate::Result<D>
where
    D: de::Deserialize<'de>,
{
    from_read(f)
}

/// TODO doc
pub fn from_read<'de, R, D>(read: R) -> crate::Result<D>
where
    R: 'de + io::Read,
    D: de::Deserialize<'de>,
{
    from_tokenizer(ByteTokenizer::new(read))
}

/// TODO doc
pub fn from_tokenizer<'de, T, D>(tokenizer: T) -> crate::Result<D>
where
    T: 'de + Tokenizer,
    D: de::Deserialize<'de>,
{
    let mut de = Deserializer::new(tokenizer);
    let value = de::Deserialize::deserialize(&mut de)?;
    de.finish()?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn behavior_from_str() {
        // TODO
    }
}
