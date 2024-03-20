pub trait ToNumberRepresentation: Sized {
    type Err;
    fn to_number_representation(self) -> Result<Vec<u8>, Self::Err>;
}

pub enum IntegerRepresentor {}
pub enum FloatRepresentor {}

pub trait Representor<N> {
    type Err;
    fn represent(n: N) -> Result<Vec<u8>, Self::Err>;
}
impl<N: itoa::Integer> Representor<N> for IntegerRepresentor {
    type Err = crate::Error;
    fn represent(n: N) -> Result<Vec<u8>, Self::Err> {
        let mut buffer = itoa::Buffer::new();
        let printed = buffer.format(n);
        Ok(printed.as_bytes().to_vec())
    }
}
impl<N: ryu::Float> Representor<N> for FloatRepresentor {
    type Err = crate::Error;
    fn represent(n: N) -> Result<Vec<u8>, Self::Err> {
        let mut buffer = ryu::Buffer::new();
        let printed = buffer.format(n);
        Ok(printed.as_bytes().to_vec())
    }
}

pub trait Represent: Sized {
    type Representor: Representor<Self>;
    fn represent(self) -> Result<Vec<u8>, <Self::Representor as Representor<Self>>::Err> {
        Self::Representor::represent(self)
    }
}

impl<T: Represent> ToNumberRepresentation for T {
    type Err = <T::Representor as Representor<T>>::Err;
    fn to_number_representation(self) -> Result<Vec<u8>, Self::Err> {
        self.represent()
    }
}

impl Represent for u8 {
    type Representor = IntegerRepresentor;
}
impl Represent for u16 {
    type Representor = IntegerRepresentor;
}
impl Represent for u32 {
    type Representor = IntegerRepresentor;
}
impl Represent for u64 {
    type Representor = IntegerRepresentor;
}
impl Represent for u128 {
    type Representor = IntegerRepresentor;
}
impl Represent for i8 {
    type Representor = IntegerRepresentor;
}
impl Represent for i16 {
    type Representor = IntegerRepresentor;
}
impl Represent for i32 {
    type Representor = IntegerRepresentor;
}
impl Represent for i64 {
    type Representor = IntegerRepresentor;
}
impl Represent for i128 {
    type Representor = IntegerRepresentor;
}
impl Represent for f32 {
    type Representor = FloatRepresentor;
}
impl Represent for f64 {
    type Representor = FloatRepresentor;
}
