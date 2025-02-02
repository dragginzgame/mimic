pub mod dynamic;
pub mod r#static;

pub use dynamic::{SaveBuilderDyn, SaveExecutorDyn, SaveQueryDyn};
pub use r#static::{SaveBuilder, SaveExecutor, SaveQuery};

use crate::{
    orm::{traits::EntityDyn, OrmError},
    query::{
        resolver::{Resolver, ResolverError},
        DebugContext,
    },
    store::{
        types::{DataKey, DataValue, Metadata},
        StoreLocal,
    },
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use strum::Display;

///
/// SaveError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
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
    OrmError { source: OrmError },

    #[snafu(transparent)]
    ResolverError { source: ResolverError },
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
fn save<'a>(
    store: StoreLocal,
    mode: &SaveMode,
    debug: &DebugContext,
    entity: Box<dyn EntityDyn + 'a>,
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
    let result = store.with_borrow(|store| store.get(&key));

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
    store.with_borrow_mut(|store| {
        store.data.insert(key.clone(), value.clone());
    });

    Ok(())
}
