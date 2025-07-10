use crate::{
    helper::{quote_one, quote_slice, to_str_lit},
    node::{ArgNumber, Def, Type},
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema, AsType},
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

impl AsMacro for EnumValue {
    fn def(&self) -> &Def {
        &self.def
    }

    fn macro_extra(&self) -> TokenStream {
        self.as_view_type()
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone().with_type_traits();

        traits.extend(vec![Trait::Copy, Trait::EnumValueKind, Trait::Hash]);

        // extra traits
        if self.has_default() {
            traits.add(Trait::Default);
        }

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        use crate::node_traits::*;

        match t {
            Trait::EnumValueKind => EnumValueKindTrait::tokens(self),
            Trait::From => FromTrait::tokens(self),
            Trait::TypeView => TypeViewTrait::tokens(self),

            _ => None,
        }
    }
}

impl AsSchema for EnumValue {
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

impl AsType for EnumValue {
    fn as_type(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;
        let variants = &self.variants;

        // quote
        quote! {
            pub enum #ident {
                #(#variants,)*
            }
        }
    }

    fn as_view_type(&self) -> TokenStream {
        let view_ident = self.def.view_ident();
        let view_variants = self.variants.iter().map(AsType::as_view_type);
        let derives = Self::basic_derives();

        quote! {
            #derives
            pub enum #view_ident {
                #(#view_variants,)*
            }
        }
    }
}

impl ToTokens for EnumValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.as_type());
    }
}

///
/// EnumValueVariant
///

#[derive(Debug, FromMeta)]
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

impl AsSchema for EnumValueVariant {
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

impl AsType for EnumValueVariant {
    fn as_type(&self) -> TokenStream {
        let name = if self.unspecified {
            Self::unspecified_ident()
        } else {
            self.name.clone()
        };

        let default_attr = if self.default {
            quote!(#[default])
        } else {
            quote!()
        };

        quote! {
            #default_attr
            #name
        }
    }

    fn as_view_type(&self) -> TokenStream {
        let name = if self.unspecified {
            Self::unspecified_ident()
        } else {
            self.name.clone()
        };

        quote! {
            #name
        }
    }
}

impl ToTokens for EnumValueVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.as_type());
    }
}
