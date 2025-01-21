use crate::imp;
use crate::{
    helper::{quote_one, quote_option, quote_vec, to_path},
    node::{
        AccessPolicy, Def, FieldList, Index, MacroNode, Node, SortKey, Trait, TraitNode, Traits,
    },
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

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

    #[darling(multiple, default, rename = "index")]
    pub indexes: Vec<Index>,

    #[darling(default)]
    pub fields: FieldList,

    #[darling(default)]
    pub acl: Option<EntityAcl>,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for Entity {
    fn expand(&self) -> TokenStream {
        let Self { fields, .. } = self;
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let derive = self.derive_struct();
        let imp = self.imp();
        let q = quote! {
            #schema
            #derive
            pub struct #ident {
                #fields
            }
            #imp
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

impl TraitNode for Entity {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();
        traits.extend(vec![
            Trait::Default,
            Trait::Entity,
            Trait::EntityDyn,
            Trait::EntityFixture,
            Trait::FieldSort,
            Trait::FieldFilter,
            Trait::SortKey,
        ]);

        traits.list()
    }

    fn map_derive(&self, t: Trait) -> bool {
        match t {
            Trait::Default => !self.fields.has_default(),
            _ => true,
        }
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        match t {
            Trait::Default if self.fields.has_default() => imp::default::entity(self, t),
            Trait::Entity => imp::entity::entity(self, t),
            Trait::EntityDyn => imp::entity::entity_dyn(self, t),
            Trait::FieldFilter => imp::record_filter::entity(self, t),
            Trait::FieldSort => imp::record_sort::entity(self, t),
            Trait::Visitable => imp::visitable::entity(self, t),

            _ => imp::any(self, t),
        }
    }
}

impl Schemable for Entity {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let store = quote_one(&self.store, to_path);
        let sort_keys = quote_vec(&self.sort_keys, SortKey::schema);
        let indexes = quote_vec(&self.indexes, Index::schema);
        let fields = &self.fields.schema();
        let acl = quote_option(&self.acl, EntityAcl::schema);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Entity(::mimic::orm::schema::node::Entity {
                def: #def,
                store: #store,
                sort_keys: #sort_keys,
                indexes: #indexes,
                fields: #fields,
                acl: #acl,
            })
        }
    }
}

///
/// EntityAcl
///

#[derive(Debug, Default, FromMeta)]
pub struct EntityAcl {
    #[darling(default)]
    pub load: AccessPolicy,

    #[darling(default)]
    pub save: AccessPolicy,

    #[darling(default)]
    pub delete: AccessPolicy,
}

impl Schemable for EntityAcl {
    fn schema(&self) -> TokenStream {
        let load = &self.load.schema();
        let save = &self.save.schema();
        let delete = &self.delete.schema();

        quote! {
            ::mimic::orm::schema::node::EntityAcl {
                load: #load,
                save: #save,
                delete: #delete,
            }
        }
    }
}
