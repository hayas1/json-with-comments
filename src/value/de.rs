use serde::{
    de::{Error as _, Visitor},
    Deserialize,
};

use crate::{
    error::{Ensure, SemanticError},
    value::{number::NumberValue, string::StringValue, JsoncValue},
};

use super::MapImpl;

impl<'de, I: num::FromPrimitive, F: num::FromPrimitive> Deserialize<'de> for JsoncValue<'de, I, F> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(JsoncValueVisitor::new())
    }
}

struct JsoncValueVisitor<I, F> {
    phantom: std::marker::PhantomData<(I, F)>,
}
impl<I, F> JsoncValueVisitor<I, F> {
    fn new() -> Self {
        Self { phantom: std::marker::PhantomData }
    }
}
impl<'de, I: num::FromPrimitive, F: num::FromPrimitive> Visitor<'de> for JsoncValueVisitor<I, F> {
    type Value = JsoncValue<'de, I, F>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("any valid JSONC value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Bool(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Number(NumberValue::Integer(I::from_i8(v).ok_or(E::custom(Ensure::CanConvertAlways))?)))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Number(NumberValue::Integer(I::from_i16(v).ok_or(E::custom(Ensure::CanConvertAlways))?)))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Number(NumberValue::Integer(I::from_i32(v).ok_or(E::custom(Ensure::CanConvertAlways))?)))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Number(NumberValue::Integer(I::from_i64(v).ok_or(E::custom(Ensure::CanConvertAlways))?)))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Number(NumberValue::Integer(I::from_i128(v).ok_or(E::custom(Ensure::CanConvertAlways))?)))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Number(NumberValue::Integer(I::from_u8(v).ok_or(E::custom(Ensure::CanConvertAlways))?)))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Number(NumberValue::Integer(I::from_u16(v).ok_or(E::custom(Ensure::CanConvertAlways))?)))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Number(NumberValue::Integer(I::from_u32(v).ok_or(E::custom(Ensure::CanConvertAlways))?)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Number(NumberValue::Integer(I::from_u64(v).ok_or(E::custom(Ensure::CanConvertAlways))?)))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Number(NumberValue::Integer(I::from_u128(v).ok_or(E::custom(Ensure::CanConvertAlways))?)))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Number(NumberValue::Integer(I::from_f32(v).ok_or(E::custom(Ensure::CanConvertAlways))?)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Number(NumberValue::Integer(I::from_f64(v).ok_or(E::custom(Ensure::CanConvertAlways))?)))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(v.encode_utf8(&mut [0u8; 4]))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::String(StringValue::Owned(v.to_string())))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::String(StringValue::Borrowed(v)))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::String(StringValue::Owned(v)))
    }

    // fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    // fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    // fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(JsoncValue::Null)
    }

    // fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut v = Vec::new();

        while let Some(elem) = seq.next_element()? {
            v.push(elem);
        }

        Ok(JsoncValue::Array(v))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut v = MapImpl::new();
        while let Some((key, value)) = map.next_entry::<JsoncValue<'_, I, F>, JsoncValue<'_, I, F>>()? {
            // TODO jsoncValue should convert Option<StringValue>
            match key {
                JsoncValue::String(s) => v.insert(s, value),
                _ => Err(A::Error::custom(SemanticError::AnyMapKey))?,
            };
        }
        Ok(JsoncValue::Object(v))
    }

    // fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
}
