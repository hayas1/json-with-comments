use crate::error::IndexError;

use super::JsoncValue;

pub trait JsoncIndex<T>: Sized {
    type Indexer: JsoncIndexer<Self, T>;
}

impl<I, F, In: JsoncIndex<JsoncValue<I, F>>> std::ops::Index<In> for JsoncValue<I, F> {
    type Output = <In::Indexer as JsoncIndexer<In, JsoncValue<I, F>>>::Output;
    fn index(&self, index: In) -> &Self::Output {
        In::Indexer::index(self, index)
    }
}
impl<I, F, In: JsoncIndex<JsoncValue<I, F>>> std::ops::IndexMut<In> for JsoncValue<I, F> {
    fn index_mut(&mut self, index: In) -> &mut Self::Output {
        In::Indexer::index_mut(self, index)
    }
}
impl<I, F> JsoncValue<I, F> {
    /// TODO doc
    pub fn get<In: JsoncIndex<Self>>(
        &self,
        index: In,
    ) -> Option<&<In::Indexer as JsoncIndexer<In, JsoncValue<I, F>>>::Output> {
        In::Indexer::get(self, index)
    }

    /// TODO doc
    pub fn get_mut<In: JsoncIndex<Self>>(
        &mut self,
        index: In,
    ) -> Option<&mut <In::Indexer as JsoncIndexer<In, JsoncValue<I, F>>>::Output> {
        In::Indexer::get_mut(self, index)
    }
}

pub trait JsoncIndexer<T, V>
where
    T: JsoncIndex<V>,
{
    type Output: ?Sized;
    fn get(value: &V, index: T) -> Option<&Self::Output>;
    fn get_mut(value: &mut V, index: T) -> Option<&mut Self::Output>;
    fn index(value: &V, index: T) -> &Self::Output;
    fn index_mut(value: &mut V, index: T) -> &mut Self::Output;
}

pub enum StringIndexer {}
impl<'a, I, F> JsoncIndexer<&'a str, JsoncValue<I, F>> for StringIndexer {
    type Output = JsoncValue<I, F>;

    fn get<'b>(value: &'b JsoncValue<I, F>, index: &'a str) -> Option<&'b Self::Output> {
        value.as_object().and_then(|map| map.get(index))
    }

    fn get_mut<'b>(value: &'b mut JsoncValue<I, F>, index: &'a str) -> Option<&'b mut Self::Output> {
        value.as_object_mut().and_then(|map| map.get_mut(index))
    }

    fn index<'b>(value: &'b JsoncValue<I, F>, index: &'a str) -> &'b Self::Output {
        &value.as_object().unwrap_or_else(|| panic!("{}", IndexError::StringIndex { value: value.value_type() }))[index]
    }

    fn index_mut<'b>(value: &'b mut JsoncValue<I, F>, index: &'a str) -> &'b mut Self::Output {
        //  `IndexMut` is not implemented for `std::collections::HashMap`

        // cannot borrow `*value` as immutable because it is also borrowed as mutable
        // value
        //     .as_object_mut()
        //     .unwrap_or_else(|| panic!("{}", IndexError::StringIndex { value: value.value_type() }))
        //     .get_mut(index)
        //     .unwrap_or_else(|| panic!("{}", IndexError::NotExistKey { key: index.to_string() }))
        match value {
            JsoncValue::Object(map) => {
                map.get_mut(index).unwrap_or_else(|| panic!("{}", IndexError::NotExistKey { key: index.to_string() }))
            }
            _ => panic!("{}", IndexError::StringIndex { value: value.value_type() }),
        }
    }
}

