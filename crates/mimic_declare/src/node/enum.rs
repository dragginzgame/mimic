use crate::{
    helper::{quote_one, quote_option, quote_slice, to_str_lit},
    node::{Def, Type, Value},
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema, AsType, MacroEmitter},
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

impl AsMacro for Enum {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone().with_type_traits();

        // extra traits
        if self.is_unit_enum() {
            traits.extend(vec![Trait::Copy, Trait::Hash, Trait::PartialOrd]);
        }

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        use crate::node_traits::*;

        match t {
            Trait::From => FromTrait::tokens(self),
            Trait::TypeView => TypeViewTrait::tokens(self),
            Trait::ValidateAuto => ValidateAutoTrait::tokens(self),
            Trait::Visitable => VisitableTrait::tokens(self),

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

impl AsSchema for Enum {
    const EMIT_SCHEMA: bool = true;

    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let variants = quote_slice(&self.variants, EnumVariant::schema);
        let ty = &self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Enum(::mimic::schema::node::Enum {
                def: #def,
                variants: #variants,
                ty: #ty,
            })
        }
    }
}

impl AsType for Enum {
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

impl AsSchema for EnumVariant {
    const EMIT_SCHEMA: bool = false;

    fn schema(&self) -> TokenStream {
        let Self {
            default,
            unspecified,
            ..
        } = self;

        // quote
        let name = quote_one(&self.name, to_str_lit);
        let value = quote_option(self.value.as_ref(), Value::schema);

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

impl AsType for EnumVariant {
    fn as_type(&self) -> Option<TokenStream> {
        let name = if self.unspecified {
            Self::unspecified_ident()
        } else {
            self.name.clone()
        };

        let default_attr = self.default.then(|| quote!(#[default]));

        let body = if let Some(value) = &self.value {
            quote!(#name(#value))
        } else {
            quote!(#name)
        };

        Some(quote! {
            #default_attr
            #body
        })
    }

    fn as_view_type(&self) -> Option<TokenStream> {
        let name = &self.name;

        let q = if let Some(value) = &self.value {
            let value_view = AsType::as_view_type(value);

            quote! {
                #name(#value_view)
            }
        } else {
            quote! {
                #name
            }
        };

        Some(q)
    }
}

impl ToTokens for EnumVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.as_type());
    }
}
