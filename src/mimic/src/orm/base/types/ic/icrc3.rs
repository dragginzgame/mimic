use crate::{orm::base::types, prelude::*};

///
/// Icrc3 Value
/// Generic value in accordance with ICRC-3
///

#[enum_(
    variant(name = "Array", value(many, item(is = "Value"))),
    variant(name = "Blob", value(item(is = "types::Blob"))),
    variant(name = "Int", value(item(is = "types::Int64"))),
    variant(name = "Map", value(item(is = "value::Map"))),
    variant(name = "Nat", value(item(is = "types::Nat64"))),
    variant(name = "Text", value(item(is = "types::Text")))
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

    #[map(key(is = "types::Text"), value(item(is = "Value")))]
    pub struct Map {}
}