pub enum SliceIndexer {}
impl<I, F, S: std::slice::SliceIndex<[JsoncValue<I, F>]> + JsoncIndex<JsoncValue<I, F>>>
    JsoncIndexer<S, JsoncValue<I, F>> for SliceIndexer
{
    type Output = <S as std::slice::SliceIndex<[JsoncValue<I, F>]>>::Output;

    fn get(value: &JsoncValue<I, F>, index: S) -> Option<&Self::Output> {
        value.as_array().and_then(|v| v.get(index))
    }

    fn get_mut(value: &mut JsoncValue<I, F>, index: S) -> Option<&mut Self::Output> {
        value.as_array_mut().and_then(|v| v.get_mut(index))
    }

    fn index(value: &JsoncValue<I, F>, index: S) -> &Self::Output {
        &value.as_array().unwrap_or_else(|| panic!("{}", IndexError::StringIndex { value: value.value_type() }))[index]
    }

    fn index_mut(value: &mut JsoncValue<I, F>, index: S) -> &mut Self::Output {
        // cannot borrow `*value` as immutable because it is also borrowed as mutable
        // &mut value.as_array_mut().unwrap_or_else(|| panic!("{}", IndexError::StringIndex { value: value.value_type() }))[index]
        match value {
            JsoncValue::Array(v) => &mut v[index],
            _ => panic!("{}", IndexError::StringIndex { value: value.value_type() }),
        }
    }
}

impl<I, F> JsoncIndex<JsoncValue<I, F>> for &str {
    type Indexer = StringIndexer;
}
impl<I, F> JsoncIndex<JsoncValue<I, F>> for usize {
    type Indexer = SliceIndexer;
}
impl<I, F> JsoncIndex<JsoncValue<I, F>> for std::ops::Range<usize> {
    type Indexer = SliceIndexer;
}
impl<I, F> JsoncIndex<JsoncValue<I, F>> for std::ops::RangeFrom<usize> {
    type Indexer = SliceIndexer;
}
impl<I, F> JsoncIndex<JsoncValue<I, F>> for std::ops::RangeFull {
    type Indexer = SliceIndexer;
}
impl<I, F> JsoncIndex<JsoncValue<I, F>> for std::ops::RangeInclusive<usize> {
    type Indexer = SliceIndexer;
}
impl<I, F> JsoncIndex<JsoncValue<I, F>> for std::ops::RangeTo<usize> {
    type Indexer = SliceIndexer;
}
impl<I, F> JsoncIndex<JsoncValue<I, F>> for std::ops::RangeToInclusive<usize> {
    type Indexer = SliceIndexer;
}

#[cfg(test)]
mod tests {
    use crate::{jsonc, jsonc_generics};

    use super::*;

    #[test]
    fn test_index_and_get() {
        let value = jsonc!({
            "name": "json-with-comments",
            "keywords": [
                "JSON with comments",
                "parser",
                "serde",
            ]
        });
        assert_eq!(value["name"], JsoncValue::String("json-with-comments".to_string()));
        assert_eq!(value["keywords"][0], JsoncValue::String("JSON with comments".to_string()));

        assert_eq!(value.get("name"), Some(&JsoncValue::String("json-with-comments".to_string())));
        assert_eq!(value.get("version"), None);
        assert_eq!(value.get("keywords").and_then(|k| k.get(1)), Some(&JsoncValue::String("parser".to_string())));
        assert_eq!(value.get("keywords").and_then(|k| k.get(100)), None);
        assert_eq!(value.get("keywords").and_then(|k| k.get("one")), None);
    }

    #[test]
    fn test_index_mut_and_get_mut() {
        let mut value = jsonc!({
            "name": "json-with-comments",
            "keywords": [
                "JSON with comments",
                "parser",
                "serde",
            ]
        });

        value["name"] = JsoncValue::String("JSON with comments".to_string());
        value["keywords"][0] = JsoncValue::Array(vec!["JSON".into(), "with".into(), "comments".into()]);
        assert_eq!(
            value,
            jsonc!({
                "name": "JSON with comments",
                "keywords": [
                    ["JSON", "with", "comments"],
                    "parser",
                    "serde",
                ]
            })
        );

        value.get_mut("keywords").unwrap().get_mut(0).unwrap().as_array_mut().unwrap().push("!".into());
        assert_eq!(
            value,
            jsonc!({
                "name": "JSON with comments",
                "keywords": [
                    ["JSON", "with", "comments", "!"],
                    "parser",
                    "serde",
                ]
            })
        );
    }

