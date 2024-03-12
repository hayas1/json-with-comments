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
impl<I, F> JsoncValue<I, F> {
    /// TODO doc
    pub fn get<In: JsoncIndex<Self>>(&self, index: In) -> Option<&In::Output> {
        index.get(self)
    }

    /// TODO doc
    pub fn get_mut<In: JsoncIndex<Self>>(&mut self, index: In) -> Option<&mut In::Output> {
        index.get_mut(self)
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
            _ => panic!("{}", IndexError::StringIndex { value: value.value_type() }),
        }
    }
    fn index_mut(self, value: &mut JsoncValue<I, F>) -> &mut Self::Output {
        //  `IndexMut` is not implemented for `std::collections::HashMap`
        match value {
            JsoncValue::Object(map) => match map.get_mut(self) {
                Some(v) => v,
                None => panic!("{}", IndexError::NotExistKey { key: self.to_string() }),
            },
            _ => panic!("{}", IndexError::StringIndex { value: value.value_type() }),
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
            _ => panic!("{}", IndexError::StringIndex { value: value.value_type() }),
        }
    }
    fn index_mut(self, value: &mut JsoncValue<I, F>) -> &mut Self::Output {
        match value {
            JsoncValue::Array(v) => &mut v[self],
            _ => panic!("{}", IndexError::StringIndex { value: value.value_type() }),
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
            _ => panic!("{}", IndexError::SliceIndex { value: value.value_type() }),
        }
    }
    fn index_mut(self, value: &mut JsoncValue<I, F>) -> &mut Self::Output {
        match value {
            JsoncValue::Array(v) => &mut v[self.0],
            _ => panic!("{}", IndexError::SliceIndex { value: value.value_type() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::from_str;

    use super::*;

    #[test]
    fn test_index_and_get() {
        let value: JsoncValue<u64, f64> = from_str(
            r#"{
                "name": "json-with-comments",
                "keywords": [
                    "JSON with comments",
                    "parser",
                    "serde",
                ]
            }"#,
        )
        .unwrap();
        assert_eq!(value["name"], JsoncValue::String("json-with-comments".to_string()));
        assert_eq!(value["keywords"][0], JsoncValue::String("JSON with comments".to_string()));
        assert_eq!(
            value["keywords"][Range(1..)],
            [JsoncValue::String("parser".to_string()), JsoncValue::String("serde".to_string())]
        );

        assert_eq!(value.get("name"), Some(&JsoncValue::String("json-with-comments".to_string())));
        assert_eq!(value.get("version"), None);
        assert_eq!(value.get("keywords").and_then(|k| k.get(1)), Some(&JsoncValue::String("parser".to_string())));
        assert_eq!(
            value.get("keywords").and_then(|k| k.get(Range(1..)).map(|v| v.to_vec())),
            Some([JsoncValue::String("parser".to_string()), JsoncValue::String("serde".to_string())].to_vec())
        );
        assert_eq!(value.get("keywords").and_then(|k| k.get(100)), None);
        assert_eq!(value.get("keywords").and_then(|k| k.get("one")), None);
    }

    #[test]
    fn test_index_mut_and_get_mut() {
        let mut value: JsoncValue<u64, f64> = r#"{
                "name": "json-with-comments",
                "keywords": [
                    "JSON with comments",
                    "parser",
                    "serde",
                ]
            }"#
        .parse()
        .unwrap();

        value["name"] = JsoncValue::String("JSON with comments".to_string());
        value["keywords"][0] = JsoncValue::Array(vec!["JSON".into(), "with".into(), "comments".into()]);
        assert_eq!(
            value,
            r#"{
                "name": "JSON with comments",
                "keywords": [
                    ["JSON", "with", "comments"],
                    "parser",
                    "serde",
                ]
            }"#
            .parse()
            .unwrap()
        );

        value.get_mut("keywords").unwrap().get_mut(0).unwrap().as_array_mut().unwrap().push("!".into());
        assert_eq!(
            value,
            r#"{
                "name": "JSON with comments",
                "keywords": [
                    ["JSON", "with", "comments", "!"],
                    "parser",
                    "serde",
                ]
            }"#
            .parse()
            .unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_index_unmatched_type() {
        let value: JsoncValue<u64, f64> = from_str(r#"{"version": 1}"#).unwrap();
        _ = value[1];
    }

    #[test]
    #[should_panic]
    fn test_index_number_by_number() {
        let value: JsoncValue<u64, f64> = from_str(r#"{"version": 1}"#).unwrap();
        _ = value["version"][3];
    }
}
