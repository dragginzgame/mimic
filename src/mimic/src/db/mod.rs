pub mod cache;
pub mod db;
pub mod query;
pub mod store;
pub mod types;

pub use db::Db;
pub use store::Store;

use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
/// top level error struct for this crate
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Db { source: db::Error },

    #[snafu(transparent)]
    Orm { source: crate::orm::Error },

    #[snafu(transparent)]
    Query { source: query::Error },

    #[snafu(transparent)]
    Delete { source: query::delete::Error },

    #[snafu(transparent)]
    Load { source: query::load::Error },

    #[snafu(transparent)]
    Save { source: query::save::Error },

    #[snafu(transparent)]
    Resolver { source: query::resolver::Error },
}
