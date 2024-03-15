use serde::{
    ser::{self, SerializeSeq},
    Serializer,
};

use crate::ser::formatter::JsoncFormatter;

use super::{jsonc::JsoncSerializer, seq::SeqSerializer};

pub struct EnumSerializer<'a, W, F>
where
    F: JsoncFormatter,
{
    seq_serializer: SeqSerializer<'a, W, F>,
    // serializer: &'a mut JsoncSerializer<W, F>,
    // index: usize,
    // len: usize,
}

impl<'a, W, F> EnumSerializer<'a, W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    pub fn start(serializer: &'a mut JsoncSerializer<W, F>, variant: &'static str, len: usize) -> crate::Result<Self> {
        serializer.formatter.write_object_start(&mut serializer.write)?;
        serializer.formatter.write_object_key_start(&mut serializer.write, 0, Some(1))?;
        serializer.serialize_str(variant)?;
        serializer.formatter.write_object_key_end(&mut serializer.write, 0, Some(1))?;
        let seq_serializer = SeqSerializer::start(serializer, Some(len))?;
        Ok(Self { seq_serializer })
    }
}

impl<'a, W, F> ser::SerializeTupleVariant for EnumSerializer<'a, W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    type Ok = ();
    type Error = crate::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        self.seq_serializer.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // self.seq_serializer.end()?; // end cause move
        self.seq_serializer.serializer.formatter.write_array_end(&mut self.seq_serializer.serializer.write)?;
        self.seq_serializer.serializer.formatter.write_object_end(&mut self.seq_serializer.serializer.write)
    }
}
