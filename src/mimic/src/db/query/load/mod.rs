pub mod builder;
pub mod dynamic;

pub use builder::{LoadBuilder, LoadQuery};
pub use dynamic::{LoadBuilderDyn, LoadQueryDyn};

use crate::db::{
    query::{resolver::Resolver, types::LoadMethod},
    types::{DataKey, DataRow},
    Db,
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("key not found: {key}"))]
    KeyNotFound { key: DataKey },

    #[snafu(display("no results found"))]
    NoResultsFound,

    #[snafu(display("range queries not allowed on composite keys"))]
    RangeNotAllowed,

    #[snafu(transparent)]
    Db { source: crate::db::db::Error },

    #[snafu(transparent)]
    Orm { source: crate::orm::Error },

    #[snafu(transparent)]
    Resolver {
        source: crate::db::query::resolver::Error,
    },
}

///
/// Loader
/// took logic from both Load types and stuck it here
///

pub struct Loader<'a> {
    db: &'a Db,
    resolver: &'a Resolver,
}

impl<'a> Loader<'a> {
    #[must_use]
    pub const fn new(db: &'a Db, resolver: &'a Resolver) -> Self {
        Loader { db, resolver }
    }

    // load
    pub fn load(&self, method: &LoadMethod) -> Result<Box<dyn Iterator<Item = DataRow>>, Error> {
        match method {
            LoadMethod::All | LoadMethod::Only => {
                let start = self.resolver.data_key(&[])?;
                let end = start.create_upper_bound();

                self.query_range(start, end)
            }

            LoadMethod::One(ck) => {
                let key = self.resolver.data_key(ck)?;
                let res = self.query_data_key(key)?;

                Ok(Box::new(std::iter::once(res)))
            }

            LoadMethod::Many(cks) => {
                let keys = cks
                    .iter()
                    .map(|ck| self.resolver.data_key(ck))
                    .collect::<Result<Vec<_>, _>>()?;

                let rows = keys
                    .into_iter()
                    .map(|key| self.query_data_key(key))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Box::new(rows.into_iter()))
            }

            LoadMethod::Prefix(prefix) => {
                let start = self.resolver.data_key(prefix)?;
                let end = start.create_upper_bound();

                self.query_range(start, end)
            }

            LoadMethod::Range(start_ck, end_ck) => {
                let start = self.resolver.data_key(start_ck)?;
                let end = self.resolver.data_key(end_ck)?;

                self.query_range(start, end)
            }
        }
    }

    // query_data_key
    fn query_data_key(&self, key: DataKey) -> Result<DataRow, Error> {
        let store_path = &self.resolver.store()?;
        let value = self
            .db
            .with_store(store_path, |store| Ok(store.data.get(&key)))?
            .ok_or_else(|| Error::KeyNotFound { key: key.clone() })?;

        Ok(DataRow { key, value })
    }

    // query_range
    fn query_range(
        &self,
        start: DataKey,
        end: DataKey,
    ) -> Result<Box<dyn Iterator<Item = DataRow>>, Error> {
        self.db
            .with_store(&self.resolver.store()?, |store| {
                // Collect data into a Vec to own it
                let rows = store
                    .data
                    .range(start..=end)
                    .map(|(key, value)| DataRow { key, value })
                    .collect::<Vec<_>>();

                // Return the iterator over the Vec
                Ok(Box::new(rows.into_iter()) as Box<dyn Iterator<Item = DataRow>>)
            })
            .map_err(Error::from)
    }
}
