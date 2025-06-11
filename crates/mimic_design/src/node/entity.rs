use crate::{
    helper::{quote_one, quote_vec, split_idents, to_path, to_string},
    imp::{self, Imp},
    node::{Def, Field, MacroNode, Node, SortKey, Trait, TraitNode, TraitTokens, Traits, Type},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

///
/// Entity
///

#[derive(Debug, FromMeta)]
pub struct Entity {
    #[darling(default, skip)]
    pub def: Def,

    pub store: Path,

    #[darling(multiple, rename = "sk")]
    pub sort_keys: Vec<SortKey>,

    #[darling(multiple, rename = "index")]
    pub indexes: Vec<EntityIndex>,

    #[darling(multiple, rename = "field")]
    pub fields: Vec<Field>,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Entity {
    // has_default
    pub fn has_default(&self) -> bool {
        self.fields.iter().any(|f| f.default.is_some())
    }
}

impl Node for Entity {
    fn expand(&self) -> TokenStream {
        let Self { fields, .. } = self;
        let Def { ident, .. } = &self.def;
        let TraitTokens { derive, impls } = self.trait_tokens();

        // quote
        let schema = self.ctor_schema();
        let q = quote! {
            #schema
            #derive
            pub struct #ident {
                #(#fields,)*
            }
            #impls
        };

        // debug
        if self.def.debug {
            let s = q.to_string();
            return quote!(compile_error!(#s););
        }

        q
    }
}

impl MacroNode for Entity {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Entity {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let store = quote_one(&self.store, to_path);
        let sort_keys = quote_vec(&self.sort_keys, SortKey::schema);
        let indexes = quote_vec(&self.indexes, EntityIndex::schema);
        let fields = quote_vec(&self.fields, Field::schema);
        let ty = &self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Entity(::mimic::schema::node::Entity {
                def: #def,
                store: #store,
                sort_keys: #sort_keys,
                indexes: #indexes,
                fields: #fields,
                ty: #ty,
            })
        }
    }
}

impl TraitNode for Entity {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();
        traits.extend(vec![
            Trait::Default,
            Trait::EntityKind,
            Trait::EntityFixture,
            Trait::EntitySearch,
            Trait::EntitySort,
            Trait::Searchable,
        ]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default if self.has_default() => imp::DefaultTrait::tokens(self, t),
            Trait::EntityKind => imp::EntityKindTrait::tokens(self, t),
            Trait::EntitySearch => imp::EntitySearchTrait::tokens(self, t),
            Trait::EntitySort => imp::EntitySortTrait::tokens(self, t),
            Trait::FormatSortKey => imp::FormatSortKeyTrait::tokens(self, t),
            Trait::ValidateAuto => imp::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => imp::VisitableTrait::tokens(self, t),

            _ => imp::any(self, t),
        }
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default => Trait::Default.derive_attribute(),
            _ => None,
        }
    }
}

///
/// EntityIndex
///

#[derive(Debug, FromMeta)]
pub struct EntityIndex {
    #[darling(default, map = "split_idents")]
    pub fields: Vec<Ident>,

    #[darling(default)]
    pub unique: bool,

    pub store: Path,
}

impl Schemable for EntityIndex {
    fn schema(&self) -> TokenStream {
        let fields = quote_vec(&self.fields, to_string);
        let unique = &self.unique;
        let store = quote_one(&self.store, to_path);

        quote! {
            ::mimic::schema::node::EntityIndex {
                fields: #fields,
                unique: #unique,
                store: #store,
            }
        }
    }
}
