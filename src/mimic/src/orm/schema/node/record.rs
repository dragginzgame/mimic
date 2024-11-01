use crate::orm::schema::{
    node::{Def, FieldList, MacroNode, ValidateNode, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Record
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    pub def: Def,
    pub fields: FieldList,
}

impl MacroNode for Record {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Record {}

impl VisitableNode for Record {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        self.fields.accept(v);
    }
}
