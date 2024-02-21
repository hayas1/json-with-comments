pub mod position;
pub mod token;

use std::io;

use serde::de;

use crate::error::SyntaxError;

use self::token::Tokenizer;

pub struct Deserializer<R>
where
    R: io::Read,
{
    tokenizer: Tokenizer<R>,
}
impl<R> Deserializer<R>
where
    R: io::Read,
{
    pub fn new(tokenizer: Tokenizer<R>) -> Self {
        Deserializer { tokenizer }
    }
}
impl<'de, 'a, R> de::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: io::Read,
{
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tokenizer.skip_whitespace().and(Err(SyntaxError::EofWhileStartParsingValue)?)? {
            (_, b'n') => self.deserialize_unit(visitor),
            (_, b'f' | b't') => self.deserialize_bool(visitor),
            (_, b'-' | b'0'..=b'9') => todo!("u64, i64, f64 and so on..."),
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
        match self.tokenizer.skip_whitespace().and(Err(SyntaxError::EofWhileStartParsingBool)?)? {
            (_, b't') => visitor.visit_bool(self.tokenizer.parse_ident(b"true", true)?),
            (_, b'f') => visitor.visit_bool(self.tokenizer.parse_ident(b"false", false)?),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileParsingBool { pos, found })?,
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
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
        match self.tokenizer.skip_whitespace().and(Err(SyntaxError::EofWhileStartParsingString)?)? {
            (_, b'"') => visitor.visit_str(&self.tokenizer.parse_str()?),
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
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tokenizer.skip_whitespace().and(Err(SyntaxError::EofWhileStartParsingNull)?)? {
            (_, b'n') => {
                self.tokenizer.parse_ident(b"null", ())?;
                visitor.visit_unit()
            }
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileParsingNull { pos, found })?,
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tokenizer.eat_whitespace().and(Err(SyntaxError::EofWhileStartParsingObject)?)? {
            (_, b'{') => {
                let map = visitor.visit_map(MapDeserializer::new(self))?;
                match self.tokenizer.eat_whitespace().and(Err(SyntaxError::EofWhileEndParsingObject)?)? {
                    (_, b'}') => Ok(map),
                    (pos, found) => Err(SyntaxError::UnexpectedTokenWhileEndingObject { pos, found })?,
                }
            }
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartingObject { pos, found })?,
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
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
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}

struct MapDeserializer<'a, R: 'a>
where
    R: io::Read,
{
    deserializer: &'a mut Deserializer<R>,
}

impl<'a, R: 'a> MapDeserializer<'a, R>
where
    R: io::Read,
{
    fn new(de: &'a mut Deserializer<R>) -> Self {
        MapDeserializer { deserializer: de }
    }
}

impl<'de, 'a, R: io::Read> de::MapAccess<'de> for MapDeserializer<'a, R>
where
    R: io::Read + 'a,
{
    type Error = crate::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        todo!()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
