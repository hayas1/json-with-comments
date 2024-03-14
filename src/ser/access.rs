pub mod jsonc;
pub mod number;
pub mod seq;

use serde::ser;
pub struct Temp {}

impl ser::SerializeTuple for Temp {
    type Ok = ();

    type Error = crate::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl ser::SerializeTupleStruct for Temp {
    type Ok = ();

    type Error = crate::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl ser::SerializeTupleVariant for Temp {
    type Ok = ();

    type Error = crate::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl ser::SerializeMap for Temp {
    type Ok = ();

    type Error = crate::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl ser::SerializeStruct for Temp {
    type Ok = ();

    type Error = crate::Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl ser::SerializeStructVariant for Temp {
    type Ok = ();

    type Error = crate::Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::ser::to_str;

    #[test]
    fn test_serialize_vec() {
        assert_eq!(to_str(vec![1, 2, 3]).unwrap(), "[1,2,3]");
        // assert_eq!(to_str(vec!["str", "string"]).unwrap(), r#"["str","string"]"#);
        assert_eq!(to_str(vec![vec![], vec![false], vec![true, false]]).unwrap(), "[[],[false],[true,false]]");
    }
}
