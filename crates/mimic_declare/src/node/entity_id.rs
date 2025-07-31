use crate::{
    node::Def,
    node_traits::{Trait, TraitStrategy, Traits},
    traits::{HasIdent, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasTypePart},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

///
/// EntityId
///

#[derive(Debug, FromMeta)]
pub struct EntityId {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(multiple, rename = "key")]
    pub keys: Vec<Ident>,

    #[darling(default)]
    pub traits: Traits,
}

impl HasIdent for EntityId {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }
}

impl HasSchema for EntityId {}

impl HasSchemaPart for EntityId {}

impl HasTraits for EntityId {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone().with_default_traits();
        traits.extend(vec![Trait::Copy, Trait::EntityIdKind, Trait::Into]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::node_traits::*;

        match t {
            Trait::Into => IntoTrait::strategy(self),

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

impl HasTypePart for EntityId {
    fn type_part(&self) -> TokenStream {
        let ident = self.ident();
        let keys = self.keys.iter().map(ToTokens::to_token_stream);

        quote! {
            pub enum #ident {
                #(#keys,)*
            }
        }
    }
}

impl ToTokens for EntityId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
