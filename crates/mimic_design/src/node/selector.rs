use crate::{
    helper::{quote_one, quote_slice, to_path, to_str_lit},
    node::{Arg, Def},
    node_traits::{self, Trait, Traits},
    traits::{MacroNode, SchemaNode},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Path};

///
/// Selector
///

#[derive(Debug, FromMeta)]
pub struct Selector {
    #[darling(default, skip)]
    pub def: Def,

    pub target: Path,

    #[darling(multiple, rename = "variant")]
    pub variants: Vec<SelectorVariant>,
}

impl ToTokens for Selector {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def;

        // quote
        let variants = &self.variants;
        tokens.extend(quote! {
            pub enum #ident {
                #(#variants,)*
            }
        });
    }
}

impl MacroNode for Selector {
    fn def(&self) -> &Def {
        &self.def
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default();

        // add default if needed
        if self.variants.iter().any(|v| v.default) {
            traits.add(Trait::Default);
        }

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        node_traits::any(self, t)
    }

    fn custom_impl(&self) -> Option<TokenStream> {
        crate::node::imp::selector::tokens(self)
    }
}

impl SchemaNode for Selector {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let target = quote_one(&self.target, to_path);
        let variants = quote_slice(&self.variants, SelectorVariant::schema);

        quote! {
            ::mimic::schema::node::SchemaNode::Selector(
                ::mimic::schema::node::Selector{
                    def: #def,
                    target: #target,
                    variants: #variants,
                }
            )
        }
    }
}

///
/// SelectorVariant
///

#[derive(Clone, Debug, FromMeta)]
pub struct SelectorVariant {
    pub name: Ident,
    pub value: Arg,

    #[darling(default)]
    pub default: bool,
}

impl ToTokens for SelectorVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;

        // default
        if self.default {
            tokens.extend(quote!(#[default]));
        }

        tokens.extend(quote! (#name));
    }
}

impl SchemaNode for SelectorVariant {
    fn schema(&self) -> TokenStream {
        let name = quote_one(&self.name, to_str_lit);
        let value = self.value.schema();
        let default = self.default;

        quote! {
            ::mimic::schema::node::SelectorVariant {
                name: #name,
                value : #value,
                default : #default,
            }
        }
    }
}
