use serde::de;

use crate::value::{number::Number, JsoncValue};

use super::number::FromNumber;

pub struct ValueDeserializer<I, F> {
    value: JsoncValue<I, F>,
}

impl<I, F> ValueDeserializer<I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    pub fn new(value: JsoncValue<I, F>) -> Self {
        Self { value }
    }

    pub fn deserialize_number_value<'de, V, Fn, N>(self, visitor: V, f: Fn) -> crate::Result<V::Value>
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

impl<'de, I, F> de::Deserializer<'de> for ValueDeserializer<I, F>
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
            // JsoncValue::Object(map) => visitor.visit_map(map),
            // JsoncValue::Array(vec) => visitor.visit_seq(vec),
            JsoncValue::Bool(b) => visitor.visit_bool(b),
            JsoncValue::Null => visitor.visit_none(),
            JsoncValue::String(s) => visitor.visit_string(s),
            JsoncValue::Number(n) => match n {
                Number::Integer(i) => visitor.visit_i64(i.to_i64().unwrap()), // TODO other number type
                Number::Float(f) => visitor.visit_f64(f.to_f64().unwrap()),
            },
            _ => todo!(),
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
            Some(s) => visitor.visit_str(s),
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
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
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
        todo!()
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
