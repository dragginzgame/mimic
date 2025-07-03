use crate::{
    node::Newtype,
    traits::{Imp, Implementor, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// TypeViewTrait
///

pub struct TypeViewTrait {}

impl Imp<Newtype> for TypeViewTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let ident = &node.def.ident;
        let view_ident = node.def.view_ident();

        let q = quote! {
            type View = #view_ident;

            fn to_view() -> Result<Self, ::mimic::MimicError> {
                Ok(Self(from.0.into()))
            }

            fn from_view(view: Self::View) -> Result<Self, ::mimic::MimicError> {
                Ok(Self(from.0))
            }
        };

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(q)
                .to_token_stream(),
        )
    }
}
