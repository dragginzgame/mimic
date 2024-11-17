use crate::imp;
use crate::{
    helper::{quote_one, quote_vec, to_string},
    node::{Arg, Def, MacroNode, Node, Trait, TraitNode, Traits},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

///
/// Selector
///

#[derive(Debug, FromMeta)]
pub struct Selector {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(multiple, rename = "variant")]
    pub variants: Vec<SelectorVariant>,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for Selector {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let derive = self.derive();
        let variants = self.variants.iter().map(Node::expand);
        let imp = self.imp();
        let q = quote! {
            #schema
            #derive
            pub enum #ident {
                #(#variants,)*
            }
            #imp
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
        let variants = quote_vec(&self.variants, SelectorVariant::schema);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Selector(
                ::mimic::orm::schema::node::Selector{
                    def: #def,
                    variants: #variants,
                }
            )
        }
    }
}

impl TraitNode for Selector {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();
        traits.extend(vec![Trait::EnumDisplay, Trait::ValidateAuto]);

        traits.list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        match t {
            Trait::Validator => imp::validator::selector(self, t),

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
}

impl Node for SelectorVariant {
    fn expand(&self) -> TokenStream {
        let name = &self.name;

        quote! (#name)
    }
}

impl Schemable for SelectorVariant {
    fn schema(&self) -> TokenStream {
        let name = quote_one(&self.name, to_string);
        let value = self.value.schema();

        quote! {
            ::mimic::orm::schema::node::SelectorVariant {
                name: #name,
                value : #value,
            }
        }
    }
}
