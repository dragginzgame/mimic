pub mod build;
pub mod node;
pub mod types;
pub mod visit;

use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// SchemaError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum SchemaError {
    #[snafu(display("error downcasting schema node: {path}"))]
    DowncastFail { path: String },

    #[snafu(display("{path} is an incorrect node type"))]
    IncorrectNodeType { path: String },

    #[snafu(display("path not found: {path}"))]
    PathNotFound { path: String },
}

impl SchemaError {
    fn downcast_fail(path: &str) -> Self {
        Self::DowncastFail {
            path: path.to_string(),
        }
    }

    fn incorrect_node_type(path: &str) -> Self {
        Self::IncorrectNodeType {
            path: path.to_string(),
        }
    }

    fn path_not_found(path: &str) -> Self {
        Self::PathNotFound {
            path: path.to_string(),
        }
    }
}
