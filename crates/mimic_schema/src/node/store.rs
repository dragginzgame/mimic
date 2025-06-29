use crate::{
    build::schema_read,
    node::{Canister, Def, MacroNode, ValidateNode, VisitableNode},
    types::StoreType,
    visit::Visitor,
};
use mimic_common::{
    error::ErrorTree,
    utils::case::{Case, Casing},
};
use serde::Serialize;

///
/// Store
///
/// A stable IC BTreeMap that stores Entity data
///

#[derive(Clone, Debug, Serialize)]
pub struct Store {
    pub def: Def,
    pub ident: &'static str,
    pub ty: StoreType,
    pub canister: &'static str,
    pub memory_id: u8,
}

impl MacroNode for Store {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Store {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();
        let schema = schema_read();

        // canister
        if let Err(e) = schema.try_get_node_as::<Canister>(self.canister) {
            errs.add(e);
        }

        // ident
        if !self.ident.is_case(Case::UpperSnake) {
            errs.add(format!("ident '{}' must be UPPER_SNAKE_CASE", self.ident));
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
