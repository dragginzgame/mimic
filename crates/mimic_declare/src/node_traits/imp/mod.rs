mod canister;
mod default;
mod entity;
mod enum_value;
mod field;
mod from;
mod implementor;
mod index;
mod inner;
mod into;
mod num;
mod store;
mod type_view;
mod validate;
mod visitable;

pub use canister::*;
pub use default::*;
pub use entity::*;
pub use enum_value::*;
pub use field::*;
pub use from::*;
pub use implementor::*;
pub use index::*;
pub use inner::*;
pub use into::*;
pub use num::*;
pub use store::*;
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
