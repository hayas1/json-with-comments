#[macro_export]
macro_rules! jsonc {
    ([ $($elems:tt),* $(,)? ]) => {
        $crate::value::JsoncValue::Array(vec![$(jsonc!($elems)),*])
    };
    ({ $($key:tt: $value:tt),* $(,)? }) => {
        $crate::value::JsoncValue::Object({vec![$(($key.into(), jsonc!($value))),*].into_iter().collect()})
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
    use crate::{value::number::Number, Value};

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
        let value: crate::Value = r#"[null, true, 2, [[], [[]], [[], [[]]]], {"four": 5.0}]"#.parse().unwrap();
        assert_eq!(value, jsonc!([null, true, 2, [[], [[]], [[], [[]]]], {"four": 5.0}]));
        assert_eq!(crate::Value::Null, jsonc!(null));
    }
}
