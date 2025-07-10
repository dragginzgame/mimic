use crate::{
    build::schema_read,
    node::{Def, Field, FieldList, MacroNode, Store, Type, TypeNode, ValidateNode, VisitableNode},
    types::StoreType,
    visit::Visitor,
};
use mimic_common::error::ErrorTree;
use serde::Serialize;
use std::ops::Not;

///
/// Entity
///

#[derive(Clone, Debug, Serialize)]
pub struct Entity {
    pub def: Def,
    pub store: &'static str,
    pub primary_key: &'static str,

    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub indexes: &'static [EntityIndex],

    pub fields: FieldList,
    pub ty: Type,
}

impl Entity {
    #[must_use]
    pub fn get_pk_field(&self) -> &Field {
        self.fields.get(self.primary_key).expect("pk field exists")
    }
}

impl MacroNode for Entity {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TypeNode for Entity {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl ValidateNode for Entity {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();
        let schema = schema_read();

        // store
        match schema.try_get_node_as::<Store>(self.store) {
            Ok(store) if !matches!(store.ty, StoreType::Data) => errs.add("store is not type Data"),
            Ok(_) => {}
            Err(e) => errs.add(e),
        }

        errs.result()
    }
}

impl VisitableNode for Entity {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        for node in self.indexes {
            node.accept(v);
        }
        self.fields.accept(v);
        self.ty.accept(v);
    }
}

///
/// EntityIndex
///

#[derive(Clone, Debug, Serialize)]
pub struct EntityIndex {
    pub fields: &'static [&'static str],

    #[serde(default, skip_serializing_if = "Not::not")]
    pub unique: bool,

    pub store: &'static str,
}

impl ValidateNode for EntityIndex {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();
        let schema = schema_read();

        // store
        match schema.try_get_node_as::<Store>(self.store) {
            Ok(store) if !matches!(store.ty, StoreType::Index) => {
                errs.add("store is not type Index");
            }
            Ok(_) => {}
            Err(e) => errs.add(e),
        }

        errs.result()
    }
}

impl VisitableNode for EntityIndex {
    fn route_key(&self) -> String {
        self.fields.join(", ")
    }
}
