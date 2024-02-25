use std::{io, iter::Peekable};

use crate::de::position::{Position, RowColIterator};

use super::Tokenizer;

pub struct ByteTokenizer<R>
where
    R: io::Read,
{
    iter: Peekable<RowColIterator<io::Bytes<R>>>,
}
impl<R> ByteTokenizer<R>
where
    R: io::Read,
{
    pub fn new(read: R) -> Self {
        ByteTokenizer { iter: RowColIterator::new(read.bytes()).peekable() }
    }
}

impl<'de, R> Tokenizer<'de> for ByteTokenizer<R>
where
    R: io::Read,
{
    fn eat(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.next() {
            Some((pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(e))) => Err(crate::Error::new(e.to_string())), // TODO handling io error
            None => Ok(None),
        }
    }

    fn find(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.peek() {
            Some(&(pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(e))) => Err(crate::Error::new(e.to_string())), // TODO handling io error
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::super::tests::*;
    use super::*;

    #[test]
    fn test_behavior_fold_token() {
        behavior_fold_token(|s| ByteTokenizer::new(s.as_bytes()));
        behavior_fold_token(|s| ByteTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    fn test_behavior_parse_ident() {
        behavior_parse_ident(|s| ByteTokenizer::new(s.as_bytes()));
        behavior_parse_ident(|s| ByteTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    fn test_behavior_tokenizer() {
        behavior_tokenizer(|s| ByteTokenizer::new(s.as_bytes()));
        behavior_tokenizer(|s| ByteTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    fn test_behavior_parse_owned_string() {
        behavior_parse_owned_string(|s| ByteTokenizer::new(s.as_bytes()));
        behavior_parse_owned_string(|s| ByteTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    fn test_behavior_parse_owned_string_err() {
        behavior_parse_owned_string_err(|s| ByteTokenizer::new(s.as_bytes()));
        behavior_parse_owned_string_err(|s| ByteTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    fn test_behavior_parse_number() {
        behavior_parse_number(|s| ByteTokenizer::new(s.as_bytes()));
        behavior_parse_number(|s| ByteTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    fn test_behavior_parse_number_err() {
        behavior_parse_number_err(|s| ByteTokenizer::new(s.as_bytes()));
        behavior_parse_number_err(|s| ByteTokenizer::new(BufReader::new(s.as_bytes())));
    }
}
