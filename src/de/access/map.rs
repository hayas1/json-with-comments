use serde::de;

use crate::{
    de::token::Tokenizer,
    error::{Ensure, SyntaxError},
};

use super::jsonc::Deserializer;

pub struct MapDeserializer<'de, 'a, T>
where
    T: 'a + Tokenizer<'de>,
{
    deserializer: &'a mut Deserializer<'de, T>,
}

impl<'de, 'a, T> MapDeserializer<'de, 'a, T>
where
    T: 'a + Tokenizer<'de>,
{
    pub fn new(de: &'a mut Deserializer<'de, T>) -> Self {
        MapDeserializer { deserializer: de }
    }
}

impl<'de, 'a, T> de::MapAccess<'de> for MapDeserializer<'de, 'a, T>
where
    T: 'de + Tokenizer<'de>,
{
    type Error = crate::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.deserializer.tokenizer.skip_whitespace()?.ok_or(SyntaxError::EofWhileParsingObjectKey)? {
            (_, b'"') => seed.deserialize(&mut *self.deserializer).map(Some),
            (_, b'}') => Ok(None),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileParsingObjectKey { pos, found })?,
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let value =
            match self.deserializer.tokenizer.eat_whitespace()?.ok_or(SyntaxError::EofWhileParsingObjectValue)? {
                (_, b':') => seed.deserialize(&mut *self.deserializer),
                (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingObjectValue { pos, found })?,
            }?;
        match self.deserializer.tokenizer.skip_whitespace()?.ok_or(SyntaxError::EofWhileParsingObjectValue)? {
            (_, b',') => _ = self.deserializer.tokenizer.eat()?.ok_or(Ensure::EatAfterLook)?,
            (_, b'}') => (),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileEndParsingObjectValue { pos, found })?,
        };
        Ok(value)
    }
}
