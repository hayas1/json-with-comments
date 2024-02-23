use serde::de;
use std::fmt;
use std::{error, fmt::Display};
use thiserror::Error;

use crate::de::position::{PosRange, Position};

pub type Result<T> = std::result::Result<T, JsonWithCommentError>;
#[derive(Error, Debug)]
pub struct JsonWithCommentError {
    #[from]
    inner: Box<dyn error::Error + Send + Sync + 'static>,
}
impl JsonWithCommentError {
    pub fn new<E: Into<Box<dyn error::Error + Send + Sync + 'static>>>(err: E) -> Self {
        Self { inner: err.into() }
    }
    // TODO downcast
    pub fn into_inner(self) -> Box<dyn error::Error + Send + Sync + 'static> {
        self.inner
    }
}
impl fmt::Display for JsonWithCommentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
impl de::Error for JsonWithCommentError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("{pos:?}: Expected value, but found {found:?}")]
    UnexpectedTokenWhileParsingValue { pos: Position, found: u8 },

    #[error("{pos:?}: Expected string start `\"`, but found {found:?}")]
    UnexpectedTokenWhileStartParsingString { pos: Position, found: u8 },

    #[error("{pos:?}: Expected string end `\"`, but found {found:?}")]
    UnexpectedTokenWhileParsingString { pos: Position, found: u8 },

    #[error("{pos:?}: Expected string end `\"`, but found {found:?}")]
    UnexpectedTokenWhileEndParsingString { pos: Position, found: u8 },

    #[error("{pos:?}: Expected escape sequence start `\\`, but found {found:?}")]
    UnexpectedTokenWhileStartParsingEscapeSequence { pos: Position, found: u8 },

    #[error("{pos:?}: Expected bool, but found {found:?}")]
    UnexpectedTokenWhileParsingBool { pos: Position, found: u8 },

    #[error("{pos:?}: Expected null, but found {found:?}")]
    UnexpectedTokenWhileParsingNull { pos: Position, found: u8 },

    #[error("{pos:?}: Expected object start `{{`, but found {found:?}")]
    UnexpectedTokenWhileStartingObject { pos: Position, found: u8 },

    #[error("{pos:?}: Expected object end `}}`, but found {found:?}")]
    UnexpectedTokenWhileEndingObject { pos: Position, found: u8 },

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

    #[error("Expected value, but got EOF")]
    EofWhileStartParsingValue,

    #[error("Expected string start `\"`, but got EOF")]
    EofWhileStartParsingString,

    #[error("Expected string end `\"`, but got EOF")]
    EofWhileEndParsingString,

    #[error("Expected escape sequence starts with `\\`, but got EOF")]
    EofWhileParsingEscapeSequence,

    #[error("Expected bool, but got EOF")]
    EofWhileStartParsingBool,

    #[error("Expected null, but got EOF")]
    EofWhileStartParsingNull,

    #[error("Expected object start `{{`, but got EOF")]
    EofWhileStartParsingObject,

    #[error("Expected object end `}}`, but got EOF")]
    EofWhileEndParsingObject,

    #[error("Expected array start `[`, but got EOF")]
    EofWhileStartParsingArray,

    #[error("Expected array end `]`, but got EOF")]
    EofWhileEndParsingArray,

    #[error("Expected ident, but got EOF")]
    EofWhileParsingIdent,

    #[error("Expected object key, but got EOF")]
    EofWhileParsingObjectKey,

    #[error("Expected object value, but got EOF")]
    EofWhileParsingObjectValue,

    #[error("Expected value, but got EOF")]
    EofWhileEndParsingValue,

    #[error("{pos:?}: Expected ident {expected:?}, but found {found:?}")]
    UnexpectedIdent { pos: PosRange, expected: Vec<u8>, found: Vec<u8> },

    #[error("{pos:?}: Expected EOF, but found trailing {found:?}")]
    ExpectedEof { pos: Position, found: u8 },

    #[error("{pos:?}: control character U+{c:04X} must be escaped in string")]
    ControlCharacterWhileParsingString { pos: Position, c: u8 },

    #[error("{pos:?}: invalid escape sequence \\{found:?}")]
    InvalidEscapeSequence { pos: Position, found: u8 },
}
impl From<SyntaxError> for JsonWithCommentError {
    fn from(err: SyntaxError) -> Self {
        JsonWithCommentError::new(err)
    }
}

#[derive(Error, Debug)]
pub enum SemanticError {
    #[error("{pos:?}: Expected struct start with `{{` or `[`, but found {found:?}")]
    ExpectStruct { pos: Position, found: u8 },
}
impl From<SemanticError> for JsonWithCommentError {
    fn from(err: SemanticError) -> Self {
        JsonWithCommentError::new(err)
    }
}

#[derive(Error, Debug)]
pub enum NeverFail {
    #[error("previous peek ensure this eat does not return None")]
    EatAfterFind,
}
impl From<NeverFail> for JsonWithCommentError {
    fn from(err: NeverFail) -> Self {
        JsonWithCommentError::new(err)
    }
}
