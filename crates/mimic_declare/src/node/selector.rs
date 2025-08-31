use crate::{
    helper::{quote_one, quote_slice, to_path},
    imp::TraitStrategy,
    node::{Arg, Def},
    schema_traits::{Trait, TraitList, Traits},
    traits::{
        HasIdent, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasType, HasTypePart,
        SchemaNodeKind,
    },
};
use darling::FromMeta;
use mimic_common::utils::case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Ident, Path};

///
/// Selector
///

#[derive(Debug, FromMeta)]
pub struct Selector {
    #[darling(default, skip)]
    pub def: Def,

    pub target: Path,

    #[darling(multiple, rename = "variant")]
    pub variants: Vec<SelectorVariant>,
}

impl HasIdent for Selector {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }
}

impl HasSchema for Selector {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Selector
    }
}

impl HasSchemaPart for Selector {
    fn schema_part(&self) -> TokenStream {
        let def = &self.def.schema_part();
        let target = quote_one(&self.target, to_path);
        let variants = quote_slice(&self.variants, SelectorVariant::schema_part);

        quote! {
            ::mimic::schema::node::Selector {
                def: #def,
                target: #target,
                variants: #variants,
            }
        }
    }
}

impl HasTraits for Selector {
    fn traits(&self) -> TraitList {
        let mut traits = Traits::new().with_default_traits();
        traits.add(Trait::Into);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::Into => IntoTrait::strategy(self),

            _ => None,
        }
    }
}

impl HasType for Selector {}

impl HasTypePart for Selector {
    fn type_part(&self) -> TokenStream {
        let ident = self.ident();
        let variants = &self.variants;

        quote! {
            pub enum #ident {
                #(#variants,)*
            }
        }
    }
}

impl ToTokens for Selector {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}

///
/// SelectorVariant
///

#[derive(Clone, Debug, FromMeta)]
pub struct SelectorVariant {
    #[darling(default)]
    pub name: String,

    pub value: Arg,

    #[darling(default)]
    pub default: bool,
}

impl SelectorVariant {
    pub fn ident(&self) -> Ident {
        let camel = self.name.to_case(Case::UpperCamel).trim().to_string();
        let needs_prefix = camel.chars().next().is_some_and(|c| c.is_ascii_digit());

        let ident_str = if needs_prefix {
            format!("V{camel}")
        } else {
            camel
        };

        Ident::new(&ident_str, Span::call_site())
    }
}

impl ToTokens for SelectorVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.ident();

        let attr = if self.default {
            quote!(#[default])
        } else {
            quote!()
        };

        tokens.extend(quote! {
            #attr
            #ident
        });
    }
}

impl HasSchemaPart for SelectorVariant {
    fn schema_part(&self) -> TokenStream {
        let name = &self.name; // just a string
        let value = self.value.schema_part();
        let default = self.default;

        quote! {
            ::mimic::schema::node::SelectorVariant {
                name: #name,
                value : #value,
                default : #default,
            }
        }
    }
}
