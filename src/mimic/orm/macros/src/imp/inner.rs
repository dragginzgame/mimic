use super::Implementor;
use crate::node::{MacroNode, Newtype, Trait};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    let primitive = node.primitive.expect("node has a primitive type");

    // quote
    let field = &node.field_imp();
    let q = quote! {
        fn inner(&self) -> &#primitive {
            #field.inner()
        }
    };

    Implementor::new(node.def(), t)
        .add_trait_generic(quote!(#primitive))
        .set_tokens(q)
        .to_token_stream()
}
