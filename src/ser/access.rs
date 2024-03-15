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
    use serde::Serialize;

    use crate::ser::to_str;

    #[test]
    fn test_serialize_literal() {
        assert_eq!(to_str(()).unwrap(), "null");
        assert_eq!(to_str(true).unwrap(), "true");
        assert_eq!(to_str(false).unwrap(), "false");
        assert_eq!(to_str(123).unwrap(), "123");
        assert_eq!(to_str(123.45).unwrap(), "123.45");
        assert_eq!(to_str(6.02214076E23).unwrap(), "602214076000000000000000"); // TODO

        assert_eq!(to_str("string").unwrap(), r#""string""#);
        assert_eq!(to_str("linefeed\n").unwrap(), r#""linefeed\n""#);
        assert_eq!(to_str("linefeed\u{000A}").unwrap(), r#""linefeed\n""#);
        assert_eq!(to_str("null\u{0000}").unwrap(), r#""null\u0000""#);
        assert_eq!(to_str("del\u{007f}").unwrap(), r#""del\u007F""#);
    }

    #[test]
    fn test_serialize_seq() {
        assert_eq!(to_str(vec![1, 2, 3]).unwrap(), "[1,2,3]");
        assert_eq!(to_str(vec!["str", "string"]).unwrap(), r#"["str","string"]"#);
        assert_eq!(to_str(vec![vec![], vec![false], vec![true, false]]).unwrap(), "[[],[false],[true,false]]");

        assert_eq!(to_str(((), true, 2)).unwrap(), "[null,true,2]");
        assert_eq!(to_str(((), true, ((), [()]))).unwrap(), "[null,true,[null,[null]]]");
        assert_eq!(to_str((false, 1, "two")).unwrap(), r#"[false,1,"two"]"#);

        #[derive(Serialize)]
        struct Lattice(usize, usize);
        assert_eq!(to_str(Lattice(1, 2)).unwrap(), "[1,2]");
        assert_eq!(to_str(vec![Lattice(1, 2), Lattice(3, 4)]).unwrap(), "[[1,2],[3,4]]");
    }
}
