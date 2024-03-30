use serde::{
    ser::{self, SerializeMap, SerializeSeq},
    Serializer,
};

use crate::{error::Ensure, ser::formatter::JsoncFormatter};

use super::{jsonc::JsoncSerializer, map::MapSerializer, seq::SeqSerializer};

pub(crate) enum Delegate<S, M> {
    Seq(S),
    Map(M),
}

pub struct EnumSerializer<'a, W, F>
where
    F: JsoncFormatter,
{
    delegate: Delegate<SeqSerializer<'a, W, F>, MapSerializer<'a, W, F>>,
}

impl<'a, W, F> EnumSerializer<'a, W, F>
where
    W: std::io::Write,
    F: JsoncFormatter,
{
    pub fn start_newtype_variant<T: ?Sized>(
        serializer: &mut JsoncSerializer<W, F>,
        variant: &'static str,
        value: &T,
    ) -> crate::Result<()>
    where
        T: ser::Serialize,
    {
        serializer.formatter.write_object_start(&mut serializer.write)?;
        serializer.formatter.write_object_key_start(&mut serializer.write, 0, Some(1))?;
        serializer.serialize_str(variant)?;
        serializer.formatter.write_object_key_end(&mut serializer.write, 0, Some(1))?;
        serializer.formatter.write_object_value_start(&mut serializer.write, 0, Some(1))?;
        value.serialize(&mut *serializer)?;
        serializer.formatter.write_object_value_end(&mut serializer.write, 0, Some(1))?;
        serializer.formatter.write_object_end(&mut serializer.write)
    }

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
            Delegate::Seq(_) => Delegate::Seq(SeqSerializer::start(serializer, Some(len))?),
            Delegate::Map(_) => Delegate::Map(MapSerializer::start(serializer, Some(len))?),
        };
        Ok(Self { delegate })
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
        match &mut self.delegate {
            Delegate::Seq(seq) => seq.serialize_element(value),
            Delegate::Map(_) => Err(Ensure::SeqLikeVariant)?,
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.delegate {
            Delegate::Seq(seq) => {
                // seq.end()?; // end() cause move
                seq.serializer.formatter.write_array_end(&mut seq.serializer.write)?;
                seq.serializer.formatter.write_object_end(&mut seq.serializer.write)
            }
            Delegate::Map(_) => Err(Ensure::SeqLikeVariant)?,
        }
    }
}

impl<'a, W, F> ser::SerializeStructVariant for EnumSerializer<'a, W, F>
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
            Delegate::Seq(_) => Err(Ensure::MapLikeVariant)?,
            Delegate::Map(map) => map.serialize_entry(key, value),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.delegate {
            Delegate::Seq(_) => Err(Ensure::MapLikeVariant)?,
            Delegate::Map(map) => {
                // map.end()?; // end() cause move
                map.serializer.formatter.write_object_end(&mut map.serializer.write)?;
                map.serializer.formatter.write_object_end(&mut map.serializer.write)
            }
        }
    }
}
