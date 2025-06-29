use crate::{
    imp::{Imp, Implementor},
    node::Newtype,
    traits::Trait,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

///
/// NumCastTrait
///

pub struct NumCastTrait {}

///
/// Newtype
///

impl Imp<Newtype> for NumCastTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let num_fn = node.primitive.num_cast_fn();
        let to_method = format_ident!("to_{}", num_fn);
        let from_method = format_ident!("from_{}", num_fn);

        let q = quote! {
            fn from<T: ::mimic::core::traits::NumToPrimitive>(n: T) -> Option<Self> {
                let num = n.#to_method()?;
                <Self as ::mimic::core::traits::NumFromPrimitive>::#from_method(num)
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// NumFromPrimitiveTrait
///

pub struct NumFromPrimitiveTrait {}

///
/// Newtype
///

impl Imp<Newtype> for NumFromPrimitiveTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let item = &node.item;

        let q = quote! {
            fn from_i64(n: i64) -> Option<Self> {
                type Ty = #item;
                Ty::from_i64(n).map(Self)
            }

            fn from_u64(n: u64) -> Option<Self> {
                type Ty = #item;
                Ty::from_u64(n).map(Self)
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// NumToPrimitiveTrait
///

pub struct NumToPrimitiveTrait {}

///
/// Newtype
///

impl Imp<Newtype> for NumToPrimitiveTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let q = quote! {
            fn to_i64(&self) -> Option<i64> {
                ::mimic::export::num_traits::NumCast::from(self.0)
            }

            fn to_u64(&self) -> Option<u64> {
                ::mimic::export::num_traits::NumCast::from(self.0)
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
