use crate::schema::node::{ValidateNode, VisitableNode};
use serde::{Deserialize, Serialize};
use std::ops::Not;

///
/// Index
///

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Index {
    pub fields: Vec<String>,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub unique: bool,
}

impl ValidateNode for Index {}

impl VisitableNode for Index {
    fn route_key(&self) -> String {
        self.fields.join(", ")
    }
}
