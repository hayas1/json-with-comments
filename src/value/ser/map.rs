use serde::ser;

use crate::{
    error::{Ensure, SemanticError},
    to_string,
    value::{JsoncValue, MapImpl},
};

use super::serializer::ValueSerializer;

pub struct ValueMapSerializer<I, F> {
    object: MapImpl<String, JsoncValue<I, F>>,
    key: Option<String>,
}

impl<I, F> ValueMapSerializer<I, F> {
    pub fn start(len: Option<usize>) -> crate::Result<Self> {
        Ok(Self { object: len.map(MapImpl::with_capacity).unwrap_or_default(), key: None })
    }
}

impl<I, F> ser::SerializeMap for ValueMapSerializer<I, F>
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    type Ok = JsoncValue<I, F>;
    type Error = crate::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Ok(self.key = Some(key.serialize(ValueMapKeySerializer)?))
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let v = value.serialize(ValueSerializer::new())?;
        self.key.take().map(|k| self.object.insert(k, v)).ok_or(Ensure::NextValue)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsoncValue::Object(self.object))
    }
}

impl<I, F> ser::SerializeStruct for ValueMapSerializer<I, F>
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    type Ok = <Self as ser::SerializeMap>::Ok;
    type Error = <Self as ser::SerializeMap>::Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        <Self as ser::SerializeMap>::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeMap>::end(self)
    }
}

pub struct ValueMapKeySerializer;

impl ser::Serializer for ValueMapKeySerializer {
    type Ok = String;
    type Error = crate::Error;

    type SerializeSeq = ser::Impossible<String, Self::Error>;
    type SerializeTuple = ser::Impossible<String, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<String, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<String, Self::Error>;
    type SerializeMap = ser::Impossible<String, Self::Error>;
    type SerializeStruct = ser::Impossible<String, Self::Error>;
    type SerializeStructVariant = ser::Impossible<String, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        to_string(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        to_string(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        to_string(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        to_string(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        to_string(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        to_string(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        to_string(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        to_string(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        to_string(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        // TODO check inf
        to_string(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        // TODO check inf
        to_string(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(String::from_utf8_lossy(v).to_string())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SemanticError::AnyMapKey)?
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        to_string(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SemanticError::AnyMapKey)?
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SemanticError::AnyMapKey)?
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        Err(SemanticError::AnyMapKey)?
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        Err(SemanticError::AnyMapKey)?
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(SemanticError::AnyMapKey)?
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SemanticError::AnyMapKey)?
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SemanticError::AnyMapKey)?
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SemanticError::AnyMapKey)?
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SemanticError::AnyMapKey)?
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(SemanticError::AnyMapKey)?
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SemanticError::AnyMapKey)?
    }
}
