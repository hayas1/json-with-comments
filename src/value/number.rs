#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NumberValue<I, F> {
    Integer(I),
    Float(F),
}
