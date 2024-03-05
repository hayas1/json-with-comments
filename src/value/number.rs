#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NumberValue<I, F> {
    Integer(I),
    Float(F),
}

impl<I: std::str::FromStr, F: std::str::FromStr> std::str::FromStr for NumberValue<I, F> {
    type Err = F::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO parse as float not only as integer fails
        s.parse::<I>().map(Self::Integer).or_else(|_| s.parse::<F>().map(Self::Float))
    }
}
