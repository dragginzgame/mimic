use crate::{
    node::{Arg, Entity, FieldList, Newtype, Record},
    node_traits::{
        Trait,
        imp::{Imp, Implementor},
    },
    traits::AsMacro,
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
    fn tokens(node: &Entity) -> Option<TokenStream> {
        let q = field_list(&node.fields);
        let tokens = Implementor::new(node.ident(), Trait::Default)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Record
///

impl Imp<Record> for DefaultTrait {
    fn tokens(node: &Record) -> Option<TokenStream> {
        let q = field_list(&node.fields);
        let tokens = Implementor::new(node.ident(), Trait::Default)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
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
    fn tokens(node: &Newtype) -> Option<TokenStream> {
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

        Some(tokens)
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
