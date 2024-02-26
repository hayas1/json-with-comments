use crate::{de::position::Position, value::string::StringValue};

use super::{slice::SliceTokenizer, Tokenizer};

pub struct RawTokenizer<'de> {
    delegate: SliceTokenizer<'de>,
}
impl<'de> RawTokenizer<'de> {
    pub fn new(slice: &'de [u8]) -> Self {
        RawTokenizer { delegate: SliceTokenizer::new(slice) }
    }
}

impl<'de> Tokenizer<'de> for RawTokenizer<'de> {
    fn eat(&mut self) -> crate::Result<Option<(Position, u8)>> {
        self.delegate.eat()
    }

    fn find(&mut self) -> crate::Result<Option<(Position, u8)>> {
        self.delegate.find()
    }

    fn parse_string_content(&mut self) -> crate::Result<StringValue<'de>> {
        let offset = self.delegate.current;
        let _ = self.parse_string_content_super()?;
        let raw = &self.delegate.slice[offset..self.delegate.current];
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
    #[should_panic]
    fn test_behavior_parse_unescaped_string() {
        behavior_parse_unescaped_string(|s| RawTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_raw_string() {
        behavior_parse_raw_string(|s| RawTokenizer::new(s.as_bytes()));
    }

    #[test]
    fn test_behavior_parse_string_err() {
        behavior_parse_string_err(|s| RawTokenizer::new(s.as_bytes()));
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
