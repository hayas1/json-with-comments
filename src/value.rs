pub mod de;
pub mod number;
pub mod string;

pub type MapImpl<K, V> = std::collections::HashMap<K, V>;

#[derive(Debug, Clone, PartialEq)]
pub enum JsoncValue<I, F> {
    Object(MapImpl<String, JsoncValue<I, F>>),
    Array(Vec<JsoncValue<I, F>>),
    Bool(bool),
    Null,
    String(String),
    Number(number::NumberValue<I, F>),
}
