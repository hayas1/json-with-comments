use crate::{de::position::Position, value::string::StringValue};

use super::{raw::RawTokenizer, Tokenizer};

pub struct StrTokenizer<'de> {
    delegate: RawTokenizer<'de>,
    unescaped: bool,
}
impl<'de> StrTokenizer<'de> {
    pub fn new(s: &'de str) -> Self {
        StrTokenizer { delegate: RawTokenizer::new(s.as_bytes()), unescaped: false }
    }
}

impl<'de> Tokenizer<'de> for StrTokenizer<'de> {
    fn eat(&mut self) -> crate::Result<Option<(Position, u8)>> {
        self.delegate.eat()
    }

    fn find(&mut self) -> crate::Result<Option<(Position, u8)>> {
        self.delegate.find()
    }

    fn parse_string_content(&mut self) -> crate::Result<StringValue<'de>> {
        let offset = self.delegate.current;
        let value = self.parse_string_content_super()?;
        if self.unescaped {
            Ok(value)
        } else {
            let raw = &self.delegate.slice[offset..self.delegate.current];
            Ok(StringValue::Borrowed(std::str::from_utf8(raw)?))
        }
    }

    fn parse_escape_sequence(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
        self.unescaped = true;
        self.delegate.parse_escape_sequence(buff)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;
    use super::*;

    #[test]
    fn test_behavior_fold_token() {
        behavior_fold_token(|s| StrTokenizer::new(s));
    }

    #[test]
    fn test_behavior_parse_ident() {
        behavior_parse_ident(|s| StrTokenizer::new(s));
    }

    #[test]
    fn test_behavior_tokenizer() {
        behavior_tokenizer(|s| StrTokenizer::new(s));
    }

    #[test]
    fn test_behavior_parse_owned_string() {
        behavior_parse_owned_string(|s| StrTokenizer::new(s));
    }

    #[test]
    fn test_behavior_parse_borrowed_string() {
        // TODO behavior_parse_borrowed_string(|s| StrTokenizer::new(s));
    }

    #[test]
    fn test_behavior_parse_owned_string_err() {
        behavior_parse_owned_string_err(|s| StrTokenizer::new(s));
    }

    #[test]
    fn test_behavior_parse_number() {
        behavior_parse_number(|s| StrTokenizer::new(s));
    }

    #[test]
    fn test_behavior_parse_number_err() {
        behavior_parse_number_err(|s| StrTokenizer::new(s));
    }
}
