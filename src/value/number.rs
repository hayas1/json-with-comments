use crate::de::access::number::{FromNumberBuilder, NumberBuilder};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NumberValue<I, F> {
    Integer(I),
    Float(F),
}
impl FromNumberBuilder for NumberValue<i64, f64> {
    type Err = crate::Error;
    fn from_number_builder(builder: NumberBuilder) -> Result<Self, Self::Err>
    where
        Self: std::marker::Sized,
    {
        match builder.ty() {
            NumberValue::Integer(()) => Ok(NumberValue::Integer(i64::from_number_builder(builder)?)),
            NumberValue::Float(()) => Ok(NumberValue::Float(f64::from_number_builder(builder)?)),
        }
    }
}
