use crate::{error::ConvertError, value::number::Number};

pub trait FromNumber<I, F>: Sized
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Err;
    fn from_number(number: &Number<I, F>) -> Result<Self, Self::Err>;
}

pub enum IntegerConverter {}
pub enum FloatConverter {}

pub trait Converter<I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Err;
    fn convert<N: Converted<I, F>>(n: &Number<I, F>) -> Result<N, Self::Err>;
}

impl<I, F> Converter<I, F> for IntegerConverter
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Err = crate::Error;
    fn convert<N: Converted<I, F>>(n: &Number<I, F>) -> Result<N, Self::Err> {
        match n {
            Number::Integer(i) => Ok(Converted::<I, F>::to_self(i).ok_or(ConvertError::InvalidIntegerConvert)?),
            Number::Float(_) => Err(ConvertError::CannotConvertFloatToInteger)?,
        }
    }
}

impl<I, F> Converter<I, F> for FloatConverter
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Err = crate::Error;
    fn convert<N: Converted<I, F>>(n: &Number<I, F>) -> Result<N, Self::Err> {
        match n {
            Number::Integer(_) => Err(ConvertError::CannotConvertIntegerToFloat)?,
            Number::Float(f) => Ok(Converted::<I, F>::to_self(f).ok_or(ConvertError::InvalidIntegerConvert)?),
        }
    }
}

pub trait Converted<I, F>: Sized
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Converter: Converter<I, F>;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self>;
    fn converted(n: &Number<I, F>) -> Result<Self, <Self::Converter as Converter<I, F>>::Err> {
        Self::Converter::convert(n)
    }
}

impl<N, I, F> FromNumber<I, F> for N
where
    N: Converted<I, F>,
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    type Err = <<N as Converted<I, F>>::Converter as Converter<I, F>>::Err;
    fn from_number(number: &Number<I, F>) -> Result<Self, Self::Err> {
        Self::converted(number)
    }
}

impl<I: num::ToPrimitive, F: num::ToPrimitive> Converted<I, F> for u8 {
    type Converter = IntegerConverter;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self> {
        p.to_u8()
    }
}
impl<I: num::ToPrimitive, F: num::ToPrimitive> Converted<I, F> for u16 {
    type Converter = IntegerConverter;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self> {
        p.to_u16()
    }
}
impl<I: num::ToPrimitive, F: num::ToPrimitive> Converted<I, F> for u32 {
    type Converter = IntegerConverter;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self> {
        p.to_u32()
    }
}
impl<I: num::ToPrimitive, F: num::ToPrimitive> Converted<I, F> for u64 {
    type Converter = IntegerConverter;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self> {
        p.to_u64()
    }
}
impl<I: num::ToPrimitive, F: num::ToPrimitive> Converted<I, F> for u128 {
    type Converter = IntegerConverter;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self> {
        p.to_u128()
    }
}
impl<I: num::ToPrimitive, F: num::ToPrimitive> Converted<I, F> for i8 {
    type Converter = IntegerConverter;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self> {
        p.to_i8()
    }
}
impl<I: num::ToPrimitive, F: num::ToPrimitive> Converted<I, F> for i16 {
    type Converter = IntegerConverter;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self> {
        p.to_i16()
    }
}
impl<I: num::ToPrimitive, F: num::ToPrimitive> Converted<I, F> for i32 {
    type Converter = IntegerConverter;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self> {
        p.to_i32()
    }
}
impl<I: num::ToPrimitive, F: num::ToPrimitive> Converted<I, F> for i64 {
    type Converter = IntegerConverter;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self> {
        p.to_i64()
    }
}
impl<I: num::ToPrimitive, F: num::ToPrimitive> Converted<I, F> for i128 {
    type Converter = IntegerConverter;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self> {
        p.to_i128()
    }
}
impl<I: num::ToPrimitive, F: num::ToPrimitive> Converted<I, F> for f32 {
    type Converter = FloatConverter;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self> {
        p.to_f32()
    }
}
impl<I: num::ToPrimitive, F: num::ToPrimitive> Converted<I, F> for f64 {
    type Converter = FloatConverter;
    fn to_self<P: num::ToPrimitive>(p: &P) -> Option<Self> {
        p.to_f64()
    }
}
