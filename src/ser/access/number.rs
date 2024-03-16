pub trait ToNumberRepresentation: Sized {
    type Err;
    fn to_number_representation(self) -> Result<Vec<u8>, Self::Err>;
}
impl<T: std::fmt::Display> ToNumberRepresentation for T {
    type Err = crate::Error;
    fn to_number_representation(self) -> Result<Vec<u8>, Self::Err> {
        Ok(self.to_string().into_bytes())
    }
}
