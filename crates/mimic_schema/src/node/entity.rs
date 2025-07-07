use crate::{
    build::schema_read,
    node::{
        DataKey, Def, FieldList, ItemTarget, MacroNode, Store, Type, TypeNode, ValidateNode,
        VisitableNode,
    },
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

    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub data_keys: &'static [DataKey],

    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub indexes: &'static [EntityIndex],

    pub fields: FieldList,
    pub ty: Type,
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

        // ensure there are data keys
        if self.data_keys.is_empty() {
            errs.add("entity has no data keys");
        }

        // check data keys
        for (i, dk) in self.data_keys.iter().enumerate() {
            let is_last = i == self.data_keys.len() - 1;

            // last data key must always point to this entity
            if is_last && dk.entity != self.def.path() {
                errs.add(format!(
                    "last data key '{}' must be '{}'",
                    &dk.entity,
                    self.def.path(),
                ));
            }

            match &dk.field {
                Some(field_name) => {
                    match self.fields.get(field_name) {
                        Some(field) => {
                            // no relations
                            if field.value.item.is_relation() {
                                errs.add(format!(
                                    "data key field '{field_name}' is a relation, which is not allowed",
                                ));
                            }
                        }
                        None => {
                            errs.add(format!("data key field '{field_name}' does not exist"));
                        }
                    }
                }
                None => {
                    if self.fields.get("id").is_some() {
                        errs.add("data key is missing a field, but entity has an 'id' field — you must specify it explicitly");
                    }
                }
            }
        }

        // indexes
        for index in self.indexes {
            for field_name in index.fields {
                // get field, and check relation
                if let Some(field) = self.fields.get(field_name) {
                    if let ItemTarget::Relation(rel) = &field.value.item.target {
                        if let Ok(entity) = schema.try_get_node_as::<Self>(rel) {
                            if entity.data_keys.len() > 1 {
                                errs.add(format!(
                                    "cannot index field '{}': related entity '{}' has multiple data keys (compound primary key)",
                                    field.name, rel
                                ));
                            }
                        }
                    }
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
        for node in self.data_keys {
            node.accept(v);
        }
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
