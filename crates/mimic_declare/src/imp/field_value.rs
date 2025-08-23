use crate::{
    imp::{Imp, Implementor, Trait, TraitStrategy},
    node::{Enum, Newtype},
    traits::HasIdent,
};
use quote::{ToTokens, quote};

///
/// FieldValueTrait
///

pub struct FieldValueTrait {}

///
/// Enum
///

impl Imp<Enum> for FieldValueTrait {
    fn strategy(node: &Enum) -> Option<TraitStrategy> {
        // generate match arms
        let arms = node.variants.iter().map(|v| {
            let v_match = {
                let v_name = &v.name;

                if v.value.is_some() {
                    quote!(#v_name(_))
                } else {
                    quote!(#v_name)
                }
            };
            let v_name = &v.name.to_string(); // schema variant name (String)

            quote! {
                Self::#v_match => {
                    ValueEnum::new(
                        Self::PATH,
                        #v_name
                    )
                }
            }
        });

        let q = quote! {
            fn to_value(&self) -> ::mimic::core::value::Value {
                use ::mimic::core::value::{ValueEnum, Value};

                let ev = match self {
                    #(#arms),*
                };

                ::mimic::core::value::Value::Enum(ev)
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::FieldValue)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Newtype
///

impl Imp<Newtype> for FieldValueTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let q = quote! {
            fn to_value(&self) -> ::mimic::core::value::Value {
                self.0.to_value()
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::FieldValue)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
