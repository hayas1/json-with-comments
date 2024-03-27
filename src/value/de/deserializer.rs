use serde::de::{self, IgnoredAny};

use crate::value::{number::Number, JsoncValue};

use super::{map::MapDeserializer, number::FromNumber, r#enum::EnumDeserializer, seq::SeqDeserializer};

pub struct ValueDeserializer<'de, I, F> {
    pub value: &'de JsoncValue<I, F>,
}

impl<'de, I, F> ValueDeserializer<'de, I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    pub fn new(value: &'de JsoncValue<I, F>) -> Self {
        Self { value }
    }

    pub fn deserialize_number_value<V, Fn, N>(self, visitor: V, f: Fn) -> crate::Result<V::Value>
    where
        V: de::Visitor<'de>,
        N: FromNumber<I, F>,
        N::Err: de::Error,
        crate::Error: From<N::Err>,
        Fn: FnOnce(V, N) -> Result<V::Value, N::Err>,
    {
        match self.value.as_number() {
            Some(number) => Ok(f(visitor, FromNumber::from_number(number)?)?),
            _ => Err(self.invalid_type::<crate::Error>(&visitor))?,
        }
    }

    pub fn invalid_type<E: de::Error>(&self, exp: &dyn de::Expected) -> E {
        E::invalid_type(self.unexpected(), exp)
    }

    pub fn unexpected(&self) -> de::Unexpected {
        match &self.value {
            JsoncValue::Object(_) => de::Unexpected::Map,
            JsoncValue::Array(_) => de::Unexpected::Seq,
            JsoncValue::Bool(b) => de::Unexpected::Bool(*b),
            JsoncValue::Null => de::Unexpected::Unit,
            JsoncValue::String(s) => de::Unexpected::Str(s),
            JsoncValue::Number(n) => match n {
                Number::Integer(i) => match i.to_i64() {
                    Some(signed) => de::Unexpected::Signed(signed),
                    None => de::Unexpected::Other("number"),
                },
                Number::Float(f) => match f.to_f64() {
                    Some(float) => de::Unexpected::Float(float),
                    None => de::Unexpected::Other("number"),
                },
            },
        }
    }
}

impl<'de, I, F> de::Deserializer<'de> for ValueDeserializer<'de, I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            JsoncValue::Object(_) => self.deserialize_map(visitor),
            JsoncValue::Array(_) => self.deserialize_seq(visitor),
            JsoncValue::Bool(_) => self.deserialize_bool(visitor),
            JsoncValue::Null => self.deserialize_unit(visitor),
            JsoncValue::String(_) => self.deserialize_str(visitor),
            JsoncValue::Number(n) => match n {
                Number::Integer(_) => self.deserialize_i64(visitor), // TODO other number type
                Number::Float(_) => self.deserialize_f64(visitor),
            },
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value.as_bool() {
            Some(&b) => visitor.visit_bool(b),
            None => Err(self.invalid_type::<crate::Error>(&visitor))?,
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_number_value(visitor, |v, n| v.visit_i8(n))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_number_value(visitor, |v, n| v.visit_i16(n))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_number_value(visitor, |v, n| v.visit_i32(n))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_number_value(visitor, |v, n| v.visit_i64(n))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_number_value(visitor, |v, n| v.visit_u8(n))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_number_value(visitor, |v, n| v.visit_u16(n))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_number_value(visitor, |v, n| v.visit_u32(n))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_number_value(visitor, |v, n| v.visit_u64(n))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_number_value(visitor, |v, n| v.visit_f32(n))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_number_value(visitor, |v, n| v.visit_f64(n))
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
        match self.value.as_str() {
            Some(s) => visitor.visit_borrowed_str(s),
            None => Err(self.invalid_type::<crate::Error>(&visitor))?,
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
        match self.value.as_str() {
            Some(s) => visitor.visit_str(s),
            _ => Err(self.invalid_type(&visitor)),
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
        match self.value.as_null() {
            Some(()) => visitor.visit_none(),
            None => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value.as_null() {
            Some(()) => visitor.visit_unit(),
            None => Err(self.invalid_type(&visitor)),
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
        match self.value.as_array() {
            Some(v) => visitor.visit_seq(SeqDeserializer::new(v)),
            None => Err(self.invalid_type(&visitor)),
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
        match self.value.as_object() {
            Some(m) => visitor.visit_map(MapDeserializer::new(m)),
            None => Err(self.invalid_type(&visitor)),
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
        match self.value {
            JsoncValue::Array(_) => self.deserialize_seq(visitor),
            JsoncValue::Object(_) => self.deserialize_map(visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            JsoncValue::Object(m) => {
                let mut iter = m.iter();
                match (iter.next(), iter.next()) {
                    (Some((key, value)), None) => visitor.visit_enum(EnumDeserializer::new(key, Some(value))),
                    _ => Err(self.invalid_type(&visitor)),
                }
            }
            JsoncValue::String(s) => visitor.visit_enum(EnumDeserializer::<I, F>::new(s, None)),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let _ = self.deserialize_any(IgnoredAny)?;
        visitor.visit_unit()
    }
}
