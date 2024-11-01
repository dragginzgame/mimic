use crate::orm::schema::{
    node::{Def, MacroNode, ValidateNode, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Permission
///

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Permission {
    pub def: Def,
}

impl MacroNode for Permission {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Permission {}

impl VisitableNode for Permission {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
    }
}
