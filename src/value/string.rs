#[derive(Debug, Clone)]
pub enum StringValue<'a> {
    Borrowed(&'a str),
    Owned(String),
}
impl<'a, T: Into<String>> From<T> for StringValue<'a> {
    fn from(t: T) -> Self {
        StringValue::Owned(t.into())
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
