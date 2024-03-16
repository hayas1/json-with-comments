use crate::ser::formatter::JsoncFormatter;

use serde::ser;

use super::{map::MapSerializer, r#enum::EnumSerializer, seq::SeqSerializer};

pub struct JsoncSerializer<W, F>
where
    F: JsoncFormatter,
{
    pub write: W,
    pub formatter: F,
}

impl<W, F> JsoncSerializer<W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    pub fn new(write: W, formatter: F) -> Self {
        JsoncSerializer { write, formatter }
    }
}

impl<'a, W, F> ser::Serializer for &'a mut JsoncSerializer<W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    type Ok = ();

    type Error = crate::Error;

    type SerializeSeq = SeqSerializer<'a, W, F>;
    type SerializeTuple = SeqSerializer<'a, W, F>;
    type SerializeTupleStruct = SeqSerializer<'a, W, F>;
    type SerializeTupleVariant = EnumSerializer<'a, W, F>;
    type SerializeMap = MapSerializer<'a, W, F>;
    type SerializeStruct = MapSerializer<'a, W, F>;
    type SerializeStructVariant = EnumSerializer<'a, W, F>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_bool(&mut self.write, v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_number(&mut self.write, v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_number(&mut self.write, v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_number(&mut self.write, v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_number(&mut self.write, v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_number(&mut self.write, v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_number(&mut self.write, v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_number(&mut self.write, v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_number(&mut self.write, v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_number(&mut self.write, v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_number(&mut self.write, v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_str(&mut self.write, &v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_str(&mut self.write, v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_str(&mut self.write, &String::from_utf8_lossy(v))
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
        self.formatter.write_null(&mut self.write)
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
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Self::SerializeSeq::start(self, len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Self::SerializeTupleVariant::start_tuple_variant(self, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Self::SerializeMap::start(self, len)
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
        Self::SerializeStructVariant::start_struct_variant(self, variant, len)
    }
}
