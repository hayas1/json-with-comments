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
        assert_eq!(
            jsonc_generics!({"key": "val"}),
            Value::Object(vec![("key".into(), "val".into())].into_iter().collect())
        );
        assert_eq!(
            jsonc_generics!({"key": "val",}),
            Value::Object(vec![("key".into(), "val".into())].into_iter().collect())
        );
        assert_eq!(
            jsonc_generics!({"one": 1, "two": 2}),
            Value::Object(vec![("one".into(), 1.into()), ("two".into(), 2.into())].into_iter().collect())
        );
        assert_eq!(
            jsonc_generics!({"one": 1, "two": 2,}),
            Value::Object(vec![("one".into(), 1.into()), ("two".into(), 2.into())].into_iter().collect())
        );
        assert_eq!(
            jsonc_generics!({("null".to_string()): null,}),
            Value::Object(vec![("null".into(), ().into())].into_iter().collect())
        );
        assert_eq!(
            jsonc_generics!({"dict": {"key": "val"}}),
            Value::Object(
                vec![("dict".into(), vec![("key".into(), "val".into())].into_iter().collect())].into_iter().collect()
            )
        );
    }

    #[test]
    fn test_jsonc_macro() {
        assert_eq!(
            jsonc!([null, true, 2, [[], [[]], [[], [[]]]], {"four": 5.0}]),
            r#"[null, true, 2, [[], [[]], [[], [[]]]], {"four": 5.0}]"#.parse().unwrap()
        );
        assert_eq!(jsonc!(null), crate::Value::Null);
    }

    #[test]
    fn test_jsonc_macro_syntax() {
        assert_eq!(jsonc!([]), Value::Array(Vec::new()));
        assert_eq!(jsonc!([1]), Value::Array(vec![1.into()]));
        assert_eq!(jsonc!([1,]), Value::Array(vec![1.into()]));
        assert_eq!(jsonc!({}), Value::Object(MapImpl::new()));
        assert_eq!(jsonc!({"key": "value"}), Value::Object(vec![("key".into(), "value".into())].into_iter().collect()),);
        assert_eq!(
            jsonc!({"key": "value",}),
            Value::Object(vec![("key".into(), "value".into())].into_iter().collect()),
        );
    }

    #[test]
    fn test_jsonc_macro_spec() {
        assert_eq!(jsonc!(1 + 1), crate::Value::Number(Number::Integer(2)));
        assert_eq!(jsonc!([1, 1 + 1]), crate::Value::Array(vec![1.into(), 2.into()]));
        assert_eq!(jsonc!([null, 1, 1 + 1]), crate::Value::Array(vec![().into(), 1.into(), 2.into()]));
        assert_eq!(
            jsonc!({ "add": 1 + 1 }),
            crate::Value::Object(vec![("add".into(), 2.into())].into_iter().collect()),
        );
    }
}
