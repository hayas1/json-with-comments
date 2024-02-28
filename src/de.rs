pub mod from;
pub mod position;
pub mod token;

use serde::de;

use crate::{
    error::{Ensure, SyntaxError},
    value::string::StringValue,
};

use self::token::Tokenizer;

pub struct Deserializer<'de, T>
where
    T: Tokenizer<'de>,
{
    tokenizer: T,
    phantom: std::marker::PhantomData<&'de ()>,
}
impl<'de, T> Deserializer<'de, T>
where
    T: Tokenizer<'de>,
{
    pub fn new(tokenizer: T) -> Self {
        Deserializer { tokenizer, phantom: std::marker::PhantomData }
    }

    pub fn finish(&mut self) -> crate::Result<()> {
        match self.tokenizer.eat_whitespace()? {
            Some((pos, found)) => Err(SyntaxError::ExpectedEof { pos, found })?,
            None => Ok(()),
        }
    }
}
impl<'de, 'a, T> de::Deserializer<'de> for &'a mut Deserializer<'de, T>
where
    T: 'de + Tokenizer<'de>,
{
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tokenizer.skip_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingValue)? {
            (_, b'n') => self.deserialize_unit(visitor),
            (_, b'f' | b't') => self.deserialize_bool(visitor),
            (_, b'-' | b'0'..=b'9') => todo!("u64, i64, f64 and so on..."), // TODO number
            (_, b'"') => self.deserialize_str(visitor),
            (_, b'[') => self.deserialize_seq(visitor),
            (_, b'{') => self.deserialize_map(visitor),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileParsingValue { pos, found })?,
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tokenizer.skip_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingBool)? {
            (_, b't') => self.tokenizer.parse_ident(b"true", visitor.visit_bool(true))?,
            (_, b'f') => self.tokenizer.parse_ident(b"false", visitor.visit_bool(false))?,
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileParsingBool { pos, found })?,
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.tokenizer.parse_number()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.tokenizer.parse_number()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.tokenizer.parse_number()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.tokenizer.parse_number()?)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i128(self.tokenizer.parse_number()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.tokenizer.parse_number()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.tokenizer.parse_number()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.tokenizer.parse_number()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.tokenizer.parse_number()?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u128(self.tokenizer.parse_number()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.tokenizer.parse_number()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.tokenizer.parse_number()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tokenizer.skip_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingString)? {
            (_, b'"') => match self.tokenizer.parse_string()? {
                StringValue::Borrowed(s) => visitor.visit_borrowed_str(s),
                StringValue::Owned(s) => visitor.visit_str(&s),
            },
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingString { pos, found })?,
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tokenizer.skip_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingBytes)? {
            (_, b'"') => visitor.visit_bytes(self.tokenizer.parse_string()?.to_string().as_bytes()), // TODO directly convert to bytes
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingBytes { pos, found })?,
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tokenizer.skip_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingValue)? {
            (_, b'n') => self.tokenizer.parse_ident(b"null", visitor.visit_unit())?,
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tokenizer.skip_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingNull)? {
            (_, b'n') => self.tokenizer.parse_ident(b"null", visitor.visit_unit())?,
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileParsingNull { pos, found })?,
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tokenizer.eat_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingArray)? {
            (_, b'[') => {
                let seq = visitor.visit_seq(SeqDeserializer::new(self))?;
                match self.tokenizer.eat_whitespace()?.ok_or(SyntaxError::EofWhileEndParsingArray)? {
                    (_, b']') => Ok(seq),
                    (pos, found) => Err(SyntaxError::UnexpectedTokenWhileEndParsingArray { pos, found })?,
                }
            }
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingArray { pos, found })?,
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tokenizer.eat_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingObject)? {
            (_, b'{') => {
                let object = visitor.visit_map(MapDeserializer::new(self))?;
                match self.tokenizer.eat_whitespace()?.ok_or(SyntaxError::EofWhileEndParsingObject)? {
                    (_, b'}') => Ok(object),
                    (pos, found) => Err(SyntaxError::UnexpectedTokenWhileEndingObject { pos, found })?,
                }
            }
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartingObject { pos, found })?,
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tokenizer.skip_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingValue)? {
            (_, b'{') => self.deserialize_map(visitor),
            (_, b'[') => self.deserialize_seq(visitor),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartingObject { pos, found })?,
        }
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
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}

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
    fn new(de: &'a mut Deserializer<'de, T>) -> Self {
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
            (_, b',') => _ = self.deserializer.tokenizer.eat()?.ok_or(Ensure::EatAfterFind)?,
            (_, b'}') => (),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileEndParsingObjectValue { pos, found })?,
        };
        Ok(value)
    }
}

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
    fn new(de: &'a mut Deserializer<'de, T>) -> Self {
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
            (_, b',') => _ = self.deserializer.tokenizer.eat()?.ok_or(Ensure::EatAfterFind)?,
            (_, b']') => (),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileParsingArrayValue { pos, found })?,
        }
        Ok(value)
    }
}
