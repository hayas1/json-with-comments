use crate::error::IndexError;

use super::JsoncValue;

pub trait JsoncIndex<T>: Sized {
    type Output: ?Sized;
    fn get(self, value: &T) -> Option<&Self::Output>;
    fn get_mut(self, value: &mut T) -> Option<&mut Self::Output>;
    fn index(self, value: &T) -> &Self::Output;
    fn index_mut(self, value: &mut T) -> &mut Self::Output;
}

impl<I, F, In: JsoncIndex<JsoncValue<I, F>>> std::ops::Index<In> for JsoncValue<I, F> {
    type Output = In::Output;
    fn index(&self, index: In) -> &Self::Output {
        index.index(self)
    }
}
impl<I, F, In: JsoncIndex<JsoncValue<I, F>>> std::ops::IndexMut<In> for JsoncValue<I, F> {
    fn index_mut(&mut self, index: In) -> &mut Self::Output {
        index.index_mut(self)
    }
}

impl<I, F> JsoncIndex<JsoncValue<I, F>> for &str {
    type Output = JsoncValue<I, F>;
    fn get(self, value: &JsoncValue<I, F>) -> Option<&Self::Output> {
        value.as_object().and_then(|map| map.get(self))
    }
    fn get_mut(self, value: &mut JsoncValue<I, F>) -> Option<&mut Self::Output> {
        value.as_object_mut().and_then(|map| map.get_mut(self))
    }
    fn index(self, value: &JsoncValue<I, F>) -> &Self::Output {
        match value {
            JsoncValue::Object(map) => &map[self],
            _ => {
                panic!("{}", IndexError::UnmatchedType { index: "str".to_string(), value: value.value_type() })
            }
        }
    }
    fn index_mut(self, value: &mut JsoncValue<I, F>) -> &mut Self::Output {
        match value {
            JsoncValue::Object(map) => map.get_mut(self).unwrap_or_else(|| panic!("no such key: \"{self}\"")),
            _ => {
                panic!("{}", IndexError::UnmatchedType { index: "str".to_string(), value: value.value_type() })
            }
        }
    }
}
impl<I, F> JsoncIndex<JsoncValue<I, F>> for usize {
    type Output = JsoncValue<I, F>;
    fn get(self, value: &JsoncValue<I, F>) -> Option<&Self::Output> {
        value.as_array().and_then(|v| v.get(self))
    }
    fn get_mut(self, value: &mut JsoncValue<I, F>) -> Option<&mut Self::Output> {
        value.as_array_mut().and_then(|v| v.get_mut(self))
    }
    fn index(self, value: &JsoncValue<I, F>) -> &Self::Output {
        match value {
            JsoncValue::Array(v) => &v[self],
            _ => {
                panic!("{}", IndexError::UnmatchedType { index: "str".to_string(), value: value.value_type() })
            }
        }
    }
    fn index_mut(self, value: &mut JsoncValue<I, F>) -> &mut Self::Output {
        match value {
            JsoncValue::Array(v) => &mut v[self],
            _ => {
                panic!("{}", IndexError::UnmatchedType { index: "str".to_string(), value: value.value_type() })
            }
        }
    }
}

pub struct Range<R>(R);
// conflicting implementations of trait `value::index::JsoncIndex<value::JsoncValue<_, _>>` for type `&str`
// upstream crates may add a new impl of trait `std::slice::SliceIndex<[value::JsoncValue<_, _>]>` for type `&str` in future versions
impl<I, F, R: std::slice::SliceIndex<[JsoncValue<I, F>]>> JsoncIndex<JsoncValue<I, F>> for Range<R> {
    type Output = R::Output;
    fn get(self, value: &JsoncValue<I, F>) -> Option<&Self::Output> {
        value.as_array().and_then(|v| v.get(self.0))
    }
    fn get_mut(self, value: &mut JsoncValue<I, F>) -> Option<&mut Self::Output> {
        value.as_array_mut().and_then(|v| v.get_mut(self.0))
    }
    fn index(self, value: &JsoncValue<I, F>) -> &Self::Output {
        match value {
            JsoncValue::Array(v) => &v[self.0],
            _ => {
                panic!("{}", IndexError::UnmatchedType { index: "str".to_string(), value: value.value_type() })
            }
        }
    }
    fn index_mut(self, value: &mut JsoncValue<I, F>) -> &mut Self::Output {
        match value {
            JsoncValue::Array(v) => &mut v[self.0],
            _ => {
                panic!("{}", IndexError::UnmatchedType { index: "str".to_string(), value: value.value_type() })
            }
        }
    }
}
