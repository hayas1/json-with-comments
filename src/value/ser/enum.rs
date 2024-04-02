use serde::{
    ser::{self, SerializeStruct, SerializeTuple},
    Serialize,
};

use crate::{
    error::Ensure,
    ser::access::r#enum::Delegate,
    value::{JsoncValue, MapImpl},
};

use super::{
    map::{ValueMapKeySerializer, ValueMapSerializer},
    seq::ValueSeqSerializer,
};

pub struct ValueEnumSerializer<I, F> {
    key: String,
    delegate: Delegate<ValueSeqSerializer<I, F>, ValueMapSerializer<I, F>>,
}

impl<I, F> ValueEnumSerializer<I, F> {
    pub fn start_newtype_variant<S: ser::Serializer, T: ?Sized>(
        serializer: S,
        variant: &'static str,
        value: &T,
    ) -> crate::Result<JsoncValue<I, F>>
    where
        JsoncValue<I, F>: From<S::Ok>,
        crate::Error: From<S::Error>,
        T: ser::Serialize,
    {
        let key = variant.serialize(ValueMapKeySerializer)?;
        Ok(JsoncValue::Object(MapImpl::from_iter([(key, value.serialize(serializer)?.into())])))
    }

    pub fn start_tuple_variant(variant: &str, len: usize) -> crate::Result<Self> {
        Self::start(variant, len, Delegate::<_, ()>::Seq(()))
    }

    pub fn start_struct_variant(variant: &str, len: usize) -> crate::Result<Self> {
        Self::start(variant, len, Delegate::<(), _>::Map(()))
    }

    fn start<S, M>(variant: &str, len: usize, delegate_type: Delegate<S, M>) -> crate::Result<Self> {
        let key = variant.serialize(ValueMapKeySerializer)?;
        let delegate = match delegate_type {
            Delegate::Seq(_) => Delegate::Seq(ValueSeqSerializer::start(Some(len))?),
            Delegate::Map(_) => Delegate::Map(ValueMapSerializer::start(Some(len))?),
        };
        Ok(Self { key, delegate })
    }
}

impl<I, F> ser::SerializeTupleVariant for ValueEnumSerializer<I, F>
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    type Ok = JsoncValue<I, F>;
    type Error = crate::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        match &mut self.delegate {
            Delegate::Seq(seq) => seq.serialize_element(value),
            Delegate::Map(_) => Err(Ensure::SeqLikeVariant)?,
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let value = match self.delegate {
            Delegate::Seq(seq) => seq.end()?,
            Delegate::Map(_) => Err(Ensure::SeqLikeVariant)?,
        };
        Ok(JsoncValue::Object(MapImpl::from_iter([(self.key, value)])))
    }
}

impl<I, F> ser::SerializeStructVariant for ValueEnumSerializer<I, F>
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    type Ok = JsoncValue<I, F>;
    type Error = crate::Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        match &mut self.delegate {
            Delegate::Seq(_) => Err(Ensure::MapLikeVariant)?,
            Delegate::Map(map) => map.serialize_field(key, value),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let value = match self.delegate {
            Delegate::Seq(_) => Err(Ensure::MapLikeVariant)?,
            Delegate::Map(map) => map.end()?,
        };
        Ok(JsoncValue::Object(MapImpl::from_iter([(self.key, value)])))
    }
}
