#[macro_export]
macro_rules! jsonc {
    ($($value:tt)+) => {
        value!($($value)+)
    };
}

macro_rules! value {
    ([$($tt:tt)*]) => {
        $crate::value::JsoncValue::Array(array!($($tt)*))
    };
    (null) => {
        $crate::value::JsoncValue::Null
    };
    ($other:expr) => {
        $crate::value::JsoncValue::from($other)
    };
}

macro_rules! array {
    ($($elems:expr),* $(,)?) => {
        vec![$(value!($elems)),*]
    };
}

pub fn test() {
    let value: crate::Value = r#"[true, 2, "three"]"#.parse().unwrap();
    assert_eq!(value, jsonc!([true, 2, "three",]));
}

#[cfg(test)]
mod tests {
    use crate::{value::number::Number, Value};

    use super::*;

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
    fn test_jsonc_macro_simple() {
        test()
    }
}
