#[cfg(not(feature = "float_map_key"))]
mod tests {
    use std::collections::HashMap;

    use json_with_comments::jsonc;

    #[test]
    fn test_float_map_key_error() {
        let v = jsonc!({"0.5": "half"});
        let m: HashMap<f32, String> = v.into_deserialize().unwrap();
    }
}

#[cfg(feature = "float_map_key")]
mod tests {
    use json_with_comments::jsonc;

    #[test]
    fn test_float_map_key() {
        let v = jsonc!({"0.5": "half"});
        let m: HashMap<f32, String> = v.into_deserialize().unwrap();
    }
}
