use crate::{
    node::{Def, Field, MacroNode, Type, TypeNode, ValidateNode, VisitableNode},
    visit::Visitor,
};
use serde::Serialize;

///
/// Record
///

#[derive(Clone, Debug, Serialize)]
pub struct Record {
    pub def: Def,
    pub fields: &'static [Field],
    pub ty: Type,
}

impl MacroNode for Record {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TypeNode for Record {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl ValidateNode for Record {}

impl VisitableNode for Record {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        for node in self.fields {
            node.accept(v);
        }
        self.ty.accept(v);
    }
}
