use crate::schema::{
    node::{Def, Item, MacroNode, Type, TypeNode, ValidateNode, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Map
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Map {
    pub def: Def,
    pub item: Item,
    pub key: String,

    #[serde(default, skip_serializing_if = "Type::skip_serializing")]
    pub ty: Type,
}

impl MacroNode for Map {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TypeNode for Map {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl ValidateNode for Map {}

impl VisitableNode for Map {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        self.item.accept(v);
        self.ty.accept(v);
    }
}
