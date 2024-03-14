use serde::ser;

use crate::ser::formatter::JsoncFormatter;

use super::jsonc::JsoncSerializer;

pub struct SeqSerializer<'a, W, F>
where
    F: JsoncFormatter,
{
    serializer: &'a mut JsoncSerializer<W, F>,
    index: usize,
    len: Option<usize>,
}

impl<'a, W, F> SeqSerializer<'a, W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    pub fn start(serializer: &'a mut JsoncSerializer<W, F>, len: Option<usize>) -> crate::Result<Self> {
        serializer.formatter.write_array_start(&mut serializer.write)?;
        Ok(SeqSerializer { serializer, index: 0, len })
    }
}

impl<'a, W, F> ser::SerializeSeq for SeqSerializer<'a, W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    type Ok = ();

    type Error = crate::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.serializer.formatter.write_array_value_start(&mut self.serializer.write, self.index, self.len)?;
        value.serialize(&mut *self.serializer)?;
        self.serializer.formatter.write_array_value_end(&mut self.serializer.write, self.index, self.len)?;
        Ok(self.index += 1)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.formatter.wite_array_end(&mut self.serializer.write)
    }
}
