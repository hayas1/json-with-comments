use std::iter::Peekable;

use crate::{
    de::position::{Position, RowColIterator},
    error::{NeverFail, SyntaxError},
    value::string::StringValue,
};

use super::Tokenizer;

pub struct SliceTokenizer<'a> {
    slice: &'a [u8],
    current: usize,
    iter: Peekable<RowColIterator<Box<dyn Iterator<Item = Result<u8, ()>> + 'a>>>,
}
impl<'a> SliceTokenizer<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        let i: Box<dyn Iterator<Item = Result<u8, ()>> + 'a> = Box::new(slice.iter().cloned().map(Ok));
        let (current, iter) = (0, RowColIterator::new(i).peekable());
        SliceTokenizer { slice, current, iter }
    }
}

impl<'a> Tokenizer for SliceTokenizer<'a> {
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

    fn parse_string<'de>(&mut self) -> crate::Result<StringValue<'de>> {
        match self.eat_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingString)? {
            (_, b'"') => {
                let offset = self.current;
                self.parse_string_content(&mut Vec::new())?; // TODO: Optimize
                match self.eat()?.ok_or(SyntaxError::EofWhileEndParsingString)? {
                    (_, b'"') => {
                        let s = &self.slice[offset..self.current];
                        Ok(StringValue::Borrowed(std::str::from_utf8(s)?))
                    }
                    (pos, found) => Err(SyntaxError::UnexpectedTokenWhileEndParsingString { pos, found })?,
                }
            }
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingString { pos, found })?,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;
    use super::*;

    #[test]
    fn test_behavior_fold_token() {
        behavior_fold_token(|s| SliceTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_ident() {
        behavior_parse_ident(|s| SliceTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_tokenizer() {
        behavior_tokenizer(|s| SliceTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_owned_string() {
        behavior_parse_owned_string(|s| SliceTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_owned_string_err() {
        behavior_parse_owned_string_err(|s| SliceTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_number() {
        behavior_parse_number(|s| SliceTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_number_err() {
        behavior_parse_number_err(|s| SliceTokenizer::new(s.as_bytes()));
    }
}
