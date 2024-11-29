use crate::orm::{
    schema::{
        build::schema_read,
        node::{Entity, ValidateNode, VisitableNode},
    },
    types::ErrorVec,
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
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // check entity
        match schema_read().try_get_node::<Entity>(&self.entity) {
            Ok(entity) => {
                if let Some(field) = &self.field {
                    if entity.fields.get_field(field).is_none() {
                        errs.add("field '{field}' does not exist on entity '{entity}'");
                    }
                }
            }
            Err(e) => errs.add(e),
        }

        // check field on entity

        errs.result()
    }
}

impl VisitableNode for SortKey {
    fn route_key(&self) -> String {
        "sort key".into()
    }
}
