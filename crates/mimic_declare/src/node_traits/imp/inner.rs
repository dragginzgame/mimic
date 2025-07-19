use crate::{
    node::Newtype,
    node_traits::{Imp, Implementor, Trait},
    traits::AsMacro,
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
    fn tokens(node: &Newtype) -> Option<TokenStream> {
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

        let tokens = Implementor::new(node.ident(), Trait::Inner)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
