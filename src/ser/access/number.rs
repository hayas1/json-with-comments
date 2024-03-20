pub trait ToNumberRepresentation: Sized {
    type Err;
    fn to_number_representation(self) -> Result<Vec<u8>, Self::Err>;
}

pub trait ToIntegerRepresentation: Sized
where
    Self: itoa::Integer,
{
    type Err;
    fn to_integer_representation(self) -> Result<Vec<u8>, Self::Err> {
        let mut buffer = itoa::Buffer::new();
        let printed = buffer.format(self);
        Ok(printed.as_bytes().to_vec())
    }
}
impl<I: ToIntegerRepresentation> ToNumberRepresentation for I {
    type Err = I::Err;
    fn to_number_representation(self) -> Result<Vec<u8>, Self::Err> {
        self.to_integer_representation()
    }
}
impl ToIntegerRepresentation for u8 {
    type Err = crate::Error;
}
impl ToIntegerRepresentation for u16 {
    type Err = crate::Error;
}
impl ToIntegerRepresentation for u32 {
    type Err = crate::Error;
}
impl ToIntegerRepresentation for u64 {
    type Err = crate::Error;
}
impl ToIntegerRepresentation for u128 {
    type Err = crate::Error;
}
impl ToIntegerRepresentation for i8 {
    type Err = crate::Error;
}
impl ToIntegerRepresentation for i16 {
    type Err = crate::Error;
}
impl ToIntegerRepresentation for i32 {
    type Err = crate::Error;
}
impl ToIntegerRepresentation for i64 {
    type Err = crate::Error;
}
impl ToIntegerRepresentation for i128 {
    type Err = crate::Error;
}

pub trait ToFloatRepresentation: Sized
where
    Self: ryu::Float,
{
    type Err;
    fn to_float_representation(self) -> Result<Vec<u8>, Self::Err> {
        let mut buffer = ryu::Buffer::new();
        let printed = buffer.format(self);
        Ok(printed.as_bytes().to_vec())
    }
}
// impl<F: ToFloatRepresentation> ToNumberRepresentation for F {
//     type Err = F::Err;
//     fn to_number_representation(self) -> Result<Vec<u8>, Self::Err> {
//         self.to_float_representation()
//     }
// }
impl ToNumberRepresentation for f32 {
    type Err = crate::Error;
    fn to_number_representation(self) -> Result<Vec<u8>, Self::Err> {
        self.to_float_representation()
    }
}
impl ToNumberRepresentation for f64 {
    type Err = crate::Error;
    fn to_number_representation(self) -> Result<Vec<u8>, Self::Err> {
        self.to_float_representation()
    }
}
impl ToFloatRepresentation for f32 {
    type Err = crate::Error;
}
impl ToFloatRepresentation for f64 {
    type Err = crate::Error;
}
