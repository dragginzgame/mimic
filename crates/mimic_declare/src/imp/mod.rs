mod default;
mod entity;
mod enum_value;
mod field_value;
mod field_values;
mod from;
mod implementor;
mod inherent;
mod inner;
mod into;
mod num_cast;
mod partial_eq;
mod partial_ord;
mod sanitize;
mod store;
mod validate;
mod view;
mod visitable;

pub use default::*;
pub use entity::*;
pub use enum_value::*;
pub use field_value::*;
pub use field_values::*;
pub use from::*;
pub use implementor::*;
pub use inherent::*;
pub use inner::*;
pub use into::*;
pub use num_cast::*;
pub use partial_eq::*;
pub use partial_ord::*;
pub use sanitize::*;
pub use store::*;
pub use validate::*;
pub use view::*;
pub use visitable::*;

use crate::schema_traits::Trait;
use proc_macro2::TokenStream;

///
/// Imp
///

pub trait Imp<N> {
    fn strategy(node: &N) -> Option<TraitStrategy>;
}

///
/// TraitStrategy
///

#[derive(Debug, Default)]
pub struct TraitStrategy {
    pub derive: Option<Trait>,
    pub imp: Option<TokenStream>,
}

impl TraitStrategy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_derive(tr: Trait) -> Self {
        Self::new().with_derive(tr)
    }

    pub fn from_impl(tokens: TokenStream) -> Self {
        Self::new().with_impl(tokens)
    }

    pub const fn with_derive(mut self, tr: Trait) -> Self {
        self.derive = Some(tr);
        self
    }

    pub fn with_impl(mut self, tokens: TokenStream) -> Self {
        self.imp = Some(tokens);
        self
    }
}
