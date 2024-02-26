use std::iter::Peekable;

use crate::{
    de::position::{Position, RowColIterator},
    error::NeverFail,
    value::string::StringValue,
};

use super::Tokenizer;

pub struct RawTokenizer<'de> {
    pub slice: &'de [u8],
    pub current: usize,
    iter: Peekable<RowColIterator<Box<dyn Iterator<Item = Result<u8, ()>> + 'de>>>,
}
impl<'de> RawTokenizer<'de> {
    pub fn new(slice: &'de [u8]) -> Self {
        let i: Box<dyn Iterator<Item = Result<u8, ()>> + 'de> = Box::new(slice.iter().cloned().map(Ok));
        let (current, iter) = (0, RowColIterator::new(i).peekable());
        RawTokenizer { slice, current, iter }
    }
}

impl<'de> Tokenizer<'de> for RawTokenizer<'de> {
    fn eat(&mut self) -> crate::Result<Option<(Position, u8)>> {
        self.current += 1;
        match self.iter.next() {
            Some((pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(()))) => Err(NeverFail::EmptyError)?,
            None => Ok(None),
        }
    }

    fn find(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.peek() {
            Some(&(pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(()))) => Err(NeverFail::EmptyError)?,
            None => Ok(None),
        }
    }

    fn parse_string_content(&mut self) -> crate::Result<StringValue<'de>> {
        let offset = self.current;
        let _ = self.parse_string_content_super()?;
        let raw = &self.slice[offset..self.current];
        Ok(StringValue::Borrowed(std::str::from_utf8(raw)?))
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;
    use super::*;

    #[test]
    fn test_behavior_fold_token() {
        behavior_fold_token(|s| RawTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_ident() {
        behavior_parse_ident(|s| RawTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_tokenizer() {
        behavior_tokenizer(|s| RawTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_owned_string() {
        // `RawTokenizer` cannot parse owned string that should be unescaped.
        // source data lifetime may be shorter than unescaped string that is created by Tokenizer.
        // behavior_parse_owned_string(|s| RawTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_borrowed_string() {
        behavior_parse_borrowed_string(|s| RawTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_owned_string_err() {
        behavior_parse_owned_string_err(|s| RawTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_number() {
        behavior_parse_number(|s| RawTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_number_err() {
        behavior_parse_number_err(|s| RawTokenizer::new(s.as_bytes()));
    }
}
