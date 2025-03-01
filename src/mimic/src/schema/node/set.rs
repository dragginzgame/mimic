use crate::schema::{
    node::{Def, Item, MacroNode, Type, TypeNode, ValidateNode, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Set
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Set {
    pub def: Def,
    pub item: Item,

    #[serde(default, skip_serializing_if = "Type::skip_serializing")]
    pub ty: Type,
}

impl MacroNode for Set {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TypeNode for Set {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl ValidateNode for Set {}

impl VisitableNode for Set {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        self.item.accept(v);
        self.ty.accept(v);
    }
}
