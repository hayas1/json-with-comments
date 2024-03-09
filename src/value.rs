pub mod de;
pub mod from;
pub mod index;
pub mod into;
pub mod number;

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
    Number(number::NumberValue<I, F>),
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

// TODO implement Deserializer for JsoncValue
// TODO doc
// pub fn from_value<T, I, F>(value: JsoncValue<I, F>) -> crate::Result<T>
// where
//     T: serde::de::DeserializeOwned,
//     I: num::FromPrimitive,
//     F: num::FromPrimitive,
// {
//     T::deserialize(value)
// }