    #[test]
    fn test_range_index_and_range_get() {
        let value = jsonc!({
            "number": ["one", "two", "three", "four", "five"],
        });

        assert_eq!(
            value["number"][1..3],
            [JsoncValue::String("two".to_string()), JsoncValue::String("three".to_string())]
        );
        assert_eq!(
            value["number"][3..],
            [JsoncValue::String("four".to_string()), JsoncValue::String("five".to_string())]
        );
        assert_eq!(
            value["number"][..],
            [
                JsoncValue::String("one".to_string()),
                JsoncValue::String("two".to_string()),
                JsoncValue::String("three".to_string()),
                JsoncValue::String("four".to_string()),
                JsoncValue::String("five".to_string()),
            ]
        );
        assert_eq!(
            value["number"][1..=2],
            [JsoncValue::String("two".to_string()), JsoncValue::String("three".to_string()),]
        );
        assert_eq!(value["number"][..1], [JsoncValue::String("one".to_string())]);
        assert_eq!(
            value["number"][..=1],
            [JsoncValue::String("one".to_string()), JsoncValue::String("two".to_string()),]
        );

        assert_eq!(
            value["number"].get(1..3),
            Some(&[JsoncValue::String("two".to_string()), JsoncValue::String("three".to_string())][..])
        );
        assert_eq!(
            value["number"].get(3..),
            Some(&[JsoncValue::String("four".to_string()), JsoncValue::String("five".to_string())][..])
        );
        assert_eq!(
            value["number"].get(..),
            Some(
                &[
                    JsoncValue::String("one".to_string()),
                    JsoncValue::String("two".to_string()),
                    JsoncValue::String("three".to_string()),
                    JsoncValue::String("four".to_string()),
                    JsoncValue::String("five".to_string()),
                ][..]
            )
        );
        assert_eq!(
            value["number"].get(1..=2),
            Some(&[JsoncValue::String("two".to_string()), JsoncValue::String("three".to_string()),][..])
        );
        assert_eq!(value["number"].get(..1), Some(&[JsoncValue::String("one".to_string())][..]));
        assert_eq!(
            value["number"].get(..=1),
            Some(&[JsoncValue::String("one".to_string()), JsoncValue::String("two".to_string())][..])
        );

        assert_eq!(value["number"].get(2..2), Some(&[][..]));
        assert_eq!(value["number"].get(10000..), None);
    }

    #[test]
    fn test_nested_index_and_nested_get() {
        let value = jsonc!({
            "object1": {
                "object2": {
                    "array": [false, true],
                }
            }
        });
        assert_eq!(value["object1"]["object2"]["array"][1], JsoncValue::Bool(true));
        assert_eq!(
            ["object1", "object2", "array"].iter().fold(&value, |val, &key| &val[key]),
            &JsoncValue::Array(vec![JsoncValue::Bool(false), JsoncValue::Bool(true)])
        );

        assert_eq!(
            ["object1", "object2", "array"].iter().try_fold(&value, |val, &key| val.get(key)),
            Some(&JsoncValue::Array(vec![JsoncValue::Bool(false), JsoncValue::Bool(true)]))
        );
    }

    #[test]
    #[should_panic]
    fn test_index_unmatched_type() {
        let value: JsoncValue<u64, f64> = jsonc_generics!({"version": 1});
        _ = value[1];
    }

    #[test]
    #[should_panic]
    fn test_index_number_by_number() {
        let value: JsoncValue<u64, f64> = jsonc_generics!({"version": 1});
        _ = value["version"][3];
    }
}
