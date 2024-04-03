/// Construct a [`crate::Value`] from rust value.
///
/// # Examples
/// ```
/// use json_with_comments::{jsonc, Value, value::number::Number};
///
/// assert_eq!(jsonc!({
///         "object": {"key": "value"},
///         "array": [null, 1, 1 + 1],
///         "bool": true,
///         "null": null,
///         "string": "String".to_string(),
///         "number": 1,
///     }),
///     Value::Object(([
///         ("object".to_string(), Value::Object(vec![("key".into(), "value".into())].into_iter().collect())),
///         ("array".to_string(), Value::Array(vec![().into(), 1.into(), 2.into()])),
///         ("bool".to_string(), Value::Bool(true)),
///         ("null".to_string(), Value::Null),
///         ("string".to_string(), Value::String("String".to_string())),
///         ("number".to_string(), Value::Number(Number::Integer(1))),
///     ].into_iter().collect())),
/// );
///
/// ```
#[macro_export]
macro_rules! jsonc {
    ($($json:tt)*) => {
        {
            let value: $crate::Value = $crate::jsonc_generics!($($json)*);
            value
        }
    };
}

/// Construct a [`crate::value::JsoncValue`] from rust value.
/// If use without generics, see [`jsonc!`] also.
///
/// # Examples
/// ```
/// use json_with_comments::{jsonc_generics, value::{JsoncValue, number::Number}};
///
/// assert_eq!(jsonc_generics!({
///         "object": {"key": "value"},
///         "array": [null, 1, 1 + 1],
///         "bool": true,
///         "null": null,
///         "string": "String".to_string(),
///         "number": 1,
///     }),
///     JsoncValue::<u8, f32>::Object(([
///         ("object".to_string(), JsoncValue::Object(vec![("key".into(), "value".into())].into_iter().collect())),
///         ("array".to_string(), JsoncValue::Array(vec![().into(), 1.into(), 2.into()])),
///         ("bool".to_string(), JsoncValue::Bool(true)),
///         ("null".to_string(), JsoncValue::Null),
///         ("string".to_string(), JsoncValue::String("String".to_string())),
///         ("number".to_string(), JsoncValue::Number(Number::Integer(1))),
///     ].into_iter().collect())),
/// );
///
/// ```
#[macro_export]
macro_rules! jsonc_generics {
    // TODO comments

    ([$($array:tt)*]) => {
        $crate::array!([] [$($array)*])
    };

    ({$($object:tt)*}) => {
        $crate::object!([] () {$($object)*})
    };

    (null) => {
        $crate::value::JsoncValue::Null
    };

    ($instance:expr) => {
        $crate::value::JsoncValue::from($instance)
    };
}

/// This is inner macro to construct a [`crate::value::JsoncValue::Array`] from rust value.
/// To construct [`crate::value::JsoncValue`], see [`jsonc!`] and [`jsonc_generics!`].
///
/// # How it works
/// [`array!`]: crate::array!
/// [`array!`] macro has two array arguments.
/// First is built array, and second is rest of the array.
/// For example, parse array `[1, 2, 3]` with [`jsonc_generics!`].
/// 1. [`jsonc_generics!`] call [`array!`] with `array!([] [1, 2, 3])`.
/// 1. [`array!`] call [`array!`] with `array!([1,] [2, 3])`.
/// 1. [`array!`] call [`array!`] with `array!([1, 2,] [3])`.
/// 1. [`array!`] call [`array!`] with `array!([1, 2, 3,] [])`.
/// 1. then, rest array is empty, so [`array!`] return array `[1, 2, 3]`
///
/// # Examples
/// ```
/// use json_with_comments::{array, value::JsoncValue};
///
/// assert_eq!(
///     array!([] [1, 2, 3]),
///     JsoncValue::<u32, f32>::Array(vec![1.into(), 2.into(), 3.into()])
/// );
/// ```
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! array {
    // Done building the array
    ([$($built:expr,)*] []) => {
        $crate::value::JsoncValue::Array([$($built),*].into())
    };

    // Next value is an array
    ([$($built:expr,)*] [[$($array:tt)*], $($rest:tt)+]) => {
        array!([$($built,)* jsonc_generics!([$($array)*]),] [$($rest)+])
    };
    // Next value is an array and the last value
    ([$($built:expr,)*] [[$($array:tt)*] $(,)?]) => {
        array!([$($built,)* jsonc_generics!([$($array)*]),] [])
    };

    // Next value is an object
    ([$($built:expr,)*] [{$($object:tt)*}, $($rest:tt)+]) => {
        array!([$($built,)* jsonc_generics!({$($object)*}),] [$($rest)+])
    };
    // Next value is an object and the last value
    ([$($built:expr,)*] [{$($object:tt)*} $(,)?]) => {
        array!([$($built,)* jsonc_generics!({$($object)*}),] [])
    };

    // Next value is `null`
    ([$($built:expr,)*] [null, $($rest:tt)+]) => {
        array!([$($built,)* jsonc_generics!(null),] [$($rest)+])
    };
    // Next value is `null` and the last value
    ([$($built:expr,)*] [null $(,)?]) => {
        array!([$($built,)* jsonc_generics!(null),] [])
    };

    // Next value is an expression
    ([$($built:expr,)*] [$next:expr, $($rest:tt)+]) => {
        array!([$($built,)* jsonc_generics!($next),] [$($rest)+])
    };
    // Next value is an expression and the last value
    ([$($built:expr,)*] [$next:expr $(,)?]) => {
        array!([$($built,)* jsonc_generics!($next),] [])
    };
}

