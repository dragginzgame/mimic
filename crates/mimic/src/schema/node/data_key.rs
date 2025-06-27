use crate::{
    error::ErrorTree,
    schema::{
        build::schema_read,
        node::{Entity, ValidateNode, VisitableNode},
    },
};
use serde::Serialize;

///
/// DataKey
///

#[derive(Clone, Debug, Serialize)]
pub struct DataKey {
    pub entity: &'static str,
    pub field: Option<&'static str>,
}

impl ValidateNode for DataKey {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();

        // check entity
        errs.add_result(schema_read().check_node_as::<Entity>(self.entity));

        errs.result()
    }
}

impl VisitableNode for DataKey {
    fn route_key(&self) -> String {
        "sort key".into()
    }
}
