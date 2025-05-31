use crate::schema::{
    node::{Def, MacroNode, Type, TypeNode, ValidateNode, VisitableNode},
    visit::Visitor,
};
use mimic_common::types::PrimitiveType;
use serde::{Deserialize, Serialize};

///
/// Primitive
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Primitive {
    pub def: Def,
    pub variant: PrimitiveType,

    #[serde(default, skip_serializing_if = "Type::skip_serializing")]
    pub ty: Type,
}

impl MacroNode for Primitive {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TypeNode for Primitive {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl ValidateNode for Primitive {}

impl VisitableNode for Primitive {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        self.ty.accept(v);
    }
}