/// This is inner macro to construct a [`crate::value::JsoncValue::Object`] from rust value.
/// To construct [`crate::value::JsoncValue`], see [`jsonc!`] and [`jsonc_generics!`].
///
/// # How it works
/// [`object!`]: crate::object!
/// [`object!`] macro has three arguments.
/// First is built (key, value) pair array, and second is building key, and rest of the object.
/// For example, parse object `{"a": 1, "b": 2}` with [`jsonc_generics!`].
/// 1. [`jsonc_generics!`] call [`object!`] with `object!([] () {"a": 1, "b": 2})`.
/// 1. [`object!`] munch token tree and call [`object!`] with `object!([] ("a") {: 1, "b": 2})`.
/// 1. [`object!`] consume built key and call [`object!`] with `object!([("a", 1),] () {"b": 2})`.
/// 1. [`object!`] munch token tree and call [`object!`] with `object!([("a", 1),] ("b") {: 2})`.
/// 1. [`object!`] consume built key and call [`object!`] with `object!([("a", 1), ("b", 2),] () {})`.
/// 1. then, rest object is empty, so [`object!`] return object `{"a": 1, "b": 2}`
///
/// # Examples
/// ```
/// use json_with_comments::{object, value::JsoncValue};
///
/// assert_eq!(
///     object!([] () {"a": 1, "b": 2}),
///     JsoncValue::<u32, f32>::Object(vec![("a".into(), 1.into()), ("b".into(), 2.into())].into_iter().collect())
/// );
/// ```
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! object {
    // Done building the object
    ([$($built:expr,)*] () {}) => {
        // TODO? why do not match ([$(($built_key:expr, $built_value:expr),)*] () {})
        $crate::value::JsoncValue::Object([$($built,)*].into_iter().collect())
    };

    // Next value is an array
    ([$($built:expr,)*] ($($key:tt)*) {: [$($array:tt)*], $($rest:tt)+}) => {
        object!([$($built,)* ($($key)*.into(), jsonc_generics!([$($array)*])),] () {$($rest)+})
    };
    // Next value is an array and the last value
    ([$($built:expr,)*] ($($key:tt)*) {: [$($array:tt)*] $(,)?}) => {
        object!([$($built,)* ($($key)*.into(), jsonc_generics!([$($array)*])),] () {})
    };

    // Next value is an object
    ([$($built:expr,)*] ($($key:tt)*) {: {$($object:tt)*}, $($rest:tt)+}) => {
        object!([$($built,)* ($($key)*.into(), jsonc_generics!({$($object)*})),] () {$($rest)+})
    };
    // Next value is an object and the last value
    ([$($built:expr,)*] ($($key:tt)*) {: {$($object:tt)*} $(,)?}) => {
        object!([$($built,)* ($($key)*.into(), jsonc_generics!({$($object)*})),] () {})
    };

    // Next value is `null`
    ([$($built:expr,)*] ($($key:tt)*) {: null, $($rest:tt)+}) => {
        object!([$($built,)* ($($key)*.into(), jsonc_generics!(null)),] () {$($rest)+})
    };
    // Next value is `null` and the last value
    ([$($built:expr,)*] ($($key:tt)*) {: null $(,)?}) => {
        object!([$($built,)* ($($key)*.into(), jsonc_generics!(null)),] () {})
    };

    // Next value is an expression
    ([$($built:expr,)*] ($($key:tt)*) {: $value:expr, $($rest:tt)+}) => {
        object!([$($built,)* ($($key)*.into(), jsonc_generics!($value)),] () {$($rest)+})
    };
    // Next value is an expression and the last value
    ([$($built:expr,)*] ($($key:tt)*) {: $value:expr $(,)?}) => {
        object!([$($built,)* ($($key)*.into(), jsonc_generics!($value)),] () {})
    };

    // (last match) munch key while `:` occurred
    ([$($built:expr,)*] ($($key:tt)*) {$head:tt $($rest:tt)*}) => {
        object!([$($built,)*] ($($key)* $head) {$($rest)*})
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        value::{number::Number, JsoncValue, MapImpl},
        Value,
    };

    #[test]
    fn test_jsonc_macro_literal() {
        assert_eq!(jsonc_generics!(null), JsoncValue::<u8, f32>::Null);
        assert_eq!(jsonc_generics!(true), JsoncValue::<u32, f64>::Bool(true));
        assert_eq!(jsonc_generics!(false), JsoncValue::<u128, f32>::Bool(false));
        assert_eq!(jsonc_generics!("string"), JsoncValue::<i8, f64>::String("string".to_string()));
        assert_eq!(jsonc_generics!(123), JsoncValue::<i32, f32>::Number(Number::Integer(123)));
        assert_eq!(jsonc_generics!(4.56), JsoncValue::<i128, f64>::Number(Number::Float(4.56)));
    }

    #[test]
    fn test_jsonc_macro_array() {
        assert_eq!(jsonc_generics!([]), Value::Array(vec![]));
        assert_eq!(jsonc_generics!([1]), Value::Array(vec![1.into()]));
        assert_eq!(jsonc_generics!([1,]), Value::Array(vec![1.into()]));
        assert_eq!(jsonc_generics!([1, 2]), Value::Array(vec![1.into(), 2.into()]));
        assert_eq!(jsonc_generics!([1, 2,]), Value::Array(vec![1.into(), 2.into()]));
        assert_eq!(jsonc_generics!([1, 1 + 1]), Value::Array(vec![1.into(), 2.into()]));
        assert_eq!(jsonc_generics!([1, 1 + 1,]), Value::Array(vec![1.into(), 2.into()]));
        assert_eq!(jsonc_generics!([1, "two".to_string()]), Value::Array(vec![1.into(), "two".into()]));
        assert_eq!(jsonc_generics!([null]), Value::Array(vec![().into()]));
        assert_eq!(jsonc_generics!([null,]), Value::Array(vec![().into()]));
        assert_eq!(jsonc_generics!([[]]), Value::Array(vec![vec![].into()]));
        assert_eq!(jsonc_generics!([null, [], 1 + 1]), Value::Array(vec![().into(), vec![].into(), 2.into()]));
    }

    #[test]
    fn test_jsonc_macro_object() {
        assert_eq!(jsonc_generics!({}), Value::Object(MapImpl::new()));
        assert_eq!(jsonc_generics!({"key": "val"}), Value::Object(MapImpl::from_iter([("key".into(), "val".into())])));
        assert_eq!(jsonc_generics!({"key": "val",}), Value::Object(MapImpl::from_iter([("key".into(), "val".into())])));
        assert_eq!(
            jsonc_generics!({"one": 1, "two": 2}),
            Value::Object(MapImpl::from_iter([("one".into(), 1.into()), ("two".into(), 2.into())]))
        );
        assert_eq!(
            jsonc_generics!({"one": 1, "two": 2,}),
            Value::Object(MapImpl::from_iter([("one".into(), 1.into()), ("two".into(), 2.into())]))
        );
        assert_eq!(
            jsonc_generics!({("null".to_string()): null,}),
            Value::Object(MapImpl::from_iter([("null".into(), ().into())]))
        );
        assert_eq!(
            jsonc_generics!({"dict": {"key": "val"}}),
            Value::Object(MapImpl::from_iter([(
                "dict".into(),
                Value::Object(MapImpl::from_iter([("key".into(), "val".into())]))
            )]))
        );
    }

    #[test]
    fn test_jsonc_macro() {
        assert_eq!(
            jsonc!([null, true, 2, [[], [[]], [[], [[]]]], {"four": 5.0}]),
            Value::Array(vec![
                ().into(),
                true.into(),
                2.into(),
                Value::Array(vec![
                    Value::Array(vec![]),
                    Value::Array(vec![Value::Array(vec![])]),
                    Value::Array(vec![Value::Array(vec![]), Value::Array(vec![Value::Array(vec![])])]),
                ]),
                Value::Object(MapImpl::from_iter([("four".into(), 5.0.into())])),
            ])
        );
        assert_eq!(jsonc!(null), crate::Value::Null);
        assert_eq!(jsonc!(()), crate::Value::Null);
        assert_eq!(
            jsonc!({"a": {"b": {"c": {"d": { "e": {}}}}}, "abc": "def"}),
            crate::Value::Object(MapImpl::from_iter([
                (
                    "a".into(),
                    Value::Object(MapImpl::from_iter([(
                        "b".into(),
                        Value::Object(MapImpl::from_iter([(
                            "c".into(),
                            Value::Object(MapImpl::from_iter([(
                                "d".into(),
                                Value::Object(MapImpl::from_iter([("e".into(), Value::Object(MapImpl::new()))])),
                            )]))
                        )]))
                    )]))
                ),
                ("abc".into(), Value::String("def".into())),
            ]))
        );
        assert_eq!(
            jsonc!({
                "object": {
                    "object": {},
                    "array": [],
                    "bool": true,
                    "null": null,
                    "string": "String".to_string(),
                    "number": 1,
                },
                "array": [{}, [], false, (), "str", 2.5,],
                "bool": true,
                "null": null,
                "string": "SSSSSSSSSSSSSSSS".to_string(),
                "number": 1111111111111111,
            }),
            crate::Value::Object(MapImpl::from_iter([
                (
                    "object".into(),
                    Value::Object(MapImpl::from_iter([
                        ("object".into(), Value::Object(MapImpl::new())),
                        ("array".into(), Value::Array(Vec::new())),
                        ("bool".into(), true.into()),
                        ("null".into(), ().into()),
                        ("string".into(), "String".into()),
                        ("number".into(), 1.into()),
                    ]))
                ),
                (
                    "array".into(),
                    Value::Array(vec![
                        Value::Object(MapImpl::new()),
                        Value::Array(Vec::new()),
                        false.into(),
                        ().into(),
                        "str".into(),
                        2.5.into(),
                    ])
                ),
                ("bool".into(), true.into()),
                ("null".into(), ().into()),
                ("string".into(), "SSSSSSSSSSSSSSSS".into()),
                ("number".into(), 1111111111111111.into()),
            ]))
        )
    }

    #[test]
    fn test_jsonc_macro_syntax() {
        assert_eq!(jsonc!([]), Value::Array(Vec::new()));
        assert_eq!(jsonc!([1]), Value::Array(vec![1.into()]));
        assert_eq!(jsonc!([1,]), Value::Array(vec![1.into()]));
        assert_eq!(jsonc!({}), Value::Object(MapImpl::new()));
        assert_eq!(jsonc!({"key": "val"}), Value::Object(MapImpl::from_iter([("key".into(), "val".into())])),);
        assert_eq!(jsonc!({"key": "val",}), Value::Object(MapImpl::from_iter([("key".into(), "val".into())])),);
    }

    #[test]
    fn test_jsonc_macro_spec() {
        assert_eq!(jsonc!(1 + 1), 2.into());
        assert_eq!(jsonc!([1, 1 + 1]), crate::Value::Array(vec![1.into(), 2.into()]));
        assert_eq!(jsonc!([null, 1, 1 + 1]), crate::Value::Array(vec![().into(), 1.into(), 2.into()]));
        assert_eq!(jsonc!({ "add": 1 + 1 }), crate::Value::Object(MapImpl::from_iter([("add".into(), 2.into())])),);
        assert_eq!(
            jsonc!({"string".to_string(): 1, "str": 2}),
            crate::Value::Object(MapImpl::from_iter([("string".into(), 1.into()), ("str".into(), 2.into())]))
        );
    }
}
