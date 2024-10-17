use crate::{
    build::schema_read,
    node::{Def, Entity, MacroNode, ValidateNode, VisitableNode},
};
use serde::{Deserialize, Serialize};
use types::ErrorVec;

///
/// EntityExtra
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityExtra {
    pub def: Def,
    pub entity: String,
    pub sources: Vec<EntityExtraSource>,
}

impl MacroNode for EntityExtra {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for EntityExtra {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // fixtures
        errs.add_result(schema_read().check_node::<Entity>(&self.entity));

        errs.result()
    }
}

impl VisitableNode for EntityExtra {
    fn route_key(&self) -> String {
        self.def.path()
    }
}

///
/// EntityExtraSource
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityExtraSource {
    pub name: String,
    pub path: String,
}
