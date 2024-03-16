pub mod r#enum;
pub mod jsonc;
pub mod map;
pub mod number;
pub mod seq;
pub mod string;

#[cfg(test)]
mod tests {
    use crate::from_str;

    #[test]
    fn test_deserialize_literal() {
        assert_eq!(from_str::<bool>("true").unwrap(), true);
        assert_eq!(from_str::<bool>("false").unwrap(), false);
        assert_eq!(from_str::<()>("null").unwrap(), ());
    }

    #[test]
    fn test_deserialize_string() {
        assert_eq!(from_str::<String>(r#""hello world""#).unwrap(), "hello world".to_string());
        assert_eq!(from_str::<&str>(r#""12345""#).unwrap(), "12345");
        assert_eq!(from_str::<String>(r#""🥒💯""#).unwrap(), "🥒💯".to_string());
    }

    #[test]
    fn test_deserialize_number() {
        assert_eq!(from_str::<u64>("57").unwrap(), 57);
        assert_eq!(from_str::<i128>("-99999999999999999").unwrap(), -99999999999999999);
        assert_eq!(from_str::<f32>("3.1415926535").unwrap(), 3.1415926535);
        assert_eq!(from_str::<f64>("6.02214076e23").unwrap(), 6.02214076E23);
    }
}
