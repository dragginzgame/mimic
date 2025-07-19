mod default;
mod entity;
mod enum_value;
mod field;
mod from;
mod has_store;
mod implementor;
mod inner;
mod into;
mod num;
mod type_view;
mod validate;
mod visitable;

pub use default::*;
pub use entity::*;
pub use enum_value::*;
pub use field::*;
pub use from::*;
pub use has_store::*;
pub use implementor::*;
pub use inner::*;
pub use into::*;
pub use num::*;
pub use type_view::*;
pub use validate::*;
pub use visitable::*;

use proc_macro2::TokenStream;

///
/// Imp
///

pub trait Imp<N> {
    fn tokens(node: &N) -> Option<TokenStream>;
}

///
/// ImpFn
/// for breaking down traits even further
///

pub trait ImpFn<N> {
    fn tokens(node: &N) -> TokenStream;
}
