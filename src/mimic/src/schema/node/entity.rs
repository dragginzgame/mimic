use crate::{
    common::types::StoreType,
    schema::{
        build::schema_read,
        node::{
            Def, Field, MacroNode, SortKey, Store, Type, TypeNode, ValidateNode, VisitableNode,
        },
        visit::Visitor,
    },
    types::ErrorTree,
};
use serde::{Deserialize, Serialize};
use std::ops::Not;

///
/// Entity
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub def: Def,
    pub store: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sort_keys: Vec<SortKey>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indexes: Vec<EntityIndex>,

    pub fields: Vec<Field>,

    #[serde(default, skip_serializing_if = "Type::skip_serializing")]
    pub ty: Type,
}

impl Entity {
    // get_field
    #[must_use]
    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
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
        match schema.try_get_node_as::<Store>(&self.store) {
            Ok(store) if !matches!(store.ty, StoreType::Data) => errs.add("store is not type Data"),
            Ok(_) => {}
            Err(e) => errs.add(e),
        }

        // ensure there are sort keys
        if self.sort_keys.is_empty() {
            errs.add("entity has no sort keys");
        }

        // check sort keys
        for (i, sk) in self.sort_keys.iter().enumerate() {
            let is_last = i == self.sort_keys.len() - 1;

            // Last sort key must always point to this entity
            if is_last && sk.entity != self.def.path() {
                errs.add(format!(
                    "last sort key '{}' must be '{}'",
                    &sk.entity,
                    self.def.path(),
                ));
            }

            match &sk.field {
                Some(field_name) => {
                    match self.get_field(field_name) {
                        Some(field) => {
                            // no relations
                            if field.value.item.is_relation() {
                                errs.add(format!(
                                    "sort key field '{field_name}' is a relation, which is not allowed",
                                ));
                            }
                        }
                        None => {
                            errs.add(format!("sort key field '{field_name}' does not exist"));
                        }
                    }
                }
                None => {
                    if self.get_field("id").is_some() {
                        errs.add("sort key is missing a field, but entity has an 'id' field — you must specify it explicitly");
                    }
                }
            }
        }

        // indexes
        for index in &self.indexes {
            for field in &index.fields {
                if self.get_field(field).is_none() {
                    errs.add(format!("index field '{field}' does not exist"));
                }
            }
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
        for node in &self.sort_keys {
            node.accept(v);
        }
        for node in &self.indexes {
            node.accept(v);
        }
        for node in &self.fields {
            node.accept(v);
        }
        self.ty.accept(v);
    }
}

///
/// EntityIndex
///

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EntityIndex {
    pub fields: Vec<String>,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub unique: bool,

    pub store: String,
}

impl ValidateNode for EntityIndex {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();
        let schema = schema_read();

        // store
        match schema.try_get_node_as::<Store>(&self.store) {
            Ok(store) if !matches!(store.ty, StoreType::Index) => {
                errs.add("store is not type Index")
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
