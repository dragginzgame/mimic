use crate::{
    helper::{quote_one, quote_slice, to_str_lit},
    node::{ArgNumber, Def, Type},
    node_traits::{Trait, TraitStrategy, Traits},
    traits::{
        HasIdent, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasType, HasTypePart,
        SchemaNodeKind,
    },
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

impl HasIdent for EnumValue {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }
}

impl HasSchema for EnumValue {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::EnumValue
    }
}

impl HasSchemaPart for EnumValue {
    fn schema_part(&self) -> TokenStream {
        let def = &self.def.schema_part();
        let variants = quote_slice(&self.variants, EnumValueVariant::schema_part);
        let ty = &self.ty.schema_part();

        quote! {
            ::mimic::schema::node::EnumValue{
                def: #def,
                variants: #variants,
                ty: #ty,
            }
        }
    }
}

impl HasTraits for EnumValue {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone().with_type_traits();
        traits.extend(vec![Trait::Copy, Trait::EnumValueKind, Trait::Hash]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::node_traits::*;

        match t {
            Trait::EnumValueKind => EnumValueKindTrait::strategy(self),
            Trait::From => FromTrait::strategy(self),
            Trait::TypeView => TypeViewTrait::strategy(self),

            _ => None,
        }
    }
}

impl HasTypePart for EnumValue {
    fn type_part(&self) -> TokenStream {
        let ident = self.ident();
        let variants = self.variants.iter().map(HasTypePart::type_part);

        quote! {
            pub enum #ident {
                #(#variants,)*
            }
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let ident = self.ident();
        let view_ident = self.view_ident();
        let view_variants = self.variants.iter().map(HasTypePart::view_type_part);
        let derives = Self::view_derives();

        quote! {
            #derives
            pub enum #view_ident {
                #(#view_variants,)*
            }

            impl Default for #view_ident {
                fn default() -> Self {
                    #ident::default().to_view()
                }
            }
        }
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

impl HasSchemaPart for EnumValueVariant {
    fn schema_part(&self) -> TokenStream {
        let Self {
            default,
            unspecified,
            ..
        } = self;

        // quote
        let name = quote_one(&self.name, to_str_lit);
        let value = self.value.schema_part();

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

impl HasTypePart for EnumValueVariant {
    fn type_part(&self) -> TokenStream {
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

    fn view_type_part(&self) -> TokenStream {
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
