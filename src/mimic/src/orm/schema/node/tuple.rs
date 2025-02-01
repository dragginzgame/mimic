use crate::orm::schema::{
    node::{Def, MacroNode, Type, TypeNode, ValidateNode, Value, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Tuple
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tuple {
    pub def: Def,
    pub values: Vec<Value>,
    pub ty: Type,
}

impl MacroNode for Tuple {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TypeNode for Tuple {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl ValidateNode for Tuple {}

impl VisitableNode for Tuple {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        for node in &self.values {
            node.accept(v);
        }
        self.ty.accept(v);
    }
}
