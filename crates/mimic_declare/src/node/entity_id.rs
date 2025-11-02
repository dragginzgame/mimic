use crate::prelude::*;

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

impl HasDef for EntityId {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl HasSchema for EntityId {}

impl HasSchemaPart for EntityId {}

impl HasTraits for EntityId {
    fn traits(&self) -> TraitList {
        let mut traits = self.traits.clone().with_default_traits();
        traits.extend(vec![Trait::Copy, Trait::EntityIdKind, Trait::Into]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

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

impl HasType for EntityId {
    fn type_part(&self) -> TokenStream {
        let ident = self.def.ident();
        let keys = self.keys.iter().map(ToTokens::to_token_stream);

        quote! {
            pub enum #ident {
                #(#keys),*
            }
        }
    }
}

impl HasTypeViews for EntityId {}

impl ToTokens for EntityId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
