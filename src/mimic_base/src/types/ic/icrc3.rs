use crate::prelude::*;

///
/// Icrc3 Value
/// Generic value in accordance with ICRC-3
///

#[enum_(
    variant(name = "Array", value(many, item(is = "Value"))),
    variant(name = "Blob", value(item(prim = "Blob"))),
    variant(name = "Int", value(item(prim = "Int64"))),
    variant(name = "Map", value(item(is = "value::Map"))),
    variant(name = "Nat", value(item(prim = "Nat64"))),
    variant(name = "Text", value(item(prim = "Text")))
)]
pub struct Value {}

impl Value {
    #[must_use]
    pub fn text(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}

pub mod value {
    use super::*;

    ///
    /// Icrc3 Value Map
    ///

    #[map(key(prim = "Text"), value(item(is = "Value")))]
    pub struct Map {}
}
