use crate::{
    helper::{quote_one, quote_slice, to_str_lit},
    node::{ArgNumber, Def, Type},
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema, AsType, MacroEmitter, SchemaKind},
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

impl AsMacro for EnumValue {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone().with_type_traits();
        traits.extend(vec![Trait::Copy, Trait::EnumValueKind, Trait::Hash]);

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
    const KIND: SchemaKind = SchemaKind::Full;

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
    fn as_type(&self) -> Option<TokenStream> {
        let ident = self.ident();
        let variants = &self.variants;

        Some(quote! {
            pub enum #ident {
                #(#variants,)*
            }
        })
    }

    fn as_view_type(&self) -> Option<TokenStream> {
        let ident = self.ident();
        let view_ident = self.view_ident();
        let view_variants = self.variants.iter().map(AsType::as_view_type);
        let derives = Self::view_derives();

        Some(quote! {
            #derives
            pub enum #view_ident {
                #(#view_variants,)*
            }

            impl Default for #view_ident {
                fn default() -> Self {
                    #ident::default().to_view()
                }
            }
        })
    }
}

impl ToTokens for EnumValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
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
    const KIND: SchemaKind = SchemaKind::Fragment;

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
    fn as_type(&self) -> Option<TokenStream> {
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

        Some(quote! {
            #default_attr
            #name
        })
    }

    fn as_view_type(&self) -> Option<TokenStream> {
        let name = if self.unspecified {
            Self::unspecified_ident()
        } else {
            self.name.clone()
        };

        Some(quote! {
            #name
        })
    }
}

impl ToTokens for EnumValueVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.as_type());
    }
}
