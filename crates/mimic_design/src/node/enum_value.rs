use crate::{
    helper::{quote_one, quote_slice, to_str_lit},
    node::{ArgNumber, Def, MacroNode, Node, TraitNode, TraitTokens, Type},
    schema::Schemable,
    traits::{self, Imp, Trait, Traits},
};
use darling::FromMeta;
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
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl EnumValue {
    pub fn has_default(&self) -> bool {
        self.variants.iter().any(|v| v.default)
    }
}

impl Node for EnumValue {
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

impl MacroNode for EnumValue {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for EnumValue {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let variants = quote_slice(&self.variants, EnumValueVariant::schema);
        let ty = &self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::EnumValue(
                ::mimic::schema::node::EnumValue{
                    def: #def,
                    variants: #variants,
                    ty: #ty,
                }
            )
        }
    }
}

impl TraitNode for EnumValue {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();
        traits.extend(vec![Trait::Copy, Trait::EnumValueKind, Trait::Hash]);

        // extra traits
        if self.has_default() {
            traits.add(Trait::Default);
        }

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::EnumValueKind => traits::EnumValueTrait::tokens(self, t),

            _ => traits::any(self, t),
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

    pub value: ArgNumber,

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
        let name = quote_one(&self.name, to_str_lit);
        let value = self.value.schema();

        quote! {
            ::mimic::schema::node::EnumValueVariant {
                name: #name,
                value : #value,
                default: #default,
                unspecified: #unspecified,
            }
        }
    }
}
