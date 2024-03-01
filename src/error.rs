use serde::de;
use std::fmt::Display;
use std::str::Utf8Error;
use std::{fmt, string::FromUtf8Error};
use thiserror::Error;

use crate::de::position::{PosRange, Position};

pub type Result<T> = std::result::Result<T, JsonWithCommentsError>;
#[derive(Error, Debug)]
pub struct JsonWithCommentsError {
    #[from]
    inner: Box<dyn std::error::Error + Send + Sync + 'static>,
}
impl JsonWithCommentsError {
    pub fn new<E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>>(err: E) -> Self {
        Self { inner: err.into() }
    }
    // TODO downcast
    pub fn into_inner(self) -> Box<dyn std::error::Error + Send + Sync + 'static> {
        self.inner
    }
}
impl fmt::Display for JsonWithCommentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
impl de::Error for JsonWithCommentsError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        JsonWithCommentsError::new(msg.to_string()) // TODO
    }
}

impl From<std::io::Error> for JsonWithCommentsError {
    fn from(value: std::io::Error) -> Self {
        JsonWithCommentsError::new(value)
    }
}
impl From<FromUtf8Error> for JsonWithCommentsError {
    fn from(value: FromUtf8Error) -> Self {
        JsonWithCommentsError::new(value)
    }
}
impl From<Utf8Error> for JsonWithCommentsError {
    fn from(value: Utf8Error) -> Self {
        JsonWithCommentsError::new(value)
    }
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("{pos:?}: Expected value, but found {found:?}")]
    UnexpectedTokenWhileParsingValue { pos: Position, found: u8 },

    #[error("{pos:?}: Expected string start `\"`, but found {found:?}")]
    UnexpectedTokenWhileStartParsingString { pos: Position, found: u8 },

    #[error("{pos:?}: Expected bytes start, but found {found:?}")]
    UnexpectedTokenWhileStartParsingBytes { pos: Position, found: u8 },

    #[error("{pos:?}: Expected string end `\"`, but found {found:?}")]
    UnexpectedTokenWhileParsingString { pos: Position, found: u8 },

    #[error("{pos:?}: Expected string end `\"`, but found {found:?}")]
    UnexpectedTokenWhileEndParsingString { pos: Position, found: u8 },

    #[error("{pos:?}: Expected escape sequence start `\\`, but found {found:?}")]
    UnexpectedTokenWhileStartParsingEscapeSequence { pos: Position, found: u8 },

    #[error("{pos:?}: Expected number start `-` or 0-9 , but found {found:?}")]
    UnexpectedTokenWhileStartParsingNumber { pos: Position, found: u8 },

    #[error("{pos:?}: Expected bool, but found {found:?}")]
    UnexpectedTokenWhileParsingBool { pos: Position, found: u8 },

    #[error("{pos:?}: Expected null, but found {found:?}")]
    UnexpectedTokenWhileParsingNull { pos: Position, found: u8 },

    #[error("{pos:?}: Expected object start `{{`, but found {found:?}")]
    UnexpectedTokenWhileStartParsingObject { pos: Position, found: u8 },

    #[error("{pos:?}: Expected object end `}}`, but found {found:?}")]
    UnexpectedTokenWhileEndParsingObject { pos: Position, found: u8 },

    #[error("{pos:?}: Expected object key, but found {found:?}")]
    UnexpectedTokenWhileParsingObjectKey { pos: Position, found: u8 },

    #[error("{pos:?}: Expected object value start `:`, but found {found:?}")]
    UnexpectedTokenWhileStartParsingObjectValue { pos: Position, found: u8 },

    #[error("{pos:?}: Expected object value end `,` or `}}`, but found {found:?}")]
    UnexpectedTokenWhileEndParsingObjectValue { pos: Position, found: u8 },

    #[error("{pos:?}: Expected array start `[`, but found {found:?}")]
    UnexpectedTokenWhileStartParsingArray { pos: Position, found: u8 },

    #[error("{pos:?}: Expected array end `]`, but found {found:?}")]
    UnexpectedTokenWhileEndParsingArray { pos: Position, found: u8 },

    #[error("{pos:?}: Expected array value, but found {found:?}")]
    UnexpectedTokenWhileParsingArrayValue { pos: Position, found: u8 },

    #[error("{pos:?}: Expected enum start `{{`, but found {found:?}")]
    UnexpectedTokenWhileStartParsingEnum { pos: Position, found: u8 },

    #[error("{pos:?}: Expected enum end `}}`, but found {found:?}")]
    UnexpectedTokenWhileEndParsingEnum { pos: Position, found: u8 },

    #[error("{pos:?}: Expected enum value start `:`, but found {found:?}")]
    UnexpectedTokenWhileStartParsingEnumValue { pos: Position, found: u8 },

    #[error("{pos:?}: Expected comment start `//` or `/*`, but found {found:?}")]
    UnexpectedTokenWhileStartParsingComment { pos: Position, found: u8 },

    #[error("{pos:?}: Expected comment end `*/`, but found {found:?}")]
    UnexpectedTokenWhileEndParsingComment { pos: Position, found: u8 },

    #[error("Expected value, but got EOF")]
    EofWhileStartParsingValue,

    #[error("Expected string start `\"`, but got EOF")]
    EofWhileStartParsingString,

    #[error("Expected string end `\"`, but got EOF")]
    EofWhileEndParsingString,

    #[error("Expected bytes start, but got EOF")]
    EofWhileStartParsingBytes,

    #[error("Expected escape sequence starts with `\\`, but got EOF")]
    EofWhileParsingEscapeSequence,

    #[error("Expected number start, but got EOF")]
    EofWhileStartParsingNumber,

    #[error("Expected fraction, but got EOF")]
    EofWhileStartParsingFraction,

    #[error("Expected exponent, but got EOF")]
    EofWhileStartParsingExponent,

    #[error("Expected number, but got EOF")]
    EofWhileParsingNumber,

    #[error("Expected bool, but got EOF")]
    EofWhileStartParsingBool,

    #[error("Expected null, but got EOF")]
    EofWhileStartParsingNull,

    #[error("Expected object start `{{`, but got EOF")]
    EofWhileStartParsingObject,

    #[error("Expected object end `}}`, but got EOF")]
    EofWhileEndParsingObject,

    #[error("Expected object key, but got EOF")]
    EofWhileParsingObjectKey,

    #[error("Expected object value, but got EOF")]
    EofWhileParsingObjectValue,

    #[error("Expected array start `[`, but got EOF")]
    EofWhileStartParsingArray,

    #[error("Expected array end `]`, but got EOF")]
    EofWhileEndParsingArray,

    #[error("Expected ident, but got EOF")]
    EofWhileParsingIdent,

    #[error("Expected enum start `{{`, but got EOF")]
    EofWhileStartParsingEnum,

    #[error("Expected enum end `}}`, but got EOF")]
    EofWhileEndParsingEnum,

    #[error("Expected start comment `//` or `/*`, but got EOF")]
    EofWhileStartParsingComment,

    #[error("{pos:?}: Expected ident {expected:?}, but found {found:?}")]
    UnexpectedIdent { pos: PosRange, expected: Vec<u8>, found: Vec<u8> },

    #[error("{pos:?}: Expected EOF, but found trailing {found:?}")]
    ExpectedEof { pos: Position, found: u8 },

    #[error("{pos:?}: control character U+{c:04X} must be escaped in string")]
    ControlCharacterWhileParsingString { pos: Position, c: u8 },

    #[error("{pos:?}: invalid escape sequence \\{found:?}")]
    InvalidEscapeSequence { pos: Position, found: u8 },

    #[error("{pos:?}: invalid \\uXXXX escape, cannot parse {found:?} as hex digit")]
    InvalidUnicodeEscape { pos: Position, found: u8 },

    #[error("{pos:?}: cannot convert {char:08X} to char")]
    CannotConvertChar { pos: Position, char: u32 },

    #[error("{pos:?}: JSON with comments number does not start from `+`")]
    InvalidLeadingPlus { pos: Position },

    #[error("{pos:?}: JSON with comments number is forbidden leading `0`")]
    InvalidLeadingZeros { pos: Position },

    #[error("{pos:?}: expect exponent part, but found {found:?}")]
    MissingExponent { pos: Position, found: u8 },

    #[error("{pos:?}: expect fraction part, but found {found:?}")]
    MissingFraction { pos: Position, found: u8 },

    #[error("{pos:?}: cannot convert {rep:?} to number")]
    InvalidNumber { pos: Position, rep: String },

    #[error("comment starts with `/*` must be ends with `*/`, but got EoF")]
    UnterminatedComment,
}
impl From<SyntaxError> for JsonWithCommentsError {
    fn from(err: SyntaxError) -> Self {
        JsonWithCommentsError::new(err)
    }
}

#[derive(Error, Debug)]
pub enum SemanticError {
    #[error("{pos:?}: Expected struct start with `{{` or `[`, but found {found:?}")]
    ExpectStruct { pos: Position, found: u8 },

    #[error("map key of JSON with comments must be string")]
    AnyMapKey,

    #[error("JSON with comments must not be empty")]
    EmptyJsonWithComment,
}
impl From<SemanticError> for JsonWithCommentsError {
    fn from(err: SemanticError) -> Self {
        JsonWithCommentsError::new(err)
    }
}

#[derive(Error, Debug)]
pub enum Ensure {
    #[error("next should return peeked value")]
    NextAfterPeek,

    #[error("previous peek ensure this eat does not return None")]
    EatAfterLook,

    #[error("returns Result for interface reasons, but does not actually fail")]
    EmptyError,

    #[error("unescaped string should be owned because of lifetime")]
    OwnedString,
}
impl From<Ensure> for JsonWithCommentsError {
    fn from(err: Ensure) -> Self {
        JsonWithCommentsError::new(err)
    }
}
