use crate::de::access::number::{FromNumberBuilder, NumberBuilder};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Number<I, F> {
    Integer(I),
    Float(F),
}
impl FromNumberBuilder for Number<i64, f64> {
    type Err = crate::Error;
    fn from_number_builder(builder: NumberBuilder) -> Result<Self, Self::Err> {
        match builder.ty() {
            Number::Integer(()) => Ok(Number::Integer(i64::from_number_builder(builder)?)),
            Number::Float(()) => Ok(Number::Float(f64::from_number_builder(builder)?)),
        }
    }
}
