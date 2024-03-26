pub mod number;
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
    /// TODO doc
    pub fn from_serialize<T>(value: T) -> crate::Result<Self>
    where
        T: serde::Serialize,
    {
        value.serialize(serializer::ValueSerializer::new())
    }
}

#[cfg(test)]
mod tests {
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
    fn test_to_value_bool() {
        let target = true;
        let tru = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(tru, jsonc!(true));
    }

    #[test]
    fn test_to_value_string() {
        let target = "String".to_string();
        let string = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(string, jsonc!("String"));

        let target = "&str";
        let string = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(string, jsonc!("&str"));
    }

    #[test]
    fn test_to_value_number() {
        let target = 123u8;
        let number = JsoncValue::<i64, f64>::from_serialize(target).unwrap();
        assert_eq!(number, jsonc!(123));
    }
}
