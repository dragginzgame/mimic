use crate::orm::schema::node::{ValidateNode, VisitableNode};
use serde::{Deserialize, Serialize};

///
/// Index
///

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Index {
    pub fields: Vec<String>,
}

impl ValidateNode for Index {}

impl VisitableNode for Index {
    fn route_key(&self) -> String {
        self.fields.join(", ")
    }
}
