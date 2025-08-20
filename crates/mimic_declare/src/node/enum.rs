use crate::{
    helper::{quote_one, quote_option, quote_slice, to_str_lit},
    imp::TraitStrategy,
    node::{Def, Type, Value},
    schema_traits::{Trait, Traits},
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
/// Enum
///

#[derive(Debug, FromMeta)]
pub struct Enum {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(multiple, rename = "variant")]
    pub variants: Vec<EnumVariant>,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Enum {
    pub fn is_unit_enum(&self) -> bool {
        self.variants.iter().all(|v| v.value.is_none())
    }
}

impl HasIdent for Enum {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }
}

impl HasTraits for Enum {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone().with_type_traits();

        // extra traits
        if self.is_unit_enum() {
            traits.extend(vec![Trait::Copy, Trait::Hash, Trait::PartialOrd]);
        }

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::From => FromTrait::strategy(self),
            Trait::TypeView => TypeViewTrait::strategy(self),
            Trait::ValidateAuto => ValidateAutoTrait::strategy(self),
            Trait::Visitable => VisitableTrait::strategy(self),

            _ => None,
        }
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Sorted => Trait::Sorted.derive_attribute(),
            _ => None,
        }
    }
}

impl HasSchemaPart for Enum {
    fn schema_part(&self) -> TokenStream {
        let def = &self.def.schema_part();
        let variants = quote_slice(&self.variants, EnumVariant::schema_part);
        let ty = &self.ty.schema_part();

        quote! {
            ::mimic::schema::node::Enum {
                def: #def,
                variants: #variants,
                ty: #ty,
            }
        }
    }
}

impl HasTypePart for Enum {
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

impl ToTokens for Enum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}

///
/// EnumVariant
///

#[derive(Clone, Debug, FromMeta)]
pub struct EnumVariant {
    #[darling(default = EnumVariant::unspecified_ident)]
    pub name: Ident,

    #[darling(default)]
    pub value: Option<Value>,

    #[darling(default)]
    pub default: bool,

    #[darling(default)]
    pub unspecified: bool,
}

impl EnumVariant {
    fn unspecified_ident() -> Ident {
        format_ident!("Unspecified")
    }
}

impl HasSchema for Enum {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Enum
    }
}

impl HasSchemaPart for EnumVariant {
    fn schema_part(&self) -> TokenStream {
        let Self {
            default,
            unspecified,
            ..
        } = self;

        // quote
        let name = quote_one(&self.name, to_str_lit);
        let value = quote_option(self.value.as_ref(), Value::schema_part);

        quote! {
            ::mimic::schema::node::EnumVariant {
                name: #name,
                value : #value,
                default: #default,
                unspecified: #unspecified,
            }
        }
    }
}

impl HasTypePart for EnumVariant {
    fn type_part(&self) -> TokenStream {
        let name = if self.unspecified {
            Self::unspecified_ident()
        } else {
            self.name.clone()
        };
        let default_attr = self.default.then(|| quote!(#[default]));

        let body = if let Some(value) = &self.value {
            let value = value.type_part();
            quote!(#name(#value))
        } else {
            quote!(#name)
        };

        quote! {
            #default_attr
            #body
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let name = &self.name;

        if let Some(value) = &self.value {
            let value_view = HasTypePart::view_type_part(value);

            quote! {
                #name(#value_view)
            }
        } else {
            quote! {
                #name
            }
        }
    }
}
