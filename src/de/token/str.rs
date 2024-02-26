use crate::{de::position::Position, error::NeverFail, value::string::StringValue};

use super::{slice::SliceTokenizer, Tokenizer};

pub struct StrTokenizer<'de> {
    delegate: SliceTokenizer<'de>,
    unescaped: bool,
}
impl<'de> StrTokenizer<'de> {
    pub fn new(s: &'de str) -> Self {
        StrTokenizer { delegate: SliceTokenizer::new(s.as_bytes()), unescaped: false }
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
        match (self.unescaped, value) {
            // if string contain escape sequence, it should be unescaped
            // but unescaped string should be owned because of lifetime
            // default implementation of `Tokenizer` return always owned string
            (true, StringValue::Borrowed(_)) => Err(NeverFail::OwnedString)?,
            (true, s @ StringValue::Owned(_)) => Ok(s),
            (false, _) => {
                let raw = &self.delegate.slice[offset..self.delegate.current];
                Ok(StringValue::Borrowed(std::str::from_utf8(raw)?))
            }
        }
    }

    fn parse_escape_sequence(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
        self.unescaped = true;
        self.delegate.parse_escape_sequence_super(buff)
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
