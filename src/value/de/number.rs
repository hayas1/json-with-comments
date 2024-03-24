use serde::de;

use crate::{error::ConvertError, value::number::Number};

pub trait FromNumber<I, F>: Sized
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Err;
    fn from_number(number: Number<I, F>) -> Result<Self, Self::Err>;
}

pub enum IntegerConverter {}
pub enum FloatConverter {}

pub trait Converter<I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Err;
    fn convert<N: Converted<I, F>>(n: Number<I, F>) -> Result<N, Self::Err>;
}

impl<I, F> Converter<I, F> for IntegerConverter
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Err = crate::Error;
    fn convert<N: Converted<I, F>>(n: Number<I, F>) -> Result<N, Self::Err> {
        match n {
            Number::Integer(i) => Ok(Converted::<I, F>::to_self(i).ok_or(ConvertError::InvalidIntegerConvert)?),
            Number::Float(_) => Err(ConvertError::CannotConvertFloatToInteger)?,
        }
    }
}

impl<I, F> Converter<I, F> for FloatConverter
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Err = crate::Error;
    fn convert<N: Converted<I, F>>(n: Number<I, F>) -> Result<N, Self::Err> {
        match n {
            Number::Integer(_) => Err(ConvertError::CannotConvertIntegerToFloat)?,
            Number::Float(f) => Ok(Converted::<I, F>::to_self(f).ok_or(ConvertError::InvalidIntegerConvert)?),
        }
    }
}

pub trait Converted<I, F>: Sized
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Converter: Converter<I, F>;
    fn to_self<P: num::ToPrimitive>(p: P) -> Option<Self>;
    fn converted(n: Number<I, F>) -> Result<Self, <Self::Converter as Converter<I, F>>::Err> {
        Self::Converter::convert(n)
    }
}

impl<T, I, F> FromNumber<I, F> for T
where
    T: Converted<I, F>,
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Err = <<T as Converted<I, F>>::Converter as Converter<I, F>>::Err;
    fn from_number(number: Number<I, F>) -> Result<Self, Self::Err> {
        Self::converted(number)
    }
}

impl<I, F> Converted<I, F> for u8
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Converter = IntegerConverter;
    fn to_self<P: num::ToPrimitive>(p: P) -> Option<Self> {
        p.to_u8()
    }
}

pub struct NumberDeserializer<I, F> {
    number: Number<I, F>,
}

impl<I, F> NumberDeserializer<I, F> {
    pub fn new(number: Number<I, F>) -> Self {
        Self { number }
    }
}

impl<'de, I, F> de::Deserializer<'de> for NumberDeserializer<I, F>
where
    I: de::Deserialize<'de>,
    F: de::Deserialize<'de>,
{
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
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
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
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
