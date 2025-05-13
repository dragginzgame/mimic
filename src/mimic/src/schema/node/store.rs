use crate::{
    schema::{
        node::{Def, MacroNode, ValidateNode, VisitableNode},
        visit::Visitor,
    },
    types::ErrorTree,
    utils::case::{Case, Casing},
};
use serde::{Deserialize, Serialize};

///
/// Store
///
/// A stable IC BTreeMap that stores Entity data
/// the name should be snake_case to keep the API consistent
///
/// crud : the default crud for the entire store, can be overwritten by Entity
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Store {
    pub def: Def,
    pub ident: String,
    pub canister: String,
}

impl MacroNode for Store {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Store {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();

        // ident
        if !self.ident.is_case(Case::UpperSnake) {
            errs.add("store ident '{}' must be UPPER_SNAKE_CASE");
        }

        errs.result()
    }
}

impl VisitableNode for Store {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
    }
}
