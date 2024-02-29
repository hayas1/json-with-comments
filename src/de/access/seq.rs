use serde::de;

use crate::{
    de::token::Tokenizer,
    error::{Ensure, SyntaxError},
};

use super::jsonc::Deserializer;

pub struct SeqDeserializer<'de, 'a, T>
where
    T: 'a + Tokenizer<'de>,
{
    deserializer: &'a mut Deserializer<'de, T>,
}

impl<'de, 'a, T> SeqDeserializer<'de, 'a, T>
where
    T: 'a + Tokenizer<'de>,
{
    pub fn new(de: &'a mut Deserializer<'de, T>) -> Self {
        SeqDeserializer { deserializer: de }
    }
}

impl<'de, 'a, T> de::SeqAccess<'de> for SeqDeserializer<'de, 'a, T>
where
    T: 'de + Tokenizer<'de>,
{
    type Error = crate::Error;

    fn next_element_seed<S>(&mut self, seed: S) -> Result<Option<S::Value>, Self::Error>
    where
        S: de::DeserializeSeed<'de>,
    {
        let value =
            match self.deserializer.tokenizer.skip_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingArray)? {
                (_, b']') => Ok(None),
                _ => seed.deserialize(&mut *self.deserializer).map(Some),
            }?;
        match self.deserializer.tokenizer.skip_whitespace()?.ok_or(SyntaxError::EofWhileEndParsingArray)? {
            (_, b',') => _ = self.deserializer.tokenizer.eat()?.ok_or(Ensure::EatAfterLook)?,
            (_, b']') => (),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileParsingArrayValue { pos, found })?,
        }
        Ok(value)
    }
}
