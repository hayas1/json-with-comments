use crate::value::number::Number;

pub trait FromNumberBuilder: Sized {
    type Err;
    fn from_number_builder(builder: NumberBuilder) -> Result<Self, Self::Err>;
}
impl<T: std::str::FromStr> FromNumberBuilder for T {
    type Err = T::Err;
    fn from_number_builder(builder: NumberBuilder) -> Result<Self, Self::Err> {
        Self::from_str(&String::from_utf8_lossy(&builder.buff))
    }
}

pub struct NumberBuilder {
    buff: Vec<u8>,
    ty: Number<(), ()>,
}
impl Default for NumberBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl NumberBuilder {
    pub fn new() -> Self {
        Self { buff: Vec::new(), ty: Number::Integer(()) }
    }

    pub fn ty(&self) -> &Number<(), ()> {
        &self.ty
    }

    pub fn build<T: FromNumberBuilder>(self) -> Result<T, T::Err> {
        T::from_number_builder(self)
    }

    pub fn push(&mut self, c: u8) {
        self.buff.push(c)
    }

    pub fn extend_from_slice(&mut self, slice: &[u8]) {
        self.buff.extend_from_slice(slice)
    }

    pub fn visit_fraction_dot(&mut self, dot: u8) {
        self.ty = Number::Float(());
        self.buff.push(dot)
    }

    pub fn visit_exponent_e(&mut self, exp: u8) {
        self.ty = Number::Float(());
        self.buff.push(exp)
    }
}
