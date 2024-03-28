use serde::de::{self, IntoDeserializer};

use crate::{error::Ensure, value::JsoncValue};

use super::deserializer::ValueDeserializer;

pub struct ValueEnumDeserializer<'de, I, F> {
    variant: &'de str,
    value: Option<&'de JsoncValue<I, F>>,
}

impl<'de, I, F> ValueEnumDeserializer<'de, I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    pub fn new(variant: &'de str, value: Option<&'de JsoncValue<I, F>>) -> Self {
        ValueEnumDeserializer { variant, value }
    }
}

impl<'de, I, F> de::EnumAccess<'de> for ValueEnumDeserializer<'de, I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Error = crate::Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        seed.deserialize(variant).map(|v| (v, self))
    }
}

impl<'de, I, F> de::VariantAccess<'de> for ValueEnumDeserializer<'de, I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Error = crate::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(self.value.is_none().then_some(()).ok_or(Ensure::UnitVariant)?)
    }

    fn newtype_variant_seed<S>(self, seed: S) -> Result<S::Value, Self::Error>
    where
        S: de::DeserializeSeed<'de>,
    {
        self.value.map_or(Err(de::Error::invalid_type(de::Unexpected::UnitVariant, &"newtype variant")), |v| {
            seed.deserialize(ValueDeserializer::new(v))
        })
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.value.map_or(Err(de::Error::invalid_type(de::Unexpected::UnitVariant, &"tuple variant")), |v| {
            de::Deserializer::deserialize_seq(ValueDeserializer::new(v), visitor)
        })
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.value.map_or(Err(de::Error::invalid_type(de::Unexpected::UnitVariant, &"struct variant")), |v| {
            de::Deserializer::deserialize_map(ValueDeserializer::new(v), visitor)
        })
    }
}
