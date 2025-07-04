use crate::{
    helper::{quote_one, quote_slice, split_idents, to_path, to_str_lit},
    node::{DataKey, Def, FieldList, Type},
    node_traits::{self, Imp, Trait, Traits},
    traits::{MacroNode, SchemaNode},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Path};

///
/// Entity
///

#[derive(Debug, FromMeta)]
pub struct Entity {
    #[darling(default, skip)]
    pub def: Def,

    pub store: Path,

    #[darling(multiple, rename = "data_key")]
    pub data_keys: Vec<DataKey>,

    #[darling(multiple, rename = "index")]
    pub indexes: Vec<EntityIndex>,

    #[darling(default)]
    pub fields: FieldList,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { fields, .. } = self;
        let Def { ident, .. } = &self.def;

        // view
        let view_ident = &self.def.view_ident();
        let view = self.fields.type_view_fields(view_ident);

        // quote
        tokens.extend(quote! {
            // data model
            #self
            pub struct #ident {
                #fields
            }

            // view
            #view
        });
    }
}

impl MacroNode for Entity {
    fn def(&self) -> &Def {
        &self.def
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();
        traits.extend(vec![
            Trait::Default,
            Trait::EntityKind,
            Trait::EntityFixture,
            Trait::EntitySearch,
            Trait::EntitySort,
        ]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default if self.fields.has_default() => {
                node_traits::DefaultTrait::tokens(self, t)
            }
            Trait::EntityKind => node_traits::EntityKindTrait::tokens(self, t),
            Trait::EntitySearch => node_traits::EntitySearchTrait::tokens(self, t),
            Trait::EntitySort => node_traits::EntitySortTrait::tokens(self, t),
            Trait::ValidateAuto => node_traits::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => node_traits::VisitableTrait::tokens(self, t),

            _ => node_traits::any(self, t),
        }
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default => Trait::Default.derive_attribute(),
            _ => None,
        }
    }
}

impl SchemaNode for Entity {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let store = quote_one(&self.store, to_path);
        let data_keys = quote_slice(&self.data_keys, DataKey::schema);
        let indexes = quote_slice(&self.indexes, EntityIndex::schema);
        let fields = &self.fields.schema();
        let ty = &self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Entity(::mimic::schema::node::Entity {
                def: #def,
                store: #store,
                data_keys: #data_keys,
                indexes: #indexes,
                fields: #fields,
                ty: #ty,
            })
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

impl SchemaNode for EntityIndex {
    fn schema(&self) -> TokenStream {
        let fields = quote_slice(&self.fields, to_str_lit);
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
