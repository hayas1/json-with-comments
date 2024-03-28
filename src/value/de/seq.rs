use serde::de;

use crate::value::JsoncValue;

use super::deserializer::ValueDeserializer;

pub struct ValueSeqDeserializer<'de, I, F> {
    iter: Box<dyn Iterator<Item = &'de JsoncValue<I, F>> + 'de>,
}

impl<'de, I, F> ValueSeqDeserializer<'de, I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    pub fn new(seq: &'de [JsoncValue<I, F>]) -> Self {
        ValueSeqDeserializer { iter: Box::new(seq.iter()) }
    }
}

impl<'de, I, F> de::SeqAccess<'de> for ValueSeqDeserializer<'de, I, F>
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
