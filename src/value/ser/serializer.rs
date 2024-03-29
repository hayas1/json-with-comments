use serde::{ser, Serialize};

use crate::value::{JsoncValue, MapImpl};

use super::{
    map::{ValueMapKeySerializer, ValueMapSerializer},
    number::ToNumber,
    r#enum::ValueEnumSerialize,
    seq::ValueSeqSerializer,
};

pub struct ValueSerializer<I, F> {
    phantom: std::marker::PhantomData<(I, F)>,
}

impl<I, F> ValueSerializer<I, F>
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    pub fn new() -> Self {
        Self { phantom: std::marker::PhantomData }
    }

    pub fn serialize_number_value<N>(self, number: N) -> crate::Result<<Self as ser::Serializer>::Ok>
    where
        N: ToNumber<I, F>,
        crate::Error: From<N::Err>,
    {
        Ok(number.to_number().map(JsoncValue::Number)?)
    }
}

impl<I, F> ser::Serializer for ValueSerializer<I, F>
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    type Ok = JsoncValue<I, F>;
    type Error = crate::Error;
    type SerializeSeq = ValueSeqSerializer<I, F>;
    type SerializeTuple = ValueSeqSerializer<I, F>;
    type SerializeTupleStruct = ValueSeqSerializer<I, F>;
    type SerializeTupleVariant = ValueEnumSerialize<I, F>;
    type SerializeMap = ValueMapSerializer<I, F>;
    type SerializeStruct = ValueMapSerializer<I, F>;
    type SerializeStructVariant = ValueEnumSerialize<I, F>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(JsoncValue::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_number_value(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_number_value(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_number_value(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_number_value(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_number_value(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_number_value(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_number_value(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_number_value(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_number_value(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serialize_number_value(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(JsoncValue::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&String::from_utf8_lossy(v))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsoncValue::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        let key = variant.serialize(ValueMapKeySerializer)?;
        Ok(JsoncValue::Object(MapImpl::from([(key, value.serialize(self)?)])))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Self::SerializeSeq::start(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Self::SerializeTuple::start(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Self::SerializeTupleStruct::start(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Self::SerializeTupleVariant::start_tuple_variant(variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Self::SerializeMap::start(len)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Self::SerializeStructVariant::start_struct_variant(variant, len)
    }
}
