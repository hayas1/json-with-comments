use serde::de;

use crate::{
    de::access::number::{FromNumberBuilder, NumberBuilder},
    error::Ensure,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Number<I, F> {
    Integer(I),
    Float(F),
}

impl<I, F> FromNumberBuilder for Number<I, F>
where
    I: FromNumberBuilder,
    F: FromNumberBuilder,
    crate::Error: From<I::Err> + From<F::Err>,
{
    type Err = crate::Error;
    fn from_number_builder(builder: NumberBuilder) -> Result<Self, Self::Err> {
        match builder.ty() {
            Number::Integer(()) => Ok(Number::Integer(I::from_number_builder(builder)?)),
            Number::Float(()) => Ok(Number::Float(F::from_number_builder(builder)?)),
        }
    }
}

impl<'de, I, F> serde::Deserialize<'de> for Number<I, F>
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO another file
        struct NumberVisitor<I, F>(std::marker::PhantomData<(I, F)>);
        impl<'de, I, F> serde::de::Visitor<'de> for NumberVisitor<I, F>
        where
            I: num::FromPrimitive,
            F: num::FromPrimitive,
        {
            type Value = Number<I, F>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a JSONC number")
            }
            // TODO other number type
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Number::Integer(I::from_i64(v).ok_or(E::custom(Ensure::CanConvertAlways))?))
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Number::Float(F::from_f64(v).ok_or(E::custom(Ensure::CanConvertAlways))?))
            }
        }
        deserializer.deserialize_any(NumberVisitor(std::marker::PhantomData))
    }
}

// TODO other number type
// impl<I, F> serde::Serialize for Number<I, F> {
impl serde::Serialize for Number<i64, f64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Number::Integer(i) => serializer.serialize_i64(i),
            Number::Float(f) => serializer.serialize_f64(f),
        }
    }
}
