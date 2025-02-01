use crate::orm::schema::{
    node::{Def, MacroNode, Type, TypeNode, ValidateNode, Value, VisitableNode},
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

    pub ty: Type,
}

impl MacroNode for Newtype {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TypeNode for Newtype {
    fn ty(&self) -> &Type {
        &self.ty
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
        self.ty.accept(v);
    }
}
