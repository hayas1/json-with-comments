use serde::de;

use crate::value::JsoncValue;

use super::deserializer::ValueDeserializer;

pub struct SeqDeserializer<'de, I, F> {
    iter: std::slice::Iter<'de, JsoncValue<I, F>>,
}

impl<'de, I, F> SeqDeserializer<'de, I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    pub fn new(iter: std::slice::Iter<'de, JsoncValue<I, F>>) -> Self {
        SeqDeserializer { iter }
    }
}

impl<'de, I, F> de::SeqAccess<'de> for SeqDeserializer<'de, I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Error = crate::Error;

    fn next_element_seed<S>(&mut self, seed: S) -> Result<Option<S::Value>, Self::Error>
    where
        S: de::DeserializeSeed<'de>,
    {
        self.iter.next().map_or(Ok(None), |v| seed.deserialize(ValueDeserializer::new(v)).map(Some))
    }

    fn size_hint(&self) -> Option<usize> {
        self.iter.size_hint().1
    }
}
