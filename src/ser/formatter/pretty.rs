use super::JsoncFormatter;

pub struct PrettySettings {
    pub indent: Vec<u8>, // TODO &'a [u8]
    pub trailing_comma: bool,
    pub max_width: Option<usize>,
}
impl Default for PrettySettings {
    fn default() -> Self {
        Self { indent: b"  ".to_vec(), trailing_comma: true, max_width: None }
    }
}

pub struct PrettyFormatter {
    settings: PrettySettings,
    indent: usize,
}
impl PrettyFormatter {
    pub fn new(settings: PrettySettings) -> Self {
        Self { settings, indent: 0 }
    }
}
impl JsoncFormatter for PrettyFormatter {
    fn write_array_start<W: std::io::Write>(&self, write: &mut W) -> crate::Result<()> {
        let sup = self.write_array_start_super(write)?;
        // self.indent += 1;
        write.write_all(b"\n")?;
        Ok(sup)
    }

    fn write_array_value_start<W: std::io::Write>(
        &self,
        write: &mut W,
        index: usize,
        len: Option<usize>,
    ) -> crate::Result<()> {
        write.write_all(&self.settings.indent.repeat(self.indent))?;
        let sup = self.write_array_value_start_super(write, index, len)?;
        Ok(sup)
    }

    fn write_array_value_end<W: std::io::Write>(
        &self,
        write: &mut W,
        index: usize,
        len: Option<usize>,
    ) -> crate::Result<()> {
        let sup = self.write_array_value_end_super(write, index, len)?;
        // self.indent -= 1;
        write.write_all(&self.settings.indent.repeat(self.indent))?;
        write.write_all(b"\n")?;
        Ok(sup)
    }
}
