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

        // sort keys
        if self.sort_keys.is_empty() {
            errs.add("entity has no sort keys");
        } else if let Some(last_key) = self.sort_keys.last() {
            // last sort key
            if last_key.entity != self.def.path() {
                errs.add("the last sort key must point to this entity");
            }
        }

        for sk in &self.sort_keys {
            if let Some(field) = &sk.field {
                if self.fields.get_field(field).is_none() {
                    errs.add(format!("sort key field '{field}' does not exist",));
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
