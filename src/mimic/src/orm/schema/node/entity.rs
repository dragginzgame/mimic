use crate::orm::{
    schema::{
        build::schema_read,
        node::{Crud, Def, FieldList, MacroNode, SortKey, Store, ValidateNode, VisitableNode},
        visit::Visitor,
    },
    types::ErrorVec,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

///
/// Entity
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub def: Def,
    pub store: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sort_keys: Vec<SortKey>,

    pub primary_keys: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crud: Option<Crud>,

    pub fields: FieldList,
}

impl Entity {
    // is_relatable
    #[must_use]
    pub fn is_relatable(&self) -> bool {
        self.primary_keys.len() == 1
    }
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
        for sk in &self.sort_keys {
            errs.add_result(schema_read().check_node::<Self>(&sk.entity));
        }

        // primary key check
        let mut seen = HashSet::<String>::default();
        for pk in &self.primary_keys {
            if self.fields.get_field(pk).is_none() {
                errs.add(format!("primary key field '{pk}' not found"));
            }
            if !seen.insert(pk.clone()) {
                errs.push(format!("duplicate value for primary key field '{pk}'"));
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
        if let Some(node) = &self.crud {
            node.accept(v);
        }
        self.fields.accept(v);
    }
}
