use crate::{
    imp::{Imp, Implementor},
    node::Newtype,
    traits::Trait,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// InnerTrait
///

pub struct InnerTrait {}

///
/// Newtype
///

impl Imp<Newtype> for InnerTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let primitive = node.primitive.as_type();

        // quote
        let q = quote! {
            type Primitive = #primitive;

            fn inner(&self) -> Self::Primitive {
                self.0.inner()
            }

            fn into_inner(self) ->  Self::Primitive {
                self.0.into_inner()
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
