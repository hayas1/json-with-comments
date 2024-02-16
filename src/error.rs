use std::{error, fmt::Display};

use serde::de;

#[derive(Debug)]
pub struct Error;
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
