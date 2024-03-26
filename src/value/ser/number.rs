use crate::{error::ConvertError, value::number::Number};

pub trait ToNumber<I, F>: Sized {
    type Err;
    fn to_number(self) -> Result<Number<I, F>, Self::Err>;
}

pub enum IntegerConverter {}
pub enum FloatConverter {}

pub trait Converter<I, F> {
    type Err;
    fn convert<N: Converting<I, F>>(n: N) -> Result<Number<I, F>, Self::Err>;
}

impl<I, F> Converter<I, F> for IntegerConverter
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    type Err = crate::Error;
    fn convert<N: Converting<I, F>>(n: N) -> Result<Number<I, F>, Self::Err> {
        match N::from_self(n) {
            Some(i) => Ok(i),
            None => Err(ConvertError::InvalidIntegerConvert)?,
        }
    }
}

pub trait Converting<I, F>: Sized {
    type Converter: Converter<I, F>;
    fn from_self(p: Self) -> Option<Number<I, F>>;
    fn converting(self) -> Result<Number<I, F>, <Self::Converter as Converter<I, F>>::Err> {
        Self::Converter::convert(self)
    }
}

impl<T, I, F> ToNumber<I, F> for T
where
    T: Converting<I, F>,
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    type Err = <<T as Converting<I, F>>::Converter as Converter<I, F>>::Err;
    fn to_number(self) -> Result<Number<I, F>, Self::Err> {
        self.converting()
    }
}

impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for u8 {
    type Converter = IntegerConverter;
    fn from_self(p: u8) -> Option<Number<I, F>> {
        Some(Number::Integer(I::from_u8(p).unwrap()))
    }
}
