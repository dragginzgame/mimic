use crate::{
    imp::{Imp, Implementor, Trait, TraitStrategy},
    node::EnumValue,
    traits::HasIdent,
};
use quote::{ToTokens, quote};

///
/// EnumValueKindTrait
///

pub struct EnumValueKindTrait {}

///
/// EnumValue
///

impl Imp<EnumValue> for EnumValueKindTrait {
    fn strategy(node: &EnumValue) -> Option<TraitStrategy> {
        let mut inner = quote!();

        // iterate variants
        for variant in &node.variants {
            let name = &variant.name;
            let value = &variant.value;

            inner.extend(quote! {
                Self::#name => #value,
            });
        }

        // quote
        let q = quote! {
            fn value(&self) -> i32 {
                match self {
                    #inner
                }
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::EnumValueKind)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
