pub mod deserializer;
pub mod r#enum;
pub mod map;
pub mod number;
pub mod seq;
pub mod visitor;

use serde::de;

use crate::value::JsoncValue;

use super::MapImpl;

impl<'de, I: num::FromPrimitive, F: num::FromPrimitive> de::Deserialize<'de> for JsoncValue<I, F> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(visitor::JsoncValueVisitor::new())
    }
}

impl<'de, I, F> JsoncValue<I, F>
where
    I: num::ToPrimitive,
    F: num::ToPrimitive,
{
    /// Deserialize a [`JsoncValue`] as type `D`.
    ///
    /// # Examples
    /// ```
    /// use serde::Deserialize;
    /// #[derive(Deserialize)]
    /// struct Person<'a> {
    ///     name: &'a str,
    ///     age: Option<u32>,
    /// }
    ///
    /// let target = json_with_comments::jsonc!({"name": "John", "age": 30});
    /// let person: Person = target.into_deserialize().unwrap();
    /// assert!(matches!(person, Person { name: "John", age: Some(30) }));
    /// ```
    pub fn into_deserialize<D>(&'de self) -> crate::Result<D>
    where
        D: serde::Deserialize<'de>,
    {
        D::deserialize(deserializer::ValueDeserializer::new(self))
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use serde::Deserialize;

    use crate::{from_str, jsonc};

    use super::JsoncValue;

    #[test]
    fn test_deserialize_value() {
        let target = r#"{"obj":{"arr":[false,true,2,3]}}"#;
        let v: JsoncValue<i64, f64> = from_str(target).unwrap();
        assert_eq!(
            v,
            jsonc!({
                "obj": {
                    "arr": [false, true, 2, 3],
                },
            })
        );
    }

    #[test]
    fn test_bool_deserialize_as_value() {
        let target = jsonc!(true);
        let tru: bool = target.into_deserialize().unwrap();
        assert_eq!(tru, true);

        let target = jsonc!(false);
        let fal: bool = target.into_deserialize().unwrap();
        assert_eq!(fal, false);
    }

    #[test]
    fn test_string_deserialize_as_value() {
        let target = jsonc!("String");
        let string: String = target.into_deserialize().unwrap();
        assert_eq!(string, "String".to_string());

        let target = jsonc!("&str");
        let str: &str = target.into_deserialize().unwrap();
        assert_eq!(str, "&str");
    }

    #[test]
    fn test_number_deserialize_as_value() {
        let target = jsonc!(1);
        let one: u8 = target.into_deserialize().unwrap();
        assert_eq!(one, 1u8);

        let target = jsonc!(0.5);
        let half: f64 = target.into_deserialize().unwrap();
        assert_eq!(half, 0.5f64);
    }

    #[test]
    fn test_option_deserialize_as_value() {
        let target = jsonc!(false);
        let fal: Option<bool> = target.into_deserialize().unwrap();
        assert_eq!(fal, Some(false));

        let target = jsonc!(null);
        let null: Option<bool> = target.into_deserialize().unwrap();
        assert_eq!(null, None);
    }

    #[test]
    fn test_seq_deserialize_as_value() {
        let target = jsonc!([1, 2, 3]);
        let natural: Vec<u8> = target.into_deserialize().unwrap();
        assert_eq!(natural, vec![1, 2, 3]);

        let target = jsonc!([0, true, "two"]);
        let tuple: (i8, bool, String) = target.into_deserialize().unwrap();
        assert_eq!(tuple, (0, true, "two".to_string()));
    }

    #[test]
    fn test_map_deserialize_as_value() {
        let target = jsonc!({"key": "value"});
        let map: HashMap<String, String> = target.into_deserialize().unwrap();
        assert_eq!(map, HashMap::from([("key".to_string(), "value".to_string())]));
    }

    #[test]
    fn test_struct_deserialize_as_value() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Person<'a> {
            name: &'a str,
            age: Option<u32>,
        }

        let target = jsonc!({"name": "John", "age": 30});
        let person: Person = target.into_deserialize().unwrap();
        assert_eq!(person, Person { name: "John", age: Some(30) });

        let target = jsonc!([{"name": "John", "age": 30},{"name": "Jin", "age": null}]);
        let person: Vec<Person> = target.into_deserialize().unwrap();
        assert_eq!(person, [Person { name: "John", age: Some(30) }, Person { name: "Jin", age: None }]);
    }

    #[test]
    fn test_enum_deserialize_as_value() {
        #[derive(Deserialize)]
        enum Animal<'a> {
            Dog,
            Cat(u8),
            Fish(&'a str, u8),
            Bird { name: &'a str },
        }

        let target = jsonc!("Dog");
        let dog: Animal = target.into_deserialize().unwrap();
        assert!(matches!(dog, Animal::Dog));

        let target = jsonc!({"Cat": 2});
        let cat: Animal = target.into_deserialize().unwrap();
        assert!(matches!(cat, Animal::Cat(2)));

        let target = jsonc!({"Fish": ["Tuna", 3]});
        let fish: Animal = target.into_deserialize().unwrap();
        assert!(matches!(fish, Animal::Fish("Tuna", 3)));

        let target = jsonc!({"Bird": {"name": "Pigeon"}});
        let bird: Animal = target.into_deserialize().unwrap();
        assert!(matches!(bird, Animal::Bird { name: "Pigeon" }));
    }
}
