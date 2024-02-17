use serde::de;
use std::fmt;
use std::{error, fmt::Display};
use thiserror::Error;

use crate::token::{PosRange, Position};

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
    #[error("Expected value, but got EOF")]
    EofWhileWhileStartParsing,

    #[error("{pos:?}: Expected value, but found {found:?}")]
    UnexpectedTokenWhileParsingValue { pos: Position, found: u8 },

    #[error("{pos:?}: Expected bool, but found {found:?}")]
    UnexpectedTokenWhileParsingBool { pos: Position, found: u8 },

    #[error("{pos:?}: Expected null, but found {found:?}")]
    UnexpectedTokenWhileParsingNull { pos: Position, found: u8 },

    #[error("Expected ident, but got EOF")]
    EofWhileParsingIdent,

    #[error("{pos:?}: Expected ident {expected:?}, but found {found:?}")]
    UnexpectedIdent { pos: PosRange, expected: Vec<u8>, found: Vec<u8> },
}
impl From<SyntaxError> for JsonWithCommentError {
    fn from(err: SyntaxError) -> Self {
        JsonWithCommentError::new(err)
    }
}
