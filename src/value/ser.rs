pub mod r#enum;
pub mod map;
pub mod number;
pub mod seq;
pub mod serializer;

use serde::Serialize;

use super::{number::Number, JsoncValue};

impl<I: Serialize, F: Serialize> Serialize for JsoncValue<I, F> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            JsoncValue::Object(map) => map.serialize(serializer),
            JsoncValue::Array(vec) => vec.serialize(serializer),
            JsoncValue::Bool(b) => b.serialize(serializer),
            JsoncValue::Null => ().serialize(serializer),
            JsoncValue::String(s) => s.serialize(serializer),
            JsoncValue::Number(n) => match n {
                Number::Integer(i) => i.serialize(serializer),
                Number::Float(f) => f.serialize(serializer),
            },
        }
    }
}

impl<I, F> JsoncValue<I, F>
where
    I: num::FromPrimitive,
    F: num::FromPrimitive,
{
    /// Serialize a [`JsoncValue`] as type `S`.
    ///
    /// # Examples
    /// ```
    /// use serde::Serialize;
    /// #[derive(Serialize)]
    /// struct Person<'a> {
    ///     name: &'a str,
    ///     age: Option<u32>,
    /// }
    /// let target = Person { name: "John", age: Some(30) };
    /// let person = json_with_comments::Value::from_serialize(target).unwrap();
    /// assert_eq!(person, json_with_comments::jsonc!({"name": "John", "age": 30}));
    /// ```
    pub fn from_serialize<S>(value: S) -> crate::Result<Self>
    where
        S: serde::Serialize,
    {
        value.serialize(serializer::ValueSerializer::new())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde::Serialize;

    use crate::{jsonc, to_string};

    use super::JsoncValue;

    #[test]
    fn test_serialize_value() {
        let target = jsonc!({
            "obj": {
                "arr": [false, true, 2, 3],
            },
        });
        assert_eq!(to_string(target).unwrap(), r#"{"obj":{"arr":[false,true,2,3]}}"#);
    }

    #[test]
    fn test_bool_serialize_as_value() {
        let target = true;
        let tru = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(tru, jsonc!(true));
    }

    #[test]
    fn test_string_serialize_as_value() {
        let target = "String".to_string();
        let string = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(string, jsonc!("String"));

        let target = "&str";
        let string = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(string, jsonc!("&str"));
    }

    #[test]
    fn test_number_serialize_as_value() {
        let target = 123u8;
        let number = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(number, jsonc!(123));

        let target = -123;
        let number = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(number, jsonc!(-123));

        let target = 123.45f64;
        let number = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(number, jsonc!(123.45));
    }

    #[test]
    fn test_option_serialize_as_value() {
        let target = false;
        let fal = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(fal, jsonc!(false));

        let target: Option<bool> = None;
        let null = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(null, jsonc!(null));
    }

    #[test]
    fn test_seq_serialize_as_value() {
        let target = vec![1, 2, 3];
        let array = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(array, jsonc!([1, 2, 3]));

        let target = (false, 1, "two");
        let order = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(order, jsonc!([false, 1, "two"]));

        #[derive(Serialize)]
        struct Coordinate(u32, u32);
        let target = vec![Coordinate(1, 2), Coordinate(3, 4)];
        let coordinate = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(coordinate, jsonc!([[1, 2], [3, 4]]));
    }

    #[test]
    fn test_map_serialize_as_value() {
        let target = BTreeMap::from([("a", 1), ("b", 2), ("c", 3)]);
        let map = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(map, jsonc!({"a": 1, "b": 2, "c": 3}));

        #[derive(Serialize)]
        struct Street<'a> {
            name: &'a str,
            number: usize,
        }
        let target = Street { name: "Main", number: 1 };
        let street = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(street, jsonc!({"name": "Main", "number": 1}));
    }

    #[test]
    fn test_enum_serialize_as_value() {
        #[derive(Serialize)]
        enum Animal<'a> {
            Dog,
            Cat(u8),
            Fish(&'a str, u8),
            Bird { name: &'a str },
        }

        let target = Animal::Dog;
        let dog = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(dog, jsonc!("Dog"));

        let target = Animal::Cat(2);
        let cat = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(cat, jsonc!({"Cat": 2}));

        let target = Animal::Fish("Tuna", 3);
        let fish = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(fish, jsonc!({"Fish": ["Tuna", 3]}));

        let target = Animal::Bird { name: "Pigeon" };
        let bird = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(bird, jsonc!({"Bird": {"name": "Pigeon"}}));
    }
}
