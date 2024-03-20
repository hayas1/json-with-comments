use crate::de::access::number::{FromNumberBuilder, NumberBuilder};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Number<I, F> {
    Integer(I),
    Float(F),
}

impl<I, F> FromNumberBuilder for Number<I, F>
where
    I: FromNumberBuilder,
    F: FromNumberBuilder,
    crate::Error: From<I::Err> + From<F::Err>,
{
    type Err = crate::Error;
    fn from_number_builder(builder: NumberBuilder) -> Result<Self, Self::Err> {
        match builder.ty() {
            Number::Integer(()) => Ok(Number::Integer(I::from_number_builder(builder)?)),
            Number::Float(()) => Ok(Number::Float(F::from_number_builder(builder)?)),
        }
    }
}
