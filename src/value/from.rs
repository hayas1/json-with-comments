use super::{number::Number, JsoncValue, MapImpl};

impl<I, F> From<MapImpl<String, JsoncValue<I, F>>> for JsoncValue<I, F> {
    fn from(value: MapImpl<String, JsoncValue<I, F>>) -> Self {
        JsoncValue::Object(value)
    }
}
impl<I, F> FromIterator<(String, JsoncValue<I, F>)> for JsoncValue<I, F> {
    fn from_iter<T: IntoIterator<Item = (String, JsoncValue<I, F>)>>(iter: T) -> Self {
        JsoncValue::Object(iter.into_iter().collect())
    }
}

impl<I, F> From<Vec<JsoncValue<I, F>>> for JsoncValue<I, F> {
    fn from(value: Vec<JsoncValue<I, F>>) -> Self {
        JsoncValue::Array(value)
    }
}
impl<I, F> FromIterator<JsoncValue<I, F>> for JsoncValue<I, F> {
    fn from_iter<T: IntoIterator<Item = JsoncValue<I, F>>>(iter: T) -> Self {
        JsoncValue::Array(iter.into_iter().collect())
    }
}

impl<I, F> From<bool> for JsoncValue<I, F> {
    fn from(value: bool) -> Self {
        JsoncValue::Bool(value)
    }
}

impl<I, F> From<()> for JsoncValue<I, F> {
    fn from(_: ()) -> Self {
        JsoncValue::Null
    }
}

impl<I, F> From<String> for JsoncValue<I, F> {
    fn from(value: String) -> Self {
        JsoncValue::String(value)
    }
}
impl<I, F> From<&str> for JsoncValue<I, F> {
    fn from(value: &str) -> Self {
        JsoncValue::String(value.to_owned())
    }
}

impl<I, F> From<Number<I, F>> for JsoncValue<I, F> {
    fn from(value: Number<I, F>) -> Self {
        JsoncValue::Number(value)
    }
}

// TODO conflict
// impl< I: num::Integer, F> From<I> for JsoncValue< I, F> {
//     fn from(value: I) -> Self {
//         JsoncValue::Number(Number::Integer(value))
//     }
// }
// impl< I, F: num::Float> From<F> for JsoncValue< I, F> {
//     fn from(value: F) -> Self {
//         JsoncValue::Number(Number::Float(value))
//     }
// }

// TODO macro?
impl<F> From<u8> for JsoncValue<u8, F> {
    fn from(value: u8) -> Self {
        JsoncValue::Number(Number::Integer(value))
    }
}
impl<F> From<u16> for JsoncValue<u16, F> {
    fn from(value: u16) -> Self {
        JsoncValue::Number(Number::Integer(value))
    }
}
impl<F> From<u32> for JsoncValue<u32, F> {
    fn from(value: u32) -> Self {
        JsoncValue::Number(Number::Integer(value))
    }
}
impl<F> From<u64> for JsoncValue<u64, F> {
    fn from(value: u64) -> Self {
        JsoncValue::Number(Number::Integer(value))
    }
}
impl<F> From<u128> for JsoncValue<u128, F> {
    fn from(value: u128) -> Self {
        JsoncValue::Number(Number::Integer(value))
    }
}
impl<F> From<i8> for JsoncValue<i8, F> {
    fn from(value: i8) -> Self {
        JsoncValue::Number(Number::Integer(value))
    }
}
impl<F> From<i16> for JsoncValue<i16, F> {
    fn from(value: i16) -> Self {
        JsoncValue::Number(Number::Integer(value))
    }
}
impl<F> From<i32> for JsoncValue<i32, F> {
    fn from(value: i32) -> Self {
        JsoncValue::Number(Number::Integer(value))
    }
}
impl<F> From<i64> for JsoncValue<i64, F> {
    fn from(value: i64) -> Self {
        JsoncValue::Number(Number::Integer(value))
    }
}
impl<F> From<i128> for JsoncValue<i128, F> {
    fn from(value: i128) -> Self {
        JsoncValue::Number(Number::Integer(value))
    }
}
impl<I> From<f32> for JsoncValue<I, f32> {
    fn from(value: f32) -> Self {
        JsoncValue::Number(Number::Float(value))
    }
}
impl<I> From<f64> for JsoncValue<I, f64> {
    fn from(value: f64) -> Self {
        JsoncValue::Number(Number::Float(value))
    }
}
