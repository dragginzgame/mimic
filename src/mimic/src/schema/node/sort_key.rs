use crate::{
    schema::{
        build::schema_read,
        node::{Entity, ValidateNode, VisitableNode},
    },
    types::ErrorTree,
};
use serde::{Deserialize, Serialize};

///
/// SortKey
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortKey {
    pub entity: String,
    pub field: Option<String>,
}

impl ValidateNode for SortKey {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();

        // check entity
        errs.add_result(schema_read().check_node_as::<Entity>(&self.entity));

        errs.result()
    }
}

impl VisitableNode for SortKey {
    fn route_key(&self) -> String {
        "sort key".into()
    }
}
