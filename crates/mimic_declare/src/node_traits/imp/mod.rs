mod default;
mod entity;
mod enum_value;
mod field;
mod field_values;
mod from;
mod implementor;
mod index;
mod into;
mod num_cast;
mod partial_eq;
mod store;
mod type_view;
mod validate;
mod visitable;

pub use default::*;
pub use entity::*;
pub use enum_value::*;
pub use field::*;
pub use field_values::*;
pub use from::*;
pub use implementor::*;
pub use index::*;
pub use into::*;
pub use num_cast::*;
pub use partial_eq::*;
pub use store::*;
pub use type_view::*;
pub use validate::*;
pub use visitable::*;

use crate::node_traits::Trait;
use proc_macro2::TokenStream;

///
/// Imp
///

pub trait Imp<N> {
    fn strategy(node: &N) -> Option<TraitStrategy>;
}

///
/// ImpFn
/// for breaking down traits even further
///

pub trait ImpFn<N> {
    fn tokens(node: &N) -> TokenStream;
}

///
/// TraitStrategy
///

#[derive(Default, Debug)]
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

    pub fn with_derive(mut self, tr: Trait) -> Self {
        self.derive = Some(tr);
        self
    }

    pub fn with_impl(mut self, tokens: TokenStream) -> Self {
        self.imp = Some(tokens);
        self
    }
}
