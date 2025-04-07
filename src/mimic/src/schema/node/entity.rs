use crate::{
    schema::{
        node::{
            Def, FieldList, Index, MacroNode, SortKey, Type, TypeNode, ValidateNode, VisitableNode,
        },
        visit::Visitor,
    },
    types::ErrorTree,
};
use serde::{Deserialize, Serialize};

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
    pub indexes: Vec<Index>,

    pub fields: FieldList,

    #[serde(default, skip_serializing_if = "Type::skip_serializing")]
    pub ty: Type,
}

impl Entity {
    #[must_use]
    pub fn can_be_relation(&self) -> bool {
        self.sort_keys.last().is_some_and(|k| k.field.is_some())
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
                Some(field_name) => match self.fields.get_field(field_name) {
                    None => {
                        errs.add(format!("sort key field '{field_name}' does not exist"));
                    }
                    Some(field) => {
                        if !is_last {
                            match &field.value.item.relation {
                                Some(relation) if *relation == sk.entity => {}
                                Some(_) => errs.add("related entity does not match sort key"),
                                None => errs.add(format!(
                                    "non-last sort key field '{field_name}' must be of type relation"
                                )),
                            }
                        }
                    }
                },

                None => {
                    // No field set: check if 'id' exists on this entity
                    if self.fields.get_field("id").is_some() {
                        errs.add("sort key is missing a field, but entity has an 'id' field — you must specify it explicitly");
                    }
                }
            }
        }

        // indexes
        for index in &self.indexes {
            for field in &index.fields {
                if self.fields.get_field(field).is_none() {
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
        self.fields.accept(v);
        self.ty.accept(v);
    }
}
