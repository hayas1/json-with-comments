use std::{io, iter::Peekable};

use crate::{
    de::position::{Position, RowColIterator},
    error::Ensure,
};

use super::Tokenizer;

pub struct ReadTokenizer<R>
where
    R: io::Read,
{
    iter: Peekable<RowColIterator<io::Bytes<R>>>,
}
impl<R> ReadTokenizer<R>
where
    R: io::Read,
{
    pub fn new(read: R) -> Self {
        ReadTokenizer { iter: RowColIterator::new(read.bytes()).peekable() }
    }
}

impl<'de, R> Tokenizer<'de> for ReadTokenizer<R>
where
    R: io::Read,
{
    fn eat(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.next() {
            Some((pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(e))) => Err(e)?,
            None => Ok(None),
        }
    }

    fn look(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.peek() {
            Some(&(pos, Ok(c))) => Ok(Some((pos, c))),
            Some(&(_, Err(_))) => match self.iter.next() {
                Some((_, Err(e))) => Err(e)?,
                _ => Err(Ensure::NextAfterPeek)?,
            },
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
        behavior_fold_token(|s| ReadTokenizer::new(s.as_bytes()));
        behavior_fold_token(|s| ReadTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    fn test_behavior_parse_ident() {
        behavior_parse_ident(|s| ReadTokenizer::new(s.as_bytes()));
        behavior_parse_ident(|s| ReadTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    fn test_behavior_tokenizer() {
        behavior_tokenizer(|s| ReadTokenizer::new(s.as_bytes()));
        behavior_tokenizer(|s| ReadTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    fn test_behavior_parse_unescaped_string() {
        behavior_parse_unescaped_string(|s| ReadTokenizer::new(s.as_bytes()));
        behavior_parse_unescaped_string(|s| ReadTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    #[should_panic]
    fn test_behavior_parse_raw_string() {
        behavior_parse_raw_string(|s| ReadTokenizer::new(s.as_bytes()));
        behavior_parse_raw_string(|s| ReadTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    fn test_behavior_parse_string_err() {
        behavior_parse_string_err(|s| ReadTokenizer::new(s.as_bytes()));
        behavior_parse_string_err(|s| ReadTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    fn test_behavior_parse_number() {
        behavior_parse_number(|s| ReadTokenizer::new(s.as_bytes()));
        behavior_parse_number(|s| ReadTokenizer::new(BufReader::new(s.as_bytes())));
    }

    #[test]
    fn test_behavior_parse_number_err() {
        behavior_parse_number_err(|s| ReadTokenizer::new(s.as_bytes()));
        behavior_parse_number_err(|s| ReadTokenizer::new(BufReader::new(s.as_bytes())));
    }
}
