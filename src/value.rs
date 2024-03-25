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
    pub fn into_deserialize<T>(self) -> crate::Result<T>
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
    use super::*;
    use crate::jsonc;

    #[test]
    fn test_from_value_number() {
        let v = jsonc!(true);
        let t: bool = v.into_deserialize().unwrap();
        assert_eq!(t, true);

        let one = jsonc!(1);
        let t: u8 = one.into_deserialize().unwrap();
        assert_eq!(t, 1u8);

        let half = jsonc!(0.5);
        let t: f64 = half.into_deserialize().unwrap();
        assert_eq!(t, 0.5f64);
    }

    #[test]
    fn test_from_value_string() {
        let s = jsonc!("String");
        let t: String = s.into_deserialize().unwrap();
        assert_eq!(t, "String".to_string());

        // let s = jsonc!("&str");
        // let t: &str = s.into_deserialize().unwrap();
        // assert_eq!(t, "&str");
    }

    #[test]
    fn test_to_value() {
        let t = JsoncValue::<i64, f64>::from_serialize(true).unwrap();
        assert_eq!(t, JsoncValue::Bool(true));
    }
}
