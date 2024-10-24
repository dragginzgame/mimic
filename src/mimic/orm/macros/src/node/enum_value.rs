use super::{
    helper::{quote_one, quote_vec, to_string},
    Def, MacroNode, Node, Trait, TraitNode, Traits,
};
use crate::imp;
use darling::FromMeta;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

///
/// EnumValue
///

#[derive(Debug, FromMeta)]
pub struct EnumValue {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(multiple, rename = "variant")]
    pub variants: Vec<EnumValueVariant>,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for EnumValue {
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

impl MacroNode for EnumValue {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for EnumValue {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let variants = quote_vec(&self.variants, EnumValueVariant::schema);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::EnumValue(
                ::mimic::orm::schema::node::EnumValue{
                    def: #def,
                    variants: #variants,
                }
            )
        }
    }
}

impl TraitNode for EnumValue {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_db_traits();
        traits.extend(vec![
            Trait::Copy,
            Trait::EnumDisplay,
            Trait::EnumValue,
            Trait::Hash,
        ]);

        traits.list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        match t {
            Trait::EnumValue => imp::enum_value::enum_value(self, t),

            _ => imp::any(self, t),
        }
    }
}

///
/// EnumValueVariant
///

#[derive(Clone, Debug, FromMeta)]
pub struct EnumValueVariant {
    #[darling(default = EnumValueVariant::unspecified_ident)]
    pub name: Ident,

    #[darling(default)]
    pub value: i64,

    #[darling(default)]
    pub default: bool,

    #[darling(default)]
    pub unspecified: bool,
}

impl EnumValueVariant {
    fn unspecified_ident() -> Ident {
        format_ident!("Unspecified")
    }
}

impl Node for EnumValueVariant {
    fn expand(&self) -> TokenStream {
        let mut q = quote!();

        // default
        if self.default {
            q.extend(quote!(#[default]));
        }

        // name
        let name = if self.unspecified {
            Self::unspecified_ident()
        } else {
            self.name.clone()
        };

        // quote
        q.extend(quote! (#name));

        q
    }
}

impl Schemable for EnumValueVariant {
    fn schema(&self) -> TokenStream {
        let Self {
            default,
            unspecified,
            ..
        } = self;

        // quote
        let name = quote_one(&self.name, to_string);
        let value = self.value;

        quote! {
            ::mimic::orm::schema::node::EnumValueVariant {
                name: #name,
                value : #value,
                default: #default,
                unspecified: #unspecified,
            }
        }
    }
}
