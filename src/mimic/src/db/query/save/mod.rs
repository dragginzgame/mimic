pub mod dynamic;
pub mod r#static;

pub use dynamic::{SaveBuilder, SaveExecutor, SaveQuery};
pub use r#static::{ESaveBuilder, ESaveExecutor, ESaveQuery};

use crate::{
    db::{
        db::Db,
        query::{
            resolver::{Resolver, ResolverError},
            DebugContext,
        },
        types::{DataKey, DataValue, Metadata},
    },
    orm::{traits::EntityDyn, OrmError},
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use strum::Display;

///
/// SaveError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum SaveError {
    #[snafu(display("key exists: {key}"))]
    KeyExists { key: DataKey },

    #[snafu(display("key not found: {key}"))]
    KeyNotFound { key: DataKey },

    #[snafu(display("no results found"))]
    NoResultsFound,

    #[snafu(display("validation failed: {path} {source}"))]
    Validation { path: String, source: OrmError },

    #[snafu(transparent)]
    Db { source: crate::db::db::DbError },

    #[snafu(transparent)]
    Orm { source: OrmError },

    #[snafu(transparent)]
    Resolver { source: ResolverError },
}

///
/// SaveMode
///
/// Create  : will only insert a row if it's empty
/// Replace : will change the row regardless of what was there
/// Update  : will only change an existing row
///

#[derive(Display)]
pub enum SaveMode {
    Create,
    Replace,
    Update,
}

// save
fn save(
    db: &Db,
    mode: &SaveMode,
    debug: &DebugContext,
    entity: Box<dyn EntityDyn>,
) -> Result<(), SaveError> {
    //
    // build key / value
    //

    let ck = entity.composite_key_dyn();
    let resolver = Resolver::new(&entity.path_dyn());
    let key = resolver.data_key(&ck).map(DataKey::from)?;

    // debug
    debug.println(&format!(
        "store.{}: {}",
        mode.to_string().to_lowercase(),
        key
    ));

    // serialize
    let data: Vec<u8> = entity.serialize_dyn()?;

    //
    // match mode
    // on Update and Replace compare old and new data
    //

    let now = crate::utils::time::now_secs();
    let store_path = resolver.store()?;
    let result = db.with_store(&store_path, |store| Ok(store.get(&key)))?;

    let (created, modified) = match mode {
        SaveMode::Create => {
            if result.is_some() {
                Err(SaveError::KeyExists { key: key.clone() })?;
            }

            (now, now)
        }

        SaveMode::Update => match result {
            Some(old) => {
                let modified = if data == old.data {
                    old.metadata.modified
                } else {
                    now
                };

                (old.metadata.created, modified)
            }
            None => Err(SaveError::KeyNotFound { key: key.clone() })?,
        },

        SaveMode::Replace => match result {
            Some(old) => {
                let modified = if data == old.data {
                    old.metadata.modified
                } else {
                    now
                };

                (old.metadata.created, modified)
            }
            None => (now, now),
        },
    };

    // insert data
    let value = DataValue {
        data,
        path: entity.path_dyn(),
        metadata: Metadata { created, modified },
    };
    db.with_store_mut(&store_path, |store| {
        store.data.insert(key.clone(), value.clone());

        Ok(())
    })?;

    Ok(())
}
