use crate::orm::{
    schema::{
        build::schema_read,
        node::{
            Crud, Def, FieldList, Index, MacroNode, SortKey, Store, ValidateNode, VisitableNode,
        },
        visit::Visitor,
    },
    types::ErrorVec,
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

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crud: Option<Crud>,

    pub fields: FieldList,
}

impl MacroNode for Entity {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Entity {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // store
        errs.add_result(schema_read().check_node::<Store>(&self.store));

        // ensure there are sort keys
        if self.sort_keys.is_empty() {
            errs.add("entity has no sort keys");
        }

        // check sort keys
        for (i, sk) in self.sort_keys.iter().enumerate() {
            if let Some(field_name) = &sk.field {
                // Check if the field exists
                match self.fields.get_field(field_name) {
                    None => errs.add(format!("sort key field '{field_name}' does not exist")),
                    Some(field) => {
                        if i == self.sort_keys.len() - 1 {
                            // Last sort key: must point to this entity and have a default value
                            if sk.entity != self.def.path() {
                                errs.add("the last sort key must point to this entity");
                            }
                            if field.name == "id" && field.value.default.is_none() {
                                errs.add(format!(
                                    "last sort key field '{field_name}' must have a default value"
                                ));
                            }
                        } else if let Some(relation) = &field.value.item.relation {
                            if *relation != sk.entity {
                                errs.add("related entity does not match sort key");
                            }
                        } else {
                            // Non-last sort keys: must be of type relation
                            errs.add(format!(
                                "non-last sort key field '{field_name}' must be of type relation"
                            ));
                        }
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
        if let Some(node) = &self.crud {
            node.accept(v);
        }
        self.fields.accept(v);
    }
}
