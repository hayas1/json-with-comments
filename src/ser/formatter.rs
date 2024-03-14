use super::access::number::ToNumberRepresentation;

pub trait JsoncFormatter {
    fn write_bool<W: std::io::Write>(&self, write: &mut W, value: bool) -> crate::Result<()> {
        Ok(write.write_all(if value { b"true" } else { b"false" })?)
    }

    fn write_null<W: std::io::Write>(&self, write: &mut W) -> crate::Result<()> {
        Ok(write.write_all(b"null")?)
    }

    fn write_number<W: std::io::Write, N: ToNumberRepresentation>(&self, write: &mut W, value: N) -> crate::Result<()>
    where
        crate::Error: From<N::Err>,
    {
        Ok(write.write_all(&value.to_number_representation()?)?)
    }

    fn write_str<W: std::io::Write>(&self, write: &mut W, value: &str) -> crate::Result<()> {
        todo!("escape"); // TODO
        Ok(write.write_all(value.as_bytes())?)
    }

    fn write_array_start<W: std::io::Write>(&self, write: &mut W) -> crate::Result<()> {
        Ok(write.write_all(b"[")?)
    }

    fn write_array_value_start<W: std::io::Write>(
        &self,
        _write: &mut W,
        _index: usize,
        _len: Option<usize>,
    ) -> crate::Result<()> {
        Ok(())
    }

    fn write_array_value_end<W: std::io::Write>(
        &self,
        write: &mut W,
        index: usize,
        len: Option<usize>,
    ) -> crate::Result<()> {
        match len.map(|l| index + 1 < l) {
            Some(true) => Ok(write.write_all(b",")?),
            _ => Ok(()),
        }
    }

    fn wite_array_end<W: std::io::Write>(&self, write: &mut W) -> crate::Result<()> {
        Ok(write.write_all(b"]")?)
    }
}

pub struct MinifyFormatter {}
impl JsoncFormatter for MinifyFormatter {}

pub struct PrettyFormatter {}
impl JsoncFormatter for PrettyFormatter {}
