#[derive(Debug, Clone, Hash)]
pub enum StringValue<'a> {
    Borrowed(&'a str),
    Owned(String),
}
impl<'a> From<String> for StringValue<'a> {
    fn from(t: String) -> Self {
        StringValue::Owned(t)
    }
}
impl<'a> From<&'a str> for StringValue<'a> {
    fn from(s: &'a str) -> Self {
        StringValue::Borrowed(s)
    }
}
impl<'a> std::fmt::Display for StringValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringValue::Borrowed(s) => write!(f, "{}", s), // TODO escape
            StringValue::Owned(s) => write!(f, "{}", s),
        }
    }
}
impl<'a> Eq for StringValue<'a> {}
// TODO do not use std::fmt::Display for Eq, PartialEq
impl<'a, Rhs: std::fmt::Display> PartialEq<Rhs> for StringValue<'a> {
    fn eq(&self, other: &Rhs) -> bool {
        match self {
            StringValue::Borrowed(s) => *s == other.to_string(),
            StringValue::Owned(s) => *s == other.to_string(),
        }
    }
}
