pub mod visitor;

use serde::Deserialize;

use crate::value::JsoncValue;

use super::MapImpl;

impl<'de, I: num::FromPrimitive, F: num::FromPrimitive> Deserialize<'de> for JsoncValue<I, F> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(visitor::JsoncValueVisitor::new())
    }
}
