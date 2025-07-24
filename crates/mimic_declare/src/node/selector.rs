use crate::{
    helper::{quote_one, quote_slice, to_path, to_str_lit},
    node::{Arg, Def},
    node_traits::{Trait, Traits},
    traits::{
        HasIdent, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasTypePart, SchemaNodeKind,
    },
};
use darling::FromMeta;
use proc_macro2::TokenStream;
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
    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::new().with_default_traits();
        traits.add(Trait::Into);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        use crate::node_traits::*;

        match t {
            Trait::Into => IntoTrait::tokens(self),

            _ => None,
        }
    }
}

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
    pub name: Ident,
    pub value: Arg,

    #[darling(default)]
    pub default: bool,
}

impl ToTokens for SelectorVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;

        let attr = if self.default {
            quote!(#[default])
        } else {
            quote!()
        };

        tokens.extend(quote! {
            #attr
            #name
        });
    }
}

impl HasSchemaPart for SelectorVariant {
    fn schema_part(&self) -> TokenStream {
        let name = quote_one(&self.name, to_str_lit);
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
