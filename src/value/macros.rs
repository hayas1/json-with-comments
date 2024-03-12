/// Construct a [`crate::Value`] from rust value
///
/// # Examples
/// TODO
#[macro_export]
macro_rules! jsonc {
    ($($json:tt)*) => {
        {
            let value: $crate::Value = jsonc_generics!($($json)*);
            value
        }
    };
}

/// Construct a [`crate::value::JsoncValue`] from rust value
///
/// # Examples
/// TODO
#[macro_export]
macro_rules! jsonc_generics {
    // TODO comments

    ([$($tt:tt)*]) => {
        array!([] [$($tt)*])
    };

    ({$($tt:tt)*}) => {
        object!([] () {$($tt)*})
    };

    (null) => {
        $crate::value::JsoncValue::Null
    };

    ($instance:expr) => {
        $crate::value::JsoncValue::from($instance)
    };
}

macro_rules! array {
    // Done building the array (only 1 array argument with trailing comma)
    ([$($built:expr,)*] []) => {
        $crate::value::JsoncValue::Array(vec![$($built),*])
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

macro_rules! object {
    // Done building the object (only 1 (key, value) pair array argument with trailing comma)
    ([$($built:expr,)*] () {}) => {
        // TODO? why do not match ([$(($built_key:expr, $built_value:expr),)*] () {})
        $crate::value::JsoncValue::Object([$($built,)*].into_iter().collect())
    };


    // Next value is an array
    ([$($built:expr,)*] ($($key:tt)*) {: [$($array:tt)*], $($rest:tt)+}) => {
        object!([$($built,)* ($($key)*into(), jsonc_generics!([$($array)*])),] {$($rest)+})
    };
    // Next value is an array and the last value
    ([$($built:expr,)*] ($($key:tt)*) {: [$($array:tt)*] $(,)?}) => {
        object!([$($built,)* ($($key)*into(), jsonc_generics!([$($array)*])),])
    };

    // Next value is an object
    ([$($built:expr,)*] ($($key:tt)*) {: {$($object:tt)*}, $($rest:tt)+}) => {
        object!([$($built,)* ($($key)*.into(), jsonc_generics!({$($object)*})),] {$($rest)+})
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
        assert_eq!(jsonc!(null), Value::Null);
        assert_eq!(jsonc!(true), Value::Bool(true));
        assert_eq!(jsonc!(false), Value::Bool(false));
        assert_eq!(jsonc!("string"), Value::String("string".to_string()));
        assert_eq!(jsonc!(123), Value::Number(Number::Integer(123)));
        assert_eq!(jsonc!(4.56), Value::Number(Number::Float(4.56)));
    }

    #[test]
    fn test_jsonc_macro_array() {
        assert_eq!(jsonc!([]), Value::Array(vec![]));
        assert_eq!(jsonc!([1]), Value::Array(vec![1.into()]));
        assert_eq!(jsonc!([1,]), Value::Array(vec![1.into()]));
        assert_eq!(jsonc!([1, 2]), Value::Array(vec![1.into(), 2.into()]));
        assert_eq!(jsonc!([1, 2,]), Value::Array(vec![1.into(), 2.into()]));
        assert_eq!(jsonc!([1, 1 + 1]), Value::Array(vec![1.into(), 2.into()]));
        assert_eq!(jsonc!([1, 1 + 1,]), Value::Array(vec![1.into(), 2.into()]));
        assert_eq!(jsonc!([1, "two".to_string()]), Value::Array(vec![1.into(), "two".into()]));
        assert_eq!(jsonc!([null]), Value::Array(vec![().into()]));
        assert_eq!(jsonc!([null,]), Value::Array(vec![().into()]));
        assert_eq!(jsonc!([[]]), Value::Array(vec![vec![].into()]));
        assert_eq!(jsonc!([null, [], 1 + 1]), Value::Array(vec![().into(), vec![].into(), 2.into()]));
    }

    #[test]
    fn test_jsonc_macro_object() {
        assert_eq!(jsonc!({}), Value::Object(MapImpl::new()));
        assert_eq!(jsonc!({"key": "val"}), Value::Object(vec![("key".into(), "val".into())].into_iter().collect()));
        assert_eq!(jsonc!({"key": "val",}), Value::Object(vec![("key".into(), "val".into())].into_iter().collect()));
        assert_eq!(
            jsonc!({"one": 1, "two": 2}),
            Value::Object(vec![("one".into(), 1.into()), ("two".into(), 2.into())].into_iter().collect())
        );
        assert_eq!(
            jsonc!({"one": 1, "two": 2,}),
            Value::Object(vec![("one".into(), 1.into()), ("two".into(), 2.into())].into_iter().collect())
        );
        assert_eq!(
            jsonc!({("null".to_string()): null,}),
            Value::Object(vec![("null".into(), ().into())].into_iter().collect())
        );
        assert_eq!(
            jsonc!({"dict": {"key": "val"}}),
            Value::Object(
                vec![("dict".into(), vec![("key".into(), "val".into())].into_iter().collect())].into_iter().collect()
            )
        );
    }

    #[test]
    fn test_jsonc_macro() {
        let value: JsoncValue<u32, f32> = r#"[null, true, 2, [[], [[]], [[], [[]]]], {"four": 5.0}]"#.parse().unwrap();
        assert_eq!(value, jsonc_generics!([null, true, 2, [[], [[]], [[], [[]]]], {"four": 5.0}]));
        assert_eq!(crate::Value::Null, jsonc_generics!(null));
    }

    #[test]
    fn test_jsonc_macro_syntax() {
        assert_eq!(JsoncValue::Array(Vec::new()), jsonc!([]));
        assert_eq!(JsoncValue::Array(vec![1.into()]), jsonc!([1]));
        assert_eq!(JsoncValue::Array(vec![1.into()]), jsonc!([1,]));
        assert_eq!(JsoncValue::Object(MapImpl::new()), jsonc!({}));
        assert_eq!(
            JsoncValue::Object(vec![("key".into(), "value".into())].into_iter().collect()),
            jsonc!({"key": "value"})
        );
        assert_eq!(
            JsoncValue::Object(vec![("key".into(), "value".into())].into_iter().collect()),
            jsonc!({"key": "value",})
        );
    }

    #[test]
    fn test_jsonc_macro_spec() {
        assert_eq!(crate::Value::Number(Number::Integer(2)), jsonc!(1 + 1));
        assert_eq!(crate::Value::Array(vec![1.into(), 2.into()]), jsonc!([1, 1 + 1]));
        assert_eq!(crate::Value::Array(vec![().into(), 1.into(), 2.into()]), jsonc!([null, 1, 1 + 1]));
        assert_eq!(
            crate::Value::Object(vec![("add".into(), 2.into())].into_iter().collect()),
            jsonc!({ "add": 1 + 1 })
        );
    }
}
