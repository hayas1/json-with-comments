use crate::{
    de::{access::string::StringValue, position::Position},
    error::Ensure,
};

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

    fn look(&mut self) -> crate::Result<Option<(Position, u8)>> {
        self.delegate.look()
    }

    fn parse_string_content(&mut self) -> crate::Result<StringValue<'de>> {
        let offset = self.delegate.current;
        let value = self.parse_string_content_super()?;
        match (self.unescaped, value) {
            // if string contain escape sequence, it should be unescaped
            // but unescaped string should be owned because of lifetime
            // default implementation of `Tokenizer` return always owned string
            (true, StringValue::Borrowed(_)) => Err(Ensure::OwnedString)?,
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
        behavior_fold_token(StrTokenizer::new);
    }

    #[test]
    fn test_behavior_parse_ident() {
        behavior_parse_ident(StrTokenizer::new);
    }

    #[test]
    fn test_behavior_tokenizer() {
        behavior_tokenizer(StrTokenizer::new);
    }

    #[test]
    fn test_behavior_parse_unescaped_string() {
        behavior_parse_unescaped_string(StrTokenizer::new);
    }

    #[test]
    #[should_panic]
    fn test_behavior_parse_raw_string() {
        behavior_parse_raw_string(StrTokenizer::new);
    }

    #[test]
    fn test_behavior_parse_string_err() {
        behavior_parse_string_err(StrTokenizer::new);
    }

    #[test]
    fn test_behavior_parse_number() {
        behavior_parse_number(StrTokenizer::new);
    }

    #[test]
    fn test_behavior_parse_number_err() {
        behavior_parse_number_err(StrTokenizer::new);
    }
}
