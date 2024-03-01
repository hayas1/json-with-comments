use serde::de::{self, IgnoredAny};

use crate::{
    de::token::Tokenizer,
    error::{Ensure, SemanticError, SyntaxError},
};

use super::jsonc::JsoncDeserializer;

pub struct MapDeserializer<'de, 'a, T>
where
    T: 'a + Tokenizer<'de>,
{
    deserializer: &'a mut JsoncDeserializer<'de, T>,
}

impl<'de, 'a, T> MapDeserializer<'de, 'a, T>
where
    T: 'a + Tokenizer<'de>,
{
    pub fn new(de: &'a mut JsoncDeserializer<'de, T>) -> Self {
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
            (_, b'"') => seed.deserialize(&mut MapKeyDeserializer::new(self.deserializer)).map(Some),
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

pub struct MapKeyDeserializer<'de, 'a, T>
where
    T: 'a + Tokenizer<'de>,
{
    deserializer: &'a mut JsoncDeserializer<'de, T>,
}

impl<'de, 'a, T> MapKeyDeserializer<'de, 'a, T>
where
    T: 'a + Tokenizer<'de>,
{
    pub fn new(de: &'a mut JsoncDeserializer<'de, T>) -> Self {
        MapKeyDeserializer { deserializer: de }
    }
}

impl<'de, 'a, T> MapKeyDeserializer<'de, 'a, T>
where
    T: 'a + Tokenizer<'de>,
    &'a mut Self: de::Deserializer<'de>,
{
    pub fn double_quote<V, F>(
        &mut self,
        visitor: V,
        f: F,
    ) -> Result<V::Value, <&'a mut Self as de::Deserializer<'de>>::Error>
    where
        V: de::Visitor<'de>,
        F: FnOnce(&mut Self, V) -> Result<V::Value, <&'a mut Self as de::Deserializer<'de>>::Error>,
        <&'a mut Self as de::Deserializer<'de>>::Error: From<crate::Error>,
    {
        match self.deserializer.tokenizer.eat()?.ok_or(SyntaxError::EofWhileParsingObjectKey.into())? {
            (_, b'"') => {
                let value = f(self, visitor)?;
                match self.deserializer.tokenizer.eat()?.ok_or(SyntaxError::EofWhileParsingObjectKey.into())? {
                    (_, b'"') => Ok(value),
                    (pos, found) => Err(SyntaxError::UnexpectedTokenWhileParsingObjectKey { pos, found }.into())?,
                }
            }
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileParsingObjectKey { pos, found }.into())?,
        }
    }
}

impl<'de, 'a, T> de::Deserializer<'de> for &'a mut MapKeyDeserializer<'de, 'a, T>
where
    T: 'de + Tokenizer<'de>,
{
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.double_quote(visitor, |md, v| md.deserializer.deserialize_bool(v))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.double_quote(visitor, |md, v| md.deserializer.deserialize_i8(v))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.double_quote(visitor, |md, v| md.deserializer.deserialize_i16(v))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.double_quote(visitor, |md, v| md.deserializer.deserialize_i32(v))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.double_quote(visitor, |md, v| md.deserializer.deserialize_i64(v))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.double_quote(visitor, |md, v| md.deserializer.deserialize_u8(v))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.double_quote(visitor, |md, v| md.deserializer.deserialize_u16(v))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.double_quote(visitor, |md, v| md.deserializer.deserialize_u32(v))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.double_quote(visitor, |md, v| md.deserializer.deserialize_u64(v))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.double_quote(visitor, |md, v| md.deserializer.deserialize_f32(v))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.double_quote(visitor, |md, v| md.deserializer.deserialize_f64(v))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserializer.deserialize_char(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserializer.deserialize_str(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserializer.deserialize_string(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserializer.deserialize_bytes(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserializer.deserialize_byte_buf(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // Map key cannot be null
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.double_quote(visitor, |md, v| md.deserializer.deserialize_unit(v))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(SemanticError::AnyMapKey)?
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(SemanticError::AnyMapKey)?
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(SemanticError::AnyMapKey)?
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(SemanticError::AnyMapKey)?
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(SemanticError::AnyMapKey)?
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(SemanticError::AnyMapKey)?
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(SemanticError::AnyMapKey)?
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserializer.deserialize_identifier(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let _ = self.deserialize_any(IgnoredAny)?;
        visitor.visit_unit()
    }
}
