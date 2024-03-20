use crate::value::number::Number;

pub trait FromNumberBuilder: Sized {
    type Err;
    fn from_number_builder(builder: NumberBuilder) -> Result<Self, Self::Err>;
}

pub enum IntegerBuilder {}
pub enum FloatBuilder {}

pub trait Builder<N> {
    type Err;
    fn build(b: NumberBuilder) -> Result<N, Self::Err>;
}
impl<N: std::str::FromStr> Builder<N> for IntegerBuilder {
    type Err = N::Err;
    fn build(b: NumberBuilder) -> Result<N, Self::Err> {
        N::from_str(&String::from_utf8_lossy(&b.buff))
    }
}
impl<N: std::str::FromStr> Builder<N> for FloatBuilder {
    type Err = N::Err;
    fn build(b: NumberBuilder) -> Result<N, Self::Err> {
        N::from_str(&String::from_utf8_lossy(&b.buff))
    }
}

pub trait Built: Sized {
    type Builder: Builder<Self>;
    fn built(b: NumberBuilder) -> Result<Self, <Self::Builder as Builder<Self>>::Err> {
        Self::Builder::build(b)
    }
}

impl<T: Built> FromNumberBuilder for T {
    type Err = <T::Builder as Builder<T>>::Err;
    fn from_number_builder(builder: NumberBuilder) -> Result<Self, Self::Err> {
        Self::built(builder)
    }
}

impl Built for u8 {
    type Builder = IntegerBuilder;
}
impl Built for u16 {
    type Builder = IntegerBuilder;
}
impl Built for u32 {
    type Builder = IntegerBuilder;
}
impl Built for u64 {
    type Builder = IntegerBuilder;
}
impl Built for u128 {
    type Builder = IntegerBuilder;
}
impl Built for i8 {
    type Builder = IntegerBuilder;
}
impl Built for i16 {
    type Builder = IntegerBuilder;
}
impl Built for i32 {
    type Builder = IntegerBuilder;
}
impl Built for i64 {
    type Builder = IntegerBuilder;
}
impl Built for i128 {
    type Builder = IntegerBuilder;
}
impl Built for f32 {
    type Builder = FloatBuilder;
}
impl Built for f64 {
    type Builder = FloatBuilder;
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
