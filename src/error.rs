use std::{error, fmt::Display};

use serde::de;

use crate::token::Position;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub struct Error {
    pos: Option<Position>,
    inner: Box<dyn error::Error + Send + Sync + 'static>,
}
impl Error {
    pub fn new<E: Into<Box<dyn error::Error + Send + Sync + 'static>>>(err: E) -> Self {
        Self { pos: None, inner: err.into() }
    }
    pub fn with_pos<E: Into<Box<dyn error::Error + Send + Sync + 'static>>>(pos: Position, err: E) -> Self {
        Self { pos: Some(pos), inner: err.into() }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl error::Error for Error {}
impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}
