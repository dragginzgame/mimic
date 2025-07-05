use crate::{
    helper::{quote_one, quote_option, quote_slice, to_str_lit},
    node::{Def, Type, Value},
    node_traits::{self, Imp, Trait, Traits},
    traits::{Macro, Schemable},
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
    pub fn has_default(&self) -> bool {
        self.variants.iter().any(|v| v.default)
    }

    pub fn is_unit_enum(&self) -> bool {
        self.variants.iter().all(|v| v.value.is_none())
    }
}

impl Macro for Enum {
    fn def(&self) -> &Def {
        &self.def
    }

    fn macro_body(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;

        // quote
        let variants = &self.variants;
        quote! {
            pub enum #ident {
                #(#variants,)*
            }
        }
    }

    fn macro_extra(&self) -> TokenStream {
        quote!()
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();

        // extra traits
        if self.is_unit_enum() {
            traits.extend(vec![Trait::Copy, Trait::Hash, Trait::PartialOrd]);
        }
        if self.has_default() {
            traits.add(Trait::Default);
        }

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            //     Trait::TypeView => node_traits::TypeViewTrait::tokens(self, t),
            Trait::ValidateAuto => node_traits::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => node_traits::VisitableTrait::tokens(self, t),

            _ => node_traits::any(self, t),
        }
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Sorted => Trait::Sorted.derive_attribute(),
            _ => None,
        }
    }
}

impl Schemable for Enum {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let variants = quote_slice(&self.variants, EnumVariant::schema);
        let ty = &self.ty.schema();

        quote! {
            ::mimic::schema::node::Schemable::Enum(::mimic::schema::node::Enum {
                def: #def,
                variants: #variants,
                ty: #ty,
            })
        }
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

impl ToTokens for EnumVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
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
        tokens.extend(if let Some(value) = &self.value {
            quote! (#name(#value))
        } else {
            quote! (#name)
        });
    }
}

impl Schemable for EnumVariant {
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
