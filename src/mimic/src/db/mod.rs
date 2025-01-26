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
    Db { source: db::DbError },

    #[snafu(transparent)]
    Orm { source: crate::orm::OrmError },

    #[snafu(transparent)]
    Delete { source: query::delete::DeleteError },

    #[snafu(transparent)]
    Load { source: query::load::LoadError },

    #[snafu(transparent)]
    Save { source: query::save::SaveError },

    #[snafu(transparent)]
    Resolver {
        source: query::resolver::ResolverError,
    },
}
