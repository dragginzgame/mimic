use crate::{
    helper::{quote_one, quote_slice, split_idents, to_path, to_str_lit},
    node::{DataKey, Def, Field, MacroNode, Node, TraitNode, TraitTokens, Type},
    schema::Schemable,
    traits::{self, Imp, Trait, Traits},
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

    #[darling(multiple, rename = "data_key")]
    pub data_keys: Vec<DataKey>,

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
        // let view_ident = format_ident!("{}_View", ident);
        let schema = self.ctor_schema();
        let q = quote! {
             // data model
             #schema
             #derive
             pub struct #ident {
                 #(#fields,)*
             }
             #impls

             // view
        //     pub struct #view_ident {}
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
        let data_keys = quote_slice(&self.data_keys, DataKey::schema);
        let indexes = quote_slice(&self.indexes, EntityIndex::schema);
        let fields = quote_slice(&self.fields, Field::schema);
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
        ]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default if self.has_default() => traits::DefaultTrait::tokens(self, t),
            Trait::EntityKind => traits::EntityKindTrait::tokens(self, t),
            Trait::EntitySearch => traits::EntitySearchTrait::tokens(self, t),
            Trait::EntitySort => traits::EntitySortTrait::tokens(self, t),
            Trait::ValidateAuto => traits::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => traits::VisitableTrait::tokens(self, t),

            _ => traits::any(self, t),
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
