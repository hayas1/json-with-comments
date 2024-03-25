pub mod de;
pub mod from;
pub mod index;
pub mod into;
pub mod macros;
pub mod number;
pub mod ser;

pub type MapImpl<K, V> = std::collections::HashMap<K, V>;

/// TODO doc
#[derive(Debug, Clone, PartialEq)]
// if JsoncValue<'a, I, F>, cannot implement FromStr
pub enum JsoncValue<I, F> {
    Object(MapImpl<String, JsoncValue<I, F>>),
    Array(Vec<JsoncValue<I, F>>),
    Bool(bool),
    Null,
    String(String),
    Number(number::Number<I, F>),
}

impl<I, F> Default for JsoncValue<I, F> {
    fn default() -> Self {
        Self::Null
    }
}
impl<I: num::FromPrimitive, F: num::FromPrimitive> std::str::FromStr for JsoncValue<I, F> {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::from_str(s)
    }
}
impl<I, F> JsoncValue<I, F> {
    /// TODO doc
    pub fn query(&self, query: &str) -> Option<&JsoncValue<I, F>> {
        // TODO better implement, tests
        query.split('.').try_fold(self, |value, key| match value {
            JsoncValue::Object(map) => map.get(key),
            JsoncValue::Array(vec) => vec.get(key.parse::<usize>().ok()?),
            _ => None,
        })
    }

    /// TODO doc
    pub fn take(&mut self) -> Self {
        std::mem::take(self)
    }

    /// TODO doc
    /// get the value type representation of [`JsoncValue`]
    pub fn value_type(&self) -> String {
        match self {
            JsoncValue::Object(_) => "Object",
            JsoncValue::Array(_) => "Array",
            JsoncValue::Bool(_) => "Boolean",
            JsoncValue::Null => "Null",
            JsoncValue::String(_) => "String",
            JsoncValue::Number(_) => "Number",
        }
        .to_string()
    }
}

impl<'de, I, F> JsoncValue<I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
    // I: serde::Deserialize<'de>,
    // F: serde::Deserialize<'de>,
{
    /// TODO doc
    pub fn into_deserialize<T>(&'de self) -> crate::Result<T>
    where
        T: serde::Deserialize<'de>,
    {
        T::deserialize(de::deserializer::ValueDeserializer::new(self))
    }
}

impl<I, F> JsoncValue<I, F>
where
    I: serde::Serialize,
    F: serde::Serialize,
{
    /// TODO doc
    pub fn from_serialize<T>(value: T) -> crate::Result<Self>
    where
        T: serde::Serialize,
    {
        value.serialize(ser::serializer::ValueSerializer::new())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde::Deserialize;

    use super::*;
    use crate::jsonc;

    #[test]
    fn test_from_value_number() {
        let target = jsonc!(true);
        let tru: bool = target.into_deserialize().unwrap();
        assert_eq!(tru, true);

        let target = jsonc!(1);
        let one: u8 = target.into_deserialize().unwrap();
        assert_eq!(one, 1u8);

        let target = jsonc!(0.5);
        let half: f64 = target.into_deserialize().unwrap();
        assert_eq!(half, 0.5f64);
    }

    #[test]
    fn test_from_value_string() {
        let target = jsonc!("String");
        let string: String = target.into_deserialize().unwrap();
        assert_eq!(string, "String".to_string());

        let target = jsonc!("&str");
        let str: &str = target.into_deserialize().unwrap();
        assert_eq!(str, "&str");
    }

    #[test]
    fn test_from_value_option() {
        let target = jsonc!(false);
        let fal: Option<bool> = target.into_deserialize().unwrap();
        assert_eq!(fal, Some(false));

        let target = jsonc!(null);
        let null: Option<bool> = target.into_deserialize().unwrap();
        assert_eq!(null, None);
    }

    #[test]
    fn test_from_value_seq() {
        let target = jsonc!([1, 2, 3]);
        let natural: Vec<u8> = target.into_deserialize().unwrap();
        assert_eq!(natural, vec![1, 2, 3]);

        let target = jsonc!([0, true, "two"]);
        let tuple: (i8, bool, String) = target.into_deserialize().unwrap();
        assert_eq!(tuple, (0, true, "two".to_string()));
    }

    #[test]
    fn test_from_value_map() {
        let target = jsonc!({"key": "value"});
        let map: HashMap<String, String> = target.into_deserialize().unwrap();
        assert_eq!(map, HashMap::from([("key".to_string(), "value".to_string())]));
    }

    #[test]
    fn test_struct_from_value() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Person<'a> {
            name: &'a str,
            age: Option<u32>,
        }
        let target = jsonc!({"name": "John", "age": 30});
        let person: Person = target.into_deserialize().unwrap();
        assert_eq!(person, Person { name: "John", age: Some(30) });

        let target = jsonc!([{"name": "John", "age": 30},{"name": "Jin", "age": null}]);
        let person: Vec<Person> = target.into_deserialize().unwrap();
        assert_eq!(person, [Person { name: "John", age: Some(30) }, Person { name: "Jin", age: None }]);
    }

    #[test]
    fn test_to_value() {
        let target = JsoncValue::<i64, f64>::from_serialize(true).unwrap();
        assert_eq!(target, JsoncValue::Bool(true));
    }
}
