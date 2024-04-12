use crate::error::InvalidRepresentsValue;

use super::{number::Number, JsoncValue, MapImpl};

impl<I, F> JsoncValue<I, F> {
    /// Returns true if the `Value` is an `Object`. Returns false otherwise.
    ///
    /// # Examples
    /// ```
    /// use json_with_comments::jsonc;
    /// assert!(jsonc!({"key": "value"}).is_object());
    /// assert!(!jsonc!([1, 2, 3]).is_object());
    /// assert!(!jsonc!(true).is_object());
    /// assert!(!jsonc!(null).is_object());
    /// assert!(!jsonc!("value").is_object());
    /// assert!(!jsonc!(123).is_object());
    /// assert!(!jsonc!(123.45).is_object());
    /// ```
    pub fn is_object(&self) -> bool {
        matches!(self, JsoncValue::Object(_))
    }
    /// TODO doc
    pub fn as_map(&self) -> Option<&MapImpl<String, Self>> {
        match self {
            JsoncValue::Object(m) => Some(m),
            _ => None,
        }
    }
    /// TODO doc
    pub fn as_map_mut(&mut self) -> Option<&mut MapImpl<String, Self>> {
        match self {
            JsoncValue::Object(m) => Some(m),
            _ => None,
        }
    }

    /// Returns true if the `Value` is an `Array`. Returns false otherwise.
    ///
    /// # Examples
    /// ```
    /// use json_with_comments::jsonc;
    /// assert!(!jsonc!({"key": "value"}).is_array());
    /// assert!(jsonc!([1, 2, 3]).is_array());
    /// assert!(!jsonc!(true).is_array());
    /// assert!(!jsonc!(null).is_array());
    /// assert!(!jsonc!("value").is_array());
    /// assert!(!jsonc!(123).is_array());
    /// assert!(!jsonc!(123.45).is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        matches!(self, JsoncValue::Array(_))
    }
    /// TODO doc
    pub fn as_vec(&self) -> Option<&Vec<Self>> {
        match self {
            JsoncValue::Array(v) => Some(v),
            _ => None,
        }
    }
    /// TODO doc
    pub fn as_vec_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            JsoncValue::Array(v) => Some(v),
            _ => None,
        }
    }

    /// Returns true if the `Value` is an `Boolean`. Returns false otherwise.
    ///
    /// # Examples
    /// ```
    /// use json_with_comments::jsonc;
    /// assert!(!jsonc!({"key": "value"}).is_boolean());
    /// assert!(!jsonc!([1, 2, 3]).is_boolean());
    /// assert!(jsonc!(true).is_boolean());
    /// assert!(!jsonc!(null).is_boolean());
    /// assert!(!jsonc!("value").is_boolean());
    /// assert!(!jsonc!(123).is_boolean());
    /// assert!(!jsonc!(123.45).is_boolean());
    /// ```
    pub fn is_boolean(&self) -> bool {
        matches!(self, JsoncValue::Bool(_))
    }
    /// TODO doc
    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            JsoncValue::Bool(v) => Some(v),
            _ => None,
        }
    }
    /// TODO doc
    pub fn as_bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            JsoncValue::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Returns true if the `Value` is an `Null`. Returns false otherwise.
    ///
    /// # Examples
    /// ```
    /// use json_with_comments::jsonc;
    /// assert!(!jsonc!({"key": "value"}).is_null());
    /// assert!(!jsonc!([1, 2, 3]).is_null());
    /// assert!(!jsonc!(true).is_null());
    /// assert!(jsonc!(null).is_null());
    /// assert!(!jsonc!("value").is_null());
    /// assert!(!jsonc!(123).is_null());
    /// assert!(!jsonc!(123.45).is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        matches!(self, JsoncValue::Null)
    }
    /// TODO doc
    pub fn as_unit(&self) -> Option<()> {
        match self {
            JsoncValue::Null => Some(()),
            _ => None,
        }
    }

    /// Returns true if the `Value` is an `String`. Returns false otherwise.
    ///
    /// # Examples
    /// ```
    /// use json_with_comments::jsonc;
    /// assert!(!jsonc!({"key": "value"}).is_string());
    /// assert!(!jsonc!([1, 2, 3]).is_string());
    /// assert!(!jsonc!(true).is_string());
    /// assert!(!jsonc!(null).is_string());
    /// assert!(jsonc!("value").is_string());
    /// assert!(!jsonc!(123).is_string());
    /// assert!(!jsonc!(123.45).is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        matches!(self, JsoncValue::String(_))
    }
    /// TODO doc
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsoncValue::String(v) => Some(v),
            _ => None,
        }
    }
    /// TODO doc
    pub fn as_str_mut(&mut self) -> Option<&mut str> {
        match self {
            JsoncValue::String(v) => Some(v),
            _ => None,
        }
    }

    /// Returns true if the `Value` is an `Number`. Returns false otherwise.
    ///
    /// # Examples
    /// ```
    /// use json_with_comments::jsonc;
    /// assert!(!jsonc!({"key": "value"}).is_number());
    /// assert!(!jsonc!([1, 2, 3]).is_number());
    /// assert!(!jsonc!(true).is_number());
    /// assert!(!jsonc!(null).is_number());
    /// assert!(!jsonc!("value").is_number());
    /// assert!(jsonc!(123).is_number());
    /// assert!(jsonc!(123.45).is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        matches!(self, JsoncValue::Number(_))
    }
    /// TODO doc
    pub fn as_number(&self) -> Option<&Number<I, F>> {
        match self {
            JsoncValue::Number(v) => Some(v),
            _ => None,
        }
    }
    /// TODO doc
    pub fn as_number_mut(&mut self) -> Option<&mut Number<I, F>> {
        match self {
            JsoncValue::Number(v) => Some(v),
            _ => None,
        }
    }

    /// Returns true if the `Value` is an `Integer`. Returns false otherwise.
    ///
    /// # Examples
    /// ```
    /// use json_with_comments::jsonc;
    /// assert!(!jsonc!({"key": "value"}).is_integer());
    /// assert!(!jsonc!([1, 2, 3]).is_integer());
    /// assert!(!jsonc!(true).is_integer());
    /// assert!(!jsonc!(null).is_integer());
    /// assert!(!jsonc!("value").is_integer());
    /// assert!(jsonc!(123).is_integer());
    /// assert!(!jsonc!(123.45).is_integer());
    /// ```
    pub fn is_integer(&self) -> bool {
        matches!(self, JsoncValue::Number(Number::Integer(_)))
    }
    /// TODO doc
    pub fn as_integer(&self) -> Option<&I> {
        match self {
            JsoncValue::Number(Number::Integer(i)) => Some(i),
            _ => None,
        }
    }
    /// TODO doc
    pub fn as_integer_mut(&mut self) -> Option<&mut I> {
        match self {
            JsoncValue::Number(Number::Integer(i)) => Some(i),
            _ => None,
        }
    }

    /// Returns true if the `Value` is an `Float`. Returns false otherwise.
    ///
    /// # Examples
    /// ```
    /// use json_with_comments::jsonc;
    /// assert!(!jsonc!({"key": "value"}).is_float());
    /// assert!(!jsonc!([1, 2, 3]).is_float());
    /// assert!(!jsonc!(true).is_float());
    /// assert!(!jsonc!(null).is_float());
    /// assert!(!jsonc!("value").is_float());
    /// assert!(!jsonc!(123).is_float());
    /// assert!(jsonc!(123.45).is_float());
    /// ```
    pub fn is_float(&self) -> bool {
        matches!(self, JsoncValue::Number(Number::Float(_)))
    }
    /// TODO doc
    pub fn as_float(&self) -> Option<&F> {
        match self {
            JsoncValue::Number(Number::Float(f)) => Some(f),
            _ => None,
        }
    }
    /// TODO doc
    pub fn as_float_mut(&mut self) -> Option<&mut F> {
        match self {
            JsoncValue::Number(Number::Float(f)) => Some(f),
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
impl<I, F> TryFrom<JsoncValue<I, F>> for Number<I, F> {
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
//             JsoncValue::Number(Number::Integer(v)) => Ok(v),
//             _ => Err(InvalidRepresentsValue::ShouldNumber)?,
//         }
//     }
// }
// impl<I, F> TryFrom<JsoncValue<I, F>> for F {
//     type Error = crate::Error;
//     fn try_from(value: JsoncValue<I, F>) -> Result<Self, Self::Error> {
//         match value {
//             JsoncValue::Number(Number::Float(v)) => Ok(v),
//             _ => Err(InvalidRepresentsValue::ShouldNumber)?,
//         }
//     }
// }

// TODO macro?
impl<F> TryFrom<JsoncValue<u8, F>> for u8 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<u8, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(Number::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<u16, F>> for u16 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<u16, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(Number::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<u32, F>> for u32 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<u32, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(Number::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<u64, F>> for u64 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<u64, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(Number::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<u128, F>> for u128 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<u128, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(Number::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<i8, F>> for i8 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<i8, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(Number::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<i16, F>> for i16 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<i16, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(Number::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<i32, F>> for i32 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<i32, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(Number::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<i64, F>> for i64 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<i64, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(Number::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<F> TryFrom<JsoncValue<i128, F>> for i128 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<i128, F>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(Number::Integer(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<I> TryFrom<JsoncValue<I, f32>> for f32 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<I, f32>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(Number::Float(v)) => Ok(v),
            _ => Err(InvalidRepresentsValue::ShouldNumber)?,
        }
    }
}
impl<I> TryFrom<JsoncValue<I, f64>> for f64 {
    type Error = crate::Error;
    fn try_from(value: JsoncValue<I, f64>) -> Result<Self, Self::Error> {
        match value {
            JsoncValue::Number(Number::Float(v)) => Ok(v),
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
                && !v.is_boolean()
                && !v.is_null()
                && !v.is_string()
                && !v.is_number()
                && !v.is_integer()
                && !v.is_float()
        );
        assert!(
            v.as_map().is_some()
                && v.as_vec().is_none()
                && v.as_bool().is_none()
                && v.as_unit().is_none()
                && v.as_str().is_none()
                && v.as_number().is_none()
                && v.as_integer().is_none()
                && v.as_float().is_none()
        );
        assert_eq!(v.as_map().unwrap(), &MapImpl::from_iter([("null".to_string(), Value::Null)]));

        let muted = {
            let mv = v.as_map_mut().unwrap();
            *mv.get_mut(&"null".to_string()).unwrap() = "null".into();
            mv.insert("key".to_string(), "value".into());
            assert_eq!(
                mv,
                &mut MapImpl::from_iter([
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
            !v.is_object()
                && v.is_array()
                && !v.is_boolean()
                && !v.is_null()
                && !v.is_string()
                && !v.is_number()
                && !v.is_integer()
                && !v.is_float()
        );
        assert!(
            v.as_map().is_none()
                && v.as_vec().is_some()
                && v.as_bool().is_none()
                && v.as_unit().is_none()
                && v.as_str().is_none()
                && v.as_number().is_none()
                && v.as_integer().is_none()
                && v.as_float().is_none()
        );
        assert_eq!(
            v.as_vec().unwrap(),
            &[
                Value::Null,
                Value::String("null".to_string()),
                Value::Bool(false),
                Value::Number(Number::Integer(123)),
                Value::Number(Number::Float(3.14)),
            ]
        );

        let muted = {
            let mv = v.as_vec_mut().unwrap();
            *mv.get_mut(0).unwrap() = "null".into();
            mv.remove(1);
            assert_eq!(
                mv,
                &mut [
                    Value::String("null".to_string()),
                    Value::Bool(false),
                    Value::Number(Number::Integer(123)),
                    Value::Number(Number::Float(3.14)),
                ]
            );
            mv.clone()
        };

        let owned_vec: Vec<Value> = v.try_into().unwrap();
        assert_eq!(muted, owned_vec);
    }

    #[test]
    fn test_value_as_boolean() {
        let mut v: Value = r#"false"#.parse().unwrap();
        assert!(
            !v.is_object()
                && !v.is_array()
                && v.is_boolean()
                && !v.is_null()
                && !v.is_string()
                && !v.is_number()
                && !v.is_integer()
                && !v.is_float()
        );
        assert!(
            v.as_map().is_none()
                && v.as_vec().is_none()
                && v.as_bool().is_some()
                && v.as_unit().is_none()
                && v.as_str().is_none()
                && v.as_number().is_none()
                && v.as_integer().is_none()
                && v.as_float().is_none()
        );
        assert_eq!(v.as_bool().unwrap(), &false);

        let muted = {
            let mv = v.as_bool_mut().unwrap();
            *mv = true;
            assert_eq!(mv, &mut true);
            *mv
        };

        let owned_bool: bool = v.try_into().unwrap();
        assert_eq!(muted, owned_bool);
    }

    #[test]
    fn test_value_as_null() {
        let v: Value = r#"null"#.parse().unwrap();
        assert!(
            !v.is_object()
                && !v.is_array()
                && !v.is_boolean()
                && v.is_null()
                && !v.is_string()
                && !v.is_number()
                && !v.is_integer()
                && !v.is_float()
        );
        assert!(
            v.as_map().is_none()
                && v.as_vec().is_none()
                && v.as_bool().is_none()
                && v.as_unit().is_some()
                && v.as_str().is_none()
                && v.as_number().is_none()
                && v.as_integer().is_none()
                && v.as_float().is_none()
        );
        assert_eq!(v.as_unit().unwrap(), ());

        let owned_null: () = v.try_into().unwrap();
        assert_eq!(owned_null, ());
    }

    #[test]
    fn test_value_as_string() {
        let mut v: Value = r#""str""#.parse().unwrap();
        assert!(
            !v.is_object()
                && !v.is_array()
                && !v.is_boolean()
                && !v.is_null()
                && v.is_string()
                && !v.is_number()
                && !v.is_integer()
                && !v.is_float()
        );
        assert!(
            v.as_map().is_none()
                && v.as_vec().is_none()
                && v.as_bool().is_none()
                && v.as_unit().is_none()
                && v.as_str().is_some()
                && v.as_number().is_none()
                && v.as_integer().is_none()
                && v.as_float().is_none()
        );
        assert_eq!(v.as_str().unwrap(), "str");

        let mut muted = String::new();
        {
            let mv = v.as_str_mut().unwrap();
            let rmv = mv.as_mut_ptr();
            unsafe {
                *rmv = b'a';
            }
            assert_eq!(mv, "atr");
            mv.clone_into(&mut muted)
        };

        let owned_str: String = v.try_into().unwrap();
        assert_eq!(muted, owned_str);
    }

    #[test]
    fn test_value_as_number() {
        let mut v: Value = "123".parse().unwrap();
        assert!(
            !v.is_object()
                && !v.is_array()
                && !v.is_boolean()
                && !v.is_null()
                && !v.is_string()
                && v.is_number()
                && v.is_integer() // number && integer
                && !v.is_float()
        );
        assert!(
            v.as_map().is_none()
                && v.as_vec().is_none()
                && v.as_bool().is_none()
                && v.as_unit().is_none()
                && v.as_str().is_none()
                && v.as_number().is_some()
                && v.as_integer().is_some() // number && integer
                && v.as_float().is_none()
        );
        assert_eq!(v.as_number().unwrap(), &Number::Integer(123));

        let muted = {
            let mv = v.as_number_mut().unwrap();
            *mv = Number::Float(3.14);
            assert_eq!(mv, &Number::Float(3.14));
            mv.clone()
        };

        let owned_number: Number<i64, f64> = v.clone().try_into().unwrap();
        assert_eq!(muted, owned_number);

        assert!(
            !v.is_object()
                && !v.is_array()
                && !v.is_boolean()
                && !v.is_null()
                && !v.is_string()
                && v.is_number()
                && !v.is_integer()
                && v.is_float() // number && float
        );
        assert!(
            v.as_map().is_none()
                && v.as_vec().is_none()
                && v.as_bool().is_none()
                && v.as_unit().is_none()
                && v.as_str().is_none()
                && v.as_number().is_some()
                && v.as_integer().is_none()
                && v.as_float().is_some() // number && float
        )
    }
}
