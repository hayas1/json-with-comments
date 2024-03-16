pub mod minify;
pub mod pretty;

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
        write.write_all(b"\"")?;
        for &b in value.as_bytes() {
            match b {
                b'"' => write.write_all(br#"""#)?,
                b'\\' => write.write_all(br"\\")?,
                b'/' => write.write_all(br"/")?,
                b'\x08' => write.write_all(br"\b")?,
                b'\x0C' => write.write_all(br"\f")?,
                b'\n' => write.write_all(br"\n")?,
                b'\r' => write.write_all(br"\r")?,
                b'\t' => write.write_all(br"\t")?,
                b @ (b'\x00'..=b'\x1F' | b'\x7F' | b'\x80'..=b'\x9F') => {
                    let (big, little) = (b >> 4, b & 0x0F);
                    let bb = if big < 10 { b'0' + big } else { b'A' + big - 10 };
                    let lb = if little < 10 { b'0' + little } else { b'A' + little - 10 };
                    write.write_all(&[b'\\', b'u', b'0', b'0', bb, lb])?
                }
                b => write.write_all(&[b])?,
            };
        }
        Ok(write.write_all(b"\"")?)
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
    fn write_array_end<W: std::io::Write>(&self, write: &mut W) -> crate::Result<()> {
        Ok(write.write_all(b"]")?)
    }

    fn write_object_start<W: std::io::Write>(&self, write: &mut W) -> crate::Result<()> {
        Ok(write.write_all(b"{")?)
    }
    fn write_object_key_start<W: std::io::Write>(
        &self,
        _write: &mut W,
        _index: usize,
        _len: Option<usize>,
    ) -> crate::Result<()> {
        Ok(())
    }
    fn write_object_key_end<W: std::io::Write>(
        &self,
        write: &mut W,
        _index: usize,
        _len: Option<usize>,
    ) -> crate::Result<()> {
        Ok(write.write_all(b":")?)
    }
    fn write_object_value_start<W: std::io::Write>(
        &self,
        _write: &mut W,
        _index: usize,
        _len: Option<usize>,
    ) -> crate::Result<()> {
        Ok(())
    }
    fn write_object_value_end<W: std::io::Write>(
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
    fn write_object_end<W: std::io::Write>(&self, write: &mut W) -> crate::Result<()> {
        Ok(write.write_all(b"}")?)
    }
}
