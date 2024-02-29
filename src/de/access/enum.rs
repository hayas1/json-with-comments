use serde::de;

use crate::{de::token::Tokenizer, error::SyntaxError};

use super::jsonc::Deserializer;

pub struct EnumDeserializer<'de, 'a, T>
where
    T: 'a + Tokenizer<'de>,
{
    deserializer: &'a mut Deserializer<'de, T>,
}

impl<'de, 'a, T> EnumDeserializer<'de, 'a, T>
where
    T: 'a + Tokenizer<'de>,
{
    pub fn new(de: &'a mut Deserializer<'de, T>) -> Self {
        EnumDeserializer { deserializer: de }
    }
}

impl<'de, 'a, T> de::EnumAccess<'de> for EnumDeserializer<'de, 'a, T>
where
    T: 'de + Tokenizer<'de>,
{
    type Error = crate::Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let key = seed.deserialize(&mut *self.deserializer)?;
        Ok((key, self))
    }
}

impl<'de, 'a, T> de::VariantAccess<'de> for EnumDeserializer<'de, 'a, T>
where
    T: 'de + Tokenizer<'de>,
{
    type Error = crate::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.deserializer.tokenizer.eat_whitespace()?.ok_or(SyntaxError::EofWhileParsingObjectValue)? {
            (_, b':') => de::Deserialize::deserialize(self.deserializer),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingEnumValue { pos, found })?,
        }
    }

    fn newtype_variant_seed<S>(self, seed: S) -> Result<S::Value, Self::Error>
    where
        S: de::DeserializeSeed<'de>,
    {
        match self.deserializer.tokenizer.eat_whitespace()?.ok_or(SyntaxError::EofWhileParsingObjectValue)? {
            (_, b':') => seed.deserialize(self.deserializer),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingEnumValue { pos, found })?,
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.deserializer.tokenizer.eat_whitespace()?.ok_or(SyntaxError::EofWhileParsingObjectValue)? {
            (_, b':') => de::Deserializer::deserialize_seq(self.deserializer, visitor),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingEnumValue { pos, found })?,
        }
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.deserializer.tokenizer.eat_whitespace()?.ok_or(SyntaxError::EofWhileParsingObjectValue)? {
            (_, b':') => de::Deserializer::deserialize_struct(self.deserializer, "", fields, visitor),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingEnumValue { pos, found })?,
        }
    }
}
