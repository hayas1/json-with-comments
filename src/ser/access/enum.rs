use serde::{
    ser::{self, SerializeMap, SerializeSeq},
    Serializer,
};

use crate::ser::formatter::JsoncFormatter;

use super::{jsonc::JsoncSerializer, map::MapSerialize, seq::SeqSerialize};

pub(crate) enum Delegate<S, M> {
    Seq(S),
    Map(M),
}

pub struct EnumSerialize<'a, W, F>
where
    F: JsoncFormatter,
{
    delegate: Delegate<SeqSerialize<'a, W, F>, MapSerialize<'a, W, F>>,
}

impl<'a, W, F> EnumSerialize<'a, W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    pub fn start_tuple_variant(
        serializer: &'a mut JsoncSerializer<W, F>,
        variant: &'static str,
        len: usize,
    ) -> crate::Result<Self> {
        Self::start(serializer, variant, len, Delegate::<_, ()>::Seq(()))
    }

    pub fn start_struct_variant(
        serializer: &'a mut JsoncSerializer<W, F>,
        variant: &'static str,
        len: usize,
    ) -> crate::Result<Self> {
        Self::start(serializer, variant, len, Delegate::<(), _>::Map(()))
    }

    fn start<S, M>(
        serializer: &'a mut JsoncSerializer<W, F>,
        variant: &'static str,
        len: usize,
        delegate_type: Delegate<S, M>,
    ) -> crate::Result<Self> {
        serializer.formatter.write_object_start(&mut serializer.write)?;
        serializer.formatter.write_object_key_start(&mut serializer.write, 0, Some(1))?;
        serializer.serialize_str(variant)?;
        serializer.formatter.write_object_key_end(&mut serializer.write, 0, Some(1))?;
        let delegate = match delegate_type {
            Delegate::Seq(_) => Delegate::Seq(SeqSerialize::start(serializer, Some(len))?),
            Delegate::Map(_) => Delegate::Map(MapSerialize::start(serializer, Some(len))?),
        };
        Ok(Self { delegate })
    }
}

impl<'a, W, F> ser::SerializeTupleVariant for EnumSerialize<'a, W, F>
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
        match &mut self.delegate {
            Delegate::Seq(seq) => seq.serialize_element(value),
            Delegate::Map(_) => unreachable!(),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.delegate {
            Delegate::Seq(seq) => {
                // seq.end()?; // end() cause move
                seq.serializer.formatter.write_array_end(&mut seq.serializer.write)?;
                seq.serializer.formatter.write_object_end(&mut seq.serializer.write)
            }
            Delegate::Map(_) => unreachable!(),
        }
    }
}

impl<'a, W, F> ser::SerializeStructVariant for EnumSerialize<'a, W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    type Ok = ();
    type Error = crate::Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        match &mut self.delegate {
            Delegate::Seq(_) => unreachable!(),
            Delegate::Map(map) => map.serialize_entry(key, value),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.delegate {
            Delegate::Seq(_) => unreachable!(),
            Delegate::Map(map) => {
                // map.end()?; // end() cause move
                map.serializer.formatter.write_object_end(&mut map.serializer.write)?;
                map.serializer.formatter.write_object_end(&mut map.serializer.write)
            }
        }
    }
}
