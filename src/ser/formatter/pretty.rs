use super::JsoncFormatter;

/// TODO doc
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrettySettings {
    pub indent: Vec<u8>, // TODO &'a [u8]
    pub trailing_comma: bool,
    // pub max_width: Option<usize>, // TODO implement
}
impl Default for PrettySettings {
    fn default() -> Self {
        // Self { indent: b"  ".to_vec(), trailing_comma: true, max_width: None }
        Self { indent: b"  ".to_vec(), trailing_comma: true }
    }
}

/// TODO doc
pub struct PrettyFormatter {
    settings: PrettySettings,
    indent: usize,
}
impl PrettyFormatter {
    pub fn new(settings: PrettySettings) -> Self {
        Self { settings, indent: 0 }
    }

    pub fn should_write_trailing_comma(&self, index: usize, len: Option<usize>) -> bool {
        matches!(len.map(|l| index + 1 == l), Some(true)) && self.settings.trailing_comma
    }
}
impl JsoncFormatter for PrettyFormatter {
    fn write_array_start<W: std::io::Write>(&mut self, write: &mut W) -> crate::Result<()> {
        let sup = self.write_array_start_super(write)?;
        self.indent += 1;
        write.write_all(b"\n")?;
        Ok(sup)
    }

    fn write_array_value_start<W: std::io::Write>(
        &mut self,
        write: &mut W,
        index: usize,
        len: Option<usize>,
    ) -> crate::Result<()> {
        write.write_all(&self.settings.indent.repeat(self.indent))?;
        let sup = self.write_array_value_start_super(write, index, len)?;
        Ok(sup)
    }

    fn write_array_value_end<W: std::io::Write>(
        &mut self,
        write: &mut W,
        index: usize,
        len: Option<usize>,
    ) -> crate::Result<()> {
        let sup = self.write_array_value_end_super(write, index, len)?;
        if self.should_write_trailing_comma(index, len) {
            write.write_all(b",\n")?;
        } else {
            write.write_all(b"\n")?;
        }
        Ok(sup)
    }

    fn write_array_end<W: std::io::Write>(&mut self, write: &mut W) -> crate::Result<()> {
        self.indent -= 1;
        write.write_all(&self.settings.indent.repeat(self.indent))?;
        let sup = self.write_array_end_super(write)?;
        Ok(sup)
    }

    fn write_object_start<W: std::io::Write>(&mut self, write: &mut W) -> crate::Result<()> {
        let sup = self.write_object_start_super(write)?;
        self.indent += 1;
        write.write_all(b"\n")?;
        Ok(sup)
    }

    fn write_object_key_start<W: std::io::Write>(
        &mut self,
        write: &mut W,
        index: usize,
        len: Option<usize>,
    ) -> crate::Result<()> {
        write.write_all(&self.settings.indent.repeat(self.indent))?;
        let sup = self.write_object_key_start_super(write, index, len)?;
        Ok(sup)
    }

    fn write_object_value_start<W: std::io::Write>(
        &mut self,
        write: &mut W,
        index: usize,
        len: Option<usize>,
    ) -> crate::Result<()> {
        write.write_all(b" ")?;
        let sup = self.write_object_value_start_super(write, index, len)?;
        Ok(sup)
    }

    fn write_object_value_end<W: std::io::Write>(
        &mut self,
        write: &mut W,
        index: usize,
        len: Option<usize>,
    ) -> crate::Result<()> {
        let sup = self.write_object_value_end_super(write, index, len)?;
        if self.should_write_trailing_comma(index, len) {
            write.write_all(b",\n")?;
        } else {
            write.write_all(b"\n")?;
        }
        Ok(sup)
    }

    fn write_object_end<W: std::io::Write>(&mut self, write: &mut W) -> crate::Result<()> {
        self.indent -= 1;
        write.write_all(&self.settings.indent.repeat(self.indent))?;
        let sup = self.write_object_end_super(write)?;
        Ok(sup)
    }
}
