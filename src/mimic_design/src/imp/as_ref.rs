use crate::{
    imp::{Imp, Implementor},
    node::{Newtype, Trait},
};
use mimic_common::types::PrimitiveType;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// AsRefTrait
///

pub struct AsRefTrait {}

///
/// Newtype
/// only bother if it's a String
///

impl Imp<Newtype> for AsRefTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let primitive = &node.primitive;

        // text
        if matches!(primitive, PrimitiveType::Text) {
            let q = quote! {
                fn as_ref(&self) -> &str {
                    self.0.as_ref()
                }
            };

            Some(
                Implementor::new(&node.def, t)
                    .add_trait_generic(quote!(str))
                    .set_tokens(q)
                    .to_token_stream(),
            )
        } else {
            None
        }
    }
}
