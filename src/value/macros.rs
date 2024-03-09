#[macro_export]
macro_rules! jsonc {
    ($($json:tt)*) => {
        {
            let value: $crate::Value = jsonc_generics!($($json)*);
            value
        }
    };
}

#[macro_export]
macro_rules! jsonc_generics {
    // TODO comments
    ([ $($elems:tt),* $(,)? ]) => {
        $crate::value::JsoncValue::Array(vec![$(jsonc_generics!($elems)),*])
    };
    ({ $($key:tt: $value:tt),* $(,)? }) => {
        $crate::value::JsoncValue::Object({vec![$(($key.into(), jsonc_generics!($value))),*].into_iter().collect()})
    };
    (null) => {
        $crate::value::JsoncValue::Null
    };
    ($other:expr) => {
        $crate::value::JsoncValue::from($other)
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        value::{number::Number, JsoncValue},
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
    fn test_jsonc_macro() {
        let value: JsoncValue<u32, f32> = r#"[null, true, 2, [[], [[]], [[], [[]]]], {"four": 5.0}]"#.parse().unwrap();
        assert_eq!(value, jsonc_generics!([null, true, 2, [[], [[]], [[], [[]]]], {"four": 5.0}]));
        assert_eq!(crate::Value::Null, jsonc_generics!(null));
        dbg!(jsonc!({ "name": "json-with-comments" }));
    }
}