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
        Ok(n.integer().map(Number::Integer).ok_or(ConvertError::InvalidIntegerConvert)?)
    }
}

impl<I, F> Converter<I, F> for FloatConverter
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    type Err = crate::Error;
    fn convert<N: Converting<I, F>>(n: N) -> Result<Number<I, F>, Self::Err> {
        Ok(n.float().map(Number::Float).ok_or(ConvertError::InvalidFloatConvert)?)
    }
}

pub trait Converting<I, F>: Sized {
    type Converter: Converter<I, F>;
    // TODO either integer() or float() must return Some and None otherwise
    fn integer(self) -> Option<I> {
        None
    }
    fn float(self) -> Option<F> {
        None
    }
    fn converting(self) -> Result<Number<I, F>, <Self::Converter as Converter<I, F>>::Err> {
        Self::Converter::convert(self)
    }
}

impl<N, I, F> ToNumber<I, F> for N
where
    N: Converting<I, F>,
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    type Err = <<N as Converting<I, F>>::Converter as Converter<I, F>>::Err;
    fn to_number(self) -> Result<Number<I, F>, Self::Err> {
        self.converting()
    }
}

impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for u8 {
    type Converter = IntegerConverter;
    fn integer(self) -> Option<I> {
        I::from_u8(self)
    }
}
impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for u16 {
    type Converter = IntegerConverter;
    fn integer(self) -> Option<I> {
        I::from_u16(self)
    }
}
impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for u32 {
    type Converter = IntegerConverter;
    fn integer(self) -> Option<I> {
        I::from_u32(self)
    }
}
impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for u64 {
    type Converter = IntegerConverter;
    fn integer(self) -> Option<I> {
        I::from_u64(self)
    }
}
impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for u128 {
    type Converter = IntegerConverter;
    fn integer(self) -> Option<I> {
        I::from_u128(self)
    }
}
impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for i8 {
    type Converter = IntegerConverter;
    fn integer(self) -> Option<I> {
        I::from_i8(self)
    }
}
impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for i16 {
    type Converter = IntegerConverter;
    fn integer(self) -> Option<I> {
        I::from_i16(self)
    }
}
impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for i32 {
    type Converter = IntegerConverter;
    fn integer(self) -> Option<I> {
        I::from_i32(self)
    }
}
impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for i64 {
    type Converter = IntegerConverter;
    fn integer(self) -> Option<I> {
        I::from_i64(self)
    }
}
impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for i128 {
    type Converter = IntegerConverter;
    fn integer(self) -> Option<I> {
        I::from_i128(self)
    }
}
impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for f32 {
    type Converter = FloatConverter;
    fn float(self) -> Option<F> {
        F::from_f32(self)
    }
}
impl<I: num::FromPrimitive, F: num::FromPrimitive> Converting<I, F> for f64 {
    type Converter = FloatConverter;
    fn float(self) -> Option<F> {
        F::from_f64(self)
    }
}
