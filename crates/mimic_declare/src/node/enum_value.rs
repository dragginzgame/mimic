use crate::{
    helper::{quote_one, quote_slice, to_str_lit},
    node::{ArgNumber, Def, Type},
    node_traits::{self, Imp, Trait, Traits},
    traits::{Macro, Schemable},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
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

impl Macro for EnumValue {
    fn def(&self) -> &Def {
        &self.def
    }

    fn macro_body(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;
        let variants = &self.variants;

        // quote
        quote! {
            pub enum #ident {
                #(#variants,)*
            }
        }
    }

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
            Trait::EnumValueKind => node_traits::EnumValueTrait::tokens(self, t),

            _ => node_traits::any(self, t),
        }
    }
}

impl Schemable for EnumValue {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let variants = quote_slice(&self.variants, EnumValueVariant::schema);
        let ty = &self.ty.schema();

        quote! {
            ::mimic::schema::node::Schemable::EnumValue(
                ::mimic::schema::node::EnumValue{
                    def: #def,
                    variants: #variants,
                    ty: #ty,
                }
            )
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

impl ToTokens for EnumValueVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // default
        if self.default {
            tokens.extend(quote!(#[default]));
        }

        // name
        let name = if self.unspecified {
            Self::unspecified_ident()
        } else {
            self.name.clone()
        };

        tokens.extend(quote! (#name));
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
