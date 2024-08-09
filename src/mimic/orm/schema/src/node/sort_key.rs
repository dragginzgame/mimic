use crate::{
    build::schema_read,
    node::{Entity, ValidateNode, VisitableNode},
};
use serde::{Deserialize, Serialize};
use types::ErrorVec;

///
/// SortKey
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortKey {
    pub entity: String,
    pub fields: Vec<String>,
}

impl ValidateNode for SortKey {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // check path
        errs.add_result(schema_read().check_node::<Entity>(&self.entity));

        errs.result()
    }
}

impl VisitableNode for SortKey {
    fn route_key(&self) -> String {
        "sort key".into()
    }
}
