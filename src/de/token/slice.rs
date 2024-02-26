use std::iter::Peekable;

use crate::{
    de::position::{Position, RowColIterator},
    error::NeverFail,
};

use super::Tokenizer;

pub struct SliceTokenizer<'de> {
    pub slice: &'de [u8],
    pub current: usize,
    iter: Peekable<RowColIterator<Box<dyn Iterator<Item = Result<u8, ()>> + 'de>>>,
}
impl<'de> SliceTokenizer<'de> {
    pub fn new(slice: &'de [u8]) -> Self {
        let i: Box<dyn Iterator<Item = Result<u8, ()>> + 'de> = Box::new(slice.iter().cloned().map(Ok));
        let (current, iter) = (0, RowColIterator::new(i).peekable());
        SliceTokenizer { slice, current, iter }
    }
}

impl<'de> Tokenizer<'de> for SliceTokenizer<'de> {
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
    fn test_behavior_parse_borrowed_string() {
        // behavior_parse_borrowed_string(|s| SliceTokenizer::new(s.as_bytes()));
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
