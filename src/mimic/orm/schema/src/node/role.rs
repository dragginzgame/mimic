use crate::{
    build::schema_read,
    node::{Def, MacroNode, Permission, ValidateNode, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use types::ErrorVec;

///
/// Role
///

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Role {
    pub def: Def,
    pub parent: Option<String>,
    pub permissions: Vec<String>,
}

impl MacroNode for Role {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Role {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // parent check
        if let Some(parent) = &self.parent {
            errs.add_result(schema_read().check_node::<Self>(parent));
        }

        // permissions check
        let mut seen = HashSet::<String>::default();
        for perm in &self.permissions {
            errs.add_result(schema_read().check_node::<Permission>(perm));
            if !seen.insert(perm.clone()) {
                errs.push(format!("duplicate value for permission '{perm}'"));
            }
        }

        errs.result()
    }
}

impl VisitableNode for Role {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
    }
}
