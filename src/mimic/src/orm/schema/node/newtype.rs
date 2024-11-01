use crate::orm::schema::{
    node::{Def, MacroNode, TypeSanitizer, TypeValidator, ValidateNode, Value, VisitableNode},
    types::PrimitiveType,
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Newtype
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Newtype {
    pub def: Def,
    pub value: Value,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primitive: Option<PrimitiveType>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sanitizers: Vec<TypeSanitizer>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validators: Vec<TypeValidator>,
}

impl MacroNode for Newtype {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Newtype {}

impl VisitableNode for Newtype {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        self.value.accept(v);
        for node in &self.sanitizers {
            node.accept(v);
        }
        for node in &self.validators {
            node.accept(v);
        }
    }
}
