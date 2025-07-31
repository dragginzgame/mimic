use crate::{
    node::{Arg, Entity, FieldList, Newtype, Record},
    node_traits::{Imp, Implementor, Trait, TraitStrategy},
    traits::HasIdent,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// DefaultTrait
///

pub struct DefaultTrait {}

///
/// Entity
///

impl Imp<Entity> for DefaultTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let q = field_list(&node.fields);
        let tokens = Implementor::new(node.ident(), Trait::Default)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Record
///

impl Imp<Record> for DefaultTrait {
    fn strategy(node: &Record) -> Option<TraitStrategy> {
        let q = field_list(&node.fields);
        let tokens = Implementor::new(node.ident(), Trait::Default)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

// field_list
fn field_list(fields: &FieldList) -> TokenStream {
    let assignments = fields.iter().map(|field| {
        let ident = &field.ident;
        let expr = if let Some(default) = &field.default {
            format_default(default)
        } else {
            quote!(Default::default())
        };

        quote! { #ident: #expr }
    });

    quote! {
        #[allow(unused)]
        fn default() -> Self {
            Self {
                #(#assignments),*
            }
        }
    }
}

///
/// Newtype
///

impl Imp<Newtype> for DefaultTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let inner = match &node.default {
            Some(arg) => format_default(arg),
            None => panic!("newtype {} is missing a default value", node.def.ident),
        };

        // quote
        let q = quote! {
            fn default() -> Self {
                Self(#inner)
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::Default)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

// format_default
// not 100% sure NumCast will always work here, may need some extra checks
fn format_default(arg: &Arg) -> TokenStream {
    match arg {
        Arg::Bool(v) => quote!(#v),
        Arg::Char(v) => quote!(#v),
        Arg::Number(v) => {
            quote!(::mimic::core::traits::NumCast::from(#v).expect("number is valid"))
        }
        Arg::Path(path) => quote!(#path()),
        Arg::String(v) => quote!(#v),
    }
}
