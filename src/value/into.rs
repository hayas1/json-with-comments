use crate::error::InvalidRepresentsValue;

use super::{number::NumberValue, JsoncValue, MapImpl};

impl<I, F> JsoncValue<I, F> {
    pub fn is_object(&self) -> bool {
        matches!(self, JsoncValue::Object(_))
    }
    pub fn as_object(&self) -> Option<&MapImpl<String, Self>> {
        match self {
            JsoncValue::Object(m) => Some(m),
            _ => None,
        }
    }
    pub fn as_object_mut(&mut self) -> Option<&mut MapImpl<String, Self>> {
        match self {
            JsoncValue::Object(m) => Some(m),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(self, JsoncValue::Array(_))
    }
    pub fn as_array(&self) -> Option<&Vec<Self>> {
        match self {
            JsoncValue::Array(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            JsoncValue::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, JsoncValue::Bool(_))
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            &JsoncValue::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, JsoncValue::Null)
    }
    pub fn as_null(&self) -> Option<()> {
        match self {
            JsoncValue::Null => Some(()),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, JsoncValue::String(_))
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsoncValue::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, JsoncValue::Number(_))
    }
    pub fn as_number(&self) -> Option<&NumberValue<I, F>> {
        match self {
            JsoncValue::Number(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, JsoncValue::Number(NumberValue::Integer(_)))
    }
    pub fn as_integer(&self) -> Option<&I> {
        match self {
            JsoncValue::Number(NumberValue::Integer(i)) => Some(i),
            _ => None,
        }
    }

    pub fn is_float(&self) -> bool {
        matches!(self, JsoncValue::Number(NumberValue::Float(_)))
    }
    pub fn as_float(&self) -> Option<&F> {
        match self {
            JsoncValue::Number(NumberValue::Float(f)) => Some(f),
            _ => None,
        }
    }
}

impl<I, F> TryFrom<JsoncValue<I, F>> for MapImpl<String, JsoncValue<I, F>> {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<I, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Object(m) => Ok(m),
            _ => Err(InvalidRepresentsValue::ShouldObject)?,
        }
    }
}
impl<I, F> TryFrom<JsoncValue<I, F>> for Vec<JsoncValue<I, F>> {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<I, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Array(v) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldArray)?,
        }
    }
}
impl<I, F> TryFrom<JsoncValue<I, F>> for bool {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<I, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Bool(v) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldBool)?,
        }
    }
}
impl<I, F> TryFrom<JsoncValue<I, F>> for () {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<I, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Null => Ok(()),
            _ => Err(InvalidRepresentsValue::ShouldNull)?,
        }
    }
}
impl<I, F> TryFrom<JsoncValue<I, F>> for String {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<I, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::String(v) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldString)?,
        }
    }
}
impl<I, F> TryFrom<JsoncValue<I, F>> for NumberValue<I, F> {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<I, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(v) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
// TODO implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
// impl<I, F> TryFrom<JsoncValue<I, F>> for I {
//     type Error = crate::Error;
//     fn try_from(value: JsoncValue<I, F>) -> Result<Self, Self::Error> {
//         match value {
//             JsoncValue::Number(NumberValue::Integer(v)) => Ok(v),
//             _ => Err(InvalidRepresentsValue::ShouldNumber)?,
//         }
//     }
// }
// impl<I, F> TryFrom<JsoncValue<I, F>> for F {
//     type Error = crate::Error;
//     fn try_from(value: JsoncValue<I, F>) -> Result<Self, Self::Error> {
//         match value {
//             JsoncValue::Number(NumberValue::Float(v)) => Ok(v),
//             _ => Err(InvalidRepresentsValue::ShouldNumber)?,
//         }
//     }
// }

// TODO macro?
impl<F> TryFrom<JsoncValue<u8, F>> for u8 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<u8, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(NumberValue::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<u16, F>> for u16 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<u16, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(NumberValue::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<u32, F>> for u32 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<u32, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(NumberValue::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<u64, F>> for u64 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<u64, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(NumberValue::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<u128, F>> for u128 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<u128, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(NumberValue::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<i8, F>> for i8 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<i8, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(NumberValue::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<i16, F>> for i16 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<i16, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(NumberValue::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<i32, F>> for i32 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<i32, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(NumberValue::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<i64, F>> for i64 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<i64, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(NumberValue::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<i128, F>> for i128 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<i128, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(NumberValue::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<I> TryFrom<JsoncValue<I, f32>> for f32 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<I, f32>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(NumberValue::Float(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<I> TryFrom<JsoncValue<I, f64>> for f64 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<I, f64>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(NumberValue::Float(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;

    use super::*;

    #[test]
    fn test_value_as_object() {
        let mut v: Value = r#"{ "null": null }"#.parse().unwrap();
        assert!(
            v.is_object()
                && !v.is_array()
                && !v.is_bool()
                && !v.is_null()
                && !v.is_string()
                && !v.is_number()
                && !v.is_integer()
                && !v.is_float()
        );
        assert!(
            v.as_object().is_some()
                && v.as_array().is_none()
                && v.as_bool().is_none()
                && v.as_null().is_none()
                && v.as_str().is_none()
                && v.as_number().is_none()
                && v.as_integer().is_none()
                && v.as_float().is_none()
        );
        assert_eq!(v.as_object().unwrap(), &MapImpl::from([("null".to_string(), Value::Null)]));

        let muted = {
            let mv = v.as_object_mut().unwrap();
            *mv.get_mut(&"null".to_string()).unwrap() = "null".into();
            mv.insert("key".to_string(), "value".into());
            assert_eq!(
                mv,
                &mut MapImpl::from([
                    ("null".to_string(), Value::String("null".to_string())),
                    ("key".to_string(), Value::String("value".to_string()))
                ])
            );
            mv.clone()
        };

        let owned_map: MapImpl<String, Value> = v.try_into().unwrap();
        assert_eq!(muted, owned_map);
    }

    #[test]
    fn test_value_as_array() {
        let mut v: Value = r#"[ null, "null", false, 123, 3.14 ]"#.parse().unwrap();
        assert!(
            v.is_array()
                && !v.is_object()
                && !v.is_bool()
                && !v.is_null()
                && !v.is_string()
                && !v.is_number()
                && !v.is_integer()
                && !v.is_float()
        );
        assert!(
            v.as_array().is_some()
                && v.as_object().is_none()
                && v.as_bool().is_none()
                && v.as_null().is_none()
                && v.as_str().is_none()
                && v.as_number().is_none()
                && v.as_integer().is_none()
                && v.as_float().is_none()
        );
        assert_eq!(
            v.as_array().unwrap(),
            &[
                Value::Null,
                Value::String("null".to_string()),
                Value::Bool(false),
                Value::Number(NumberValue::Integer(123)),
                Value::Number(NumberValue::Float(3.14)),
            ]
        );

        let muted = {
            let mv = v.as_array_mut().unwrap();
            assert_eq!(
                mv,
                &mut [
                    Value::Null,
                    Value::String("null".to_string()),
                    Value::Bool(false),
                    Value::Number(NumberValue::Integer(123)),
                    Value::Number(NumberValue::Float(3.14)),
                ]
            );
            *mv.get_mut(0).unwrap() = "null".into();
            mv.remove(1);
            assert_eq!(
                mv,
                &mut [
                    Value::String("null".to_string()),
                    Value::Bool(false),
                    Value::Number(NumberValue::Integer(123)),
                    Value::Number(NumberValue::Float(3.14)),
                ]
            );
            mv.clone()
        };

        let owned_vec: Vec<Value> = v.try_into().unwrap();
        assert_eq!(muted, owned_vec);
    }
}
