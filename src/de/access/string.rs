#[derive(Debug, Clone, Hash)]
pub enum ParsedString<'a> {
    Borrowed(&'a str),
    Owned(String),
}
impl<'a> From<String> for ParsedString<'a> {
    fn from(t: String) -> Self {
        ParsedString::Owned(t)
    }
}
impl<'a> From<&'a str> for ParsedString<'a> {
    fn from(s: &'a str) -> Self {
        ParsedString::Borrowed(s)
    }
}
impl<'a> std::fmt::Display for ParsedString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsedString::Borrowed(s) => write!(f, "{}", s), // TODO escape
            ParsedString::Owned(s) => write!(f, "{}", s),
        }
    }
}
impl<'a> Eq for ParsedString<'a> {}
// TODO do not use std::fmt::Display for Eq, PartialEq
impl<'a, Rhs: std::fmt::Display> PartialEq<Rhs> for ParsedString<'a> {
    fn eq(&self, other: &Rhs) -> bool {
        match self {
            ParsedString::Borrowed(s) => *s == other.to_string(),
            ParsedString::Owned(s) => *s == other.to_string(),
        }
    }
}
