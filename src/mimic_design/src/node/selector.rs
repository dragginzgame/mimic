use crate::imp;
use crate::{
    helper::{quote_one, quote_vec, to_path, to_string},
    node::{Arg, Def, MacroNode, Node, Trait, TraitNode, TraitTokens, Traits},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
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

impl Node for Selector {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;
        let TraitTokens { derive, impls } = self.trait_tokens();

        // quote
        let schema = self.ctor_schema();
        let variants = self.variants.iter().map(Node::expand);
        let q = quote! {
            #schema
            #derive
            pub enum #ident {
                #(#variants,)*
            }
            #impls
        };

        // debug
        if self.def.debug {
            let s = q.to_string();
            return quote!(compile_error!(#s););
        }

        q
    }
}

impl MacroNode for Selector {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Selector {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let target = quote_one(&self.target, to_path);
        let variants = quote_vec(&self.variants, SelectorVariant::schema);

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

impl TraitNode for Selector {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default();
        traits.add(Trait::Into);

        // add default if needed
        if self.variants.iter().any(|v| v.default) {
            traits.add(Trait::Default);
        }

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Into => imp::into::selector(self, t),

            _ => imp::any(self, t),
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

impl Node for SelectorVariant {
    fn expand(&self) -> TokenStream {
        let name = &self.name;
        let mut q = quote!();

        // default
        if self.default {
            q.extend(quote!(#[default]));
        }

        // quote
        q.extend(quote! (#name));

        q
    }
}

impl Schemable for SelectorVariant {
    fn schema(&self) -> TokenStream {
        let name = quote_one(&self.name, to_string);
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
