use serde::{ser, Serialize};

use crate::{error::SemanticError, ser::formatter::JsoncFormatter};

use super::jsonc::JsoncSerializer;

pub struct MapSerializer<'a, W, F>
where
    F: JsoncFormatter,
{
    pub serializer: &'a mut JsoncSerializer<W, F>,
    index: usize,
    len: Option<usize>,
}

impl<'a, W, F> MapSerializer<'a, W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    pub fn start(serializer: &'a mut JsoncSerializer<W, F>, len: Option<usize>) -> crate::Result<Self> {
        serializer.formatter.write_object_start(&mut serializer.write)?;
        Ok(Self { serializer, index: 0, len })
    }
}

impl<'a, W, F> ser::SerializeMap for MapSerializer<'a, W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    type Ok = ();
    type Error = crate::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        self.serializer.formatter.write_object_key_start(&mut self.serializer.write, self.index, self.len)?;
        key.serialize(&mut MapKeySerializer::new(self.serializer))?;
        self.serializer.formatter.write_object_key_end(&mut self.serializer.write, self.index, self.len)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        self.serializer.formatter.write_object_value_start(&mut self.serializer.write, self.index, self.len)?;
        value.serialize(&mut *self.serializer)?;
        self.serializer.formatter.write_object_value_end(&mut self.serializer.write, self.index, self.len)?;
        Ok(self.index += 1)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.formatter.write_object_end(&mut self.serializer.write)
    }
}

impl<'a, W, F> ser::SerializeStruct for MapSerializer<'a, W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    type Ok = ();

    type Error = crate::Error;

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

pub struct MapKeySerializer<'a, W, F>
where
    F: JsoncFormatter,
{
    pub serializer: &'a mut JsoncSerializer<W, F>,
}

impl<'a, W, F> MapKeySerializer<'a, W, F>
where
    F: JsoncFormatter,
    W: std::io::Write,
{
    pub fn new(serializer: &'a mut JsoncSerializer<W, F>) -> Self {
        Self { serializer }
    }

    pub fn double_quote<Fn>(&mut self, f: Fn) -> Result<<&'a mut Self as ser::Serializer>::Ok, crate::Error>
    where
        Fn: FnOnce(
            &mut Self,
        ) -> Result<<&'a mut Self as ser::Serializer>::Ok, <&'a mut Self as ser::Serializer>::Error>,
        <&'a mut Self as ser::Serializer>::Error: From<crate::Error>,
    {
        self.serializer.write.write_all(b"\"")?;
        f(self)?;
        self.serializer.write.write_all(b"\"")?;
        Ok(())
    }
}

impl<'a, W, F> ser::Serializer for &'a mut MapKeySerializer<'a, W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    type Ok = ();
    type Error = crate::Error;
    type SerializeSeq = ser::Impossible<(), Self::Error>;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeMap = ser::Impossible<(), Self::Error>;
    type SerializeStruct = ser::Impossible<(), Self::Error>;
    type SerializeStructVariant = ser::Impossible<(), Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.double_quote(|s| v.serialize(&mut *s.serializer))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.double_quote(|s| v.serialize(&mut *s.serializer))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.double_quote(|s| v.serialize(&mut *s.serializer))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.double_quote(|s| v.serialize(&mut *s.serializer))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.double_quote(|s| v.serialize(&mut *s.serializer))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.double_quote(|s| v.serialize(&mut *s.serializer))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.double_quote(|s| v.serialize(&mut *s.serializer))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.double_quote(|s| v.serialize(&mut *s.serializer))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.double_quote(|s| v.serialize(&mut *s.serializer))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        // TODO check inf
        self.double_quote(|s| v.serialize(&mut *s.serializer))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        // TODO check inf
        self.double_quote(|s| v.serialize(&mut *s.serializer))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.double_quote(|s| v.serialize(&mut *s.serializer))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        v.serialize(&mut *self.serializer)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        v.serialize(&mut *self.serializer)
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
        self.double_quote(|s| ().serialize(&mut *s.serializer))
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
