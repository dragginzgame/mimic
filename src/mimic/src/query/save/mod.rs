pub mod dynamic;
pub mod generic;

pub use dynamic::{SaveBuilderDyn, SaveExecutorDyn, SaveQueryDyn, SaveResponseDyn};
pub use generic::{SaveBuilder, SaveExecutor, SaveQuery, SaveResponse};

use crate::{
    SerializeError, ThisError, ValidationError,
    db::{
        DbError, DbLocal,
        types::{DataValue, Metadata, SortKey},
    },
    query::{
        DebugContext, QueryError,
        resolver::{Resolver, ResolverError},
    },
    traits::EntityDyn,
};
use candid::CandidType;
use derive_more::Display;
use serde::{Deserialize, Serialize};

///
/// SaveError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum SaveError {
    #[error("key exists: {0}")]
    KeyExists(SortKey),

    #[error("key not found: {0}")]
    KeyNotFound(SortKey),

    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    ResolverError(#[from] ResolverError),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),
}

///
/// SaveMode
///
/// Create  : will only insert a row if it's empty
/// Replace : will change the row regardless of what was there
/// Update  : will only change an existing row
///

#[derive(CandidType, Clone, Copy, Debug, Display, Serialize, Deserialize)]
pub enum SaveMode {
    Create,
    Replace,
    Update,
}

// save
fn save<'a>(
    db: DbLocal,
    mode: SaveMode,
    debug: &DebugContext,
    entity: Box<dyn EntityDyn + 'a>,
) -> Result<(), QueryError> {
    //
    // build key / value
    //

    let ck = entity.composite_key_dyn();
    let resolver = Resolver::new(&entity.path_dyn());
    let key = resolver.data_key(&ck).map_err(SaveError::from)?;

    // debug
    debug.println(&format!("store.{mode}: {key}",));

    // serialize
    let data: Vec<u8> = entity.serialize_dyn().map_err(SaveError::from)?;

    //
    // match mode
    // on Update and Replace compare old and new data
    //

    let now = crate::utils::time::now_secs();
    let store = db
        .with(|db| db.try_get_store(&entity.store_dyn()))
        .map_err(SaveError::from)?;
    let result = store.with_borrow(|store| store.get(&key));

    let (created, modified) = match mode {
        SaveMode::Create => {
            if result.is_some() {
                #[allow(clippy::redundant_clone)]
                Err(SaveError::KeyExists(key.clone()))?;
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
            None => Err(SaveError::KeyNotFound(key.clone()))?,
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

    // prepare data value
    let path = entity.path_dyn();
    let value = DataValue {
        data,
        path,
        metadata: Metadata { created, modified },
    };

    // insert data row
    store.with_borrow_mut(|store| {
        store.data.insert(key.clone(), value.clone());
    });

    Ok(())
}
