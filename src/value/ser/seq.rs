use serde::ser;

use crate::value::JsoncValue;

use super::serializer::ValueSerializer;

pub struct SeqSerialize<I, F> {
    array: Vec<JsoncValue<I, F>>,
}

impl<I, F> SeqSerialize<I, F> {
    pub fn start(len: Option<usize>) -> Self {
        Self { array: len.map(Vec::with_capacity).unwrap_or_default() }
    }
}

impl<I, F> ser::SerializeSeq for SeqSerialize<I, F>
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    type Ok = JsoncValue<I, F>;
    type Error = crate::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Ok(self.array.push(value.serialize(ValueSerializer::new())?))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsoncValue::Array(self.array))
    }
}

impl<I, F> ser::SerializeTuple for SeqSerialize<I, F>
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    type Ok = <Self as ser::SerializeSeq>::Ok;
    type Error = <Self as ser::SerializeSeq>::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        <Self as ser::SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeSeq>::end(self)
    }
}

impl<I, F> ser::SerializeTupleStruct for SeqSerialize<I, F>
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    type Ok = <Self as ser::SerializeSeq>::Ok;
    type Error = <Self as ser::SerializeSeq>::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        <Self as ser::SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeSeq>::end(self)
    }
}
