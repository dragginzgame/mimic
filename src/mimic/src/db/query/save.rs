use crate::{
    db::{
        query::{DebugContext, Error as QueryError, Resolver},
        types::{DataKey, DataRow, DataValue, EntityRow, Metadata},
        Db,
    },
    orm::{
        traits::{Entity, EntityDyn},
        Error as OrmError,
    },
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::mem;
use strum::Display;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("key exists: {key}"))]
    KeyExists { key: DataKey },

    #[snafu(display("key not found: {key}"))]
    KeyNotFound { key: DataKey },

    #[snafu(display("no results found"))]
    NoResultsFound,

    #[snafu(display("validation failed: {path} {source}"))]
    Validation { path: String, source: OrmError },

    #[snafu(transparent)]
    Db { source: crate::db::db::Error },

    #[snafu(transparent)]
    Orm { source: OrmError },

    #[snafu(transparent)]
    Resolver { source: super::resolver::Error },
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

///
/// SaveOptions
///

pub struct SaveOptions {
    pub sanitize: bool,
    pub validate: bool,
}

impl Default for SaveOptions {
    fn default() -> Self {
        Self {
            sanitize: true,
            validate: true,
        }
    }
}

///
/// SaveBuilderConfig
///

pub struct SaveBuilderConfig {
    mode: SaveMode,
    debug: DebugContext,
    options: SaveOptions,
}

///
/// SaveBuilder
///

pub struct SaveBuilder {
    config: SaveBuilderConfig,
}

impl SaveBuilder {
    // new
    #[must_use]
    pub fn new(mode: SaveMode) -> Self {
        Self {
            config: SaveBuilderConfig {
                mode,
                debug: DebugContext::default(),
                options: SaveOptions::default(),
            },
        }
    }

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.config.debug.enable();
        self
    }

    // from_data
    pub fn from_data<E: Entity + 'static>(self, data: &[u8]) -> Result<SaveQuery, QueryError> {
        let entity: E = crate::orm::deserialize(data)?;

        Ok(SaveQuery::new(self, vec![Box::new(entity)]))
    }

    // from_entity
    pub fn from_entity<E: EntityDyn + 'static>(self, entity: E) -> SaveQuery {
        SaveQuery::new(self, vec![Box::new(entity)])
    }

    // from_entities
    #[must_use]
    pub fn from_entities<E: EntityDyn + 'static>(self, entities: Vec<E>) -> SaveQuery {
        let boxed_entities = entities
            .into_iter()
            .map(|entity| Box::new(entity) as Box<dyn EntityDyn>)
            .collect();

        SaveQuery::new(self, boxed_entities)
    }

    // from_entity_dynamic
    #[must_use]
    pub fn from_entity_dynamic(self, entity: Box<dyn EntityDyn>) -> SaveQuery {
        SaveQuery::new(self, vec![entity])
    }

    // from_entities_dynamic
    #[must_use]
    pub fn from_entities_dynamic(self, entities: Vec<Box<dyn EntityDyn>>) -> SaveQuery {
        SaveQuery::new(self, entities)
    }
}

///
/// SaveQuery
///

pub struct SaveQuery {
    config: SaveBuilderConfig,
    entities: Vec<Box<dyn EntityDyn>>,
}

impl SaveQuery {
    #[must_use]
    pub fn new(prev: SaveBuilder, entities: Vec<Box<dyn EntityDyn>>) -> Self {
        Self {
            config: prev.config,
            entities,
        }
    }

    // execute
    pub fn execute(&mut self, db: &Db) -> Result<SaveResponse, QueryError> {
        // Temporarily take the entities out of self to avoid multiple mutable borrows
        let mut entities = mem::take(&mut self.entities);

        // get results
        let mut results = Vec::new();
        for entity in &mut entities {
            let data_row = self.execute_one(db, &mut **entity)?;
            results.push(data_row);
        }

        Ok(SaveResponse::new(results))
    }

    // execute_one
    fn execute_one(&self, db: &Db, entity: &mut dyn EntityDyn) -> Result<DataRow, Error> {
        let mode = &self.config.mode;

        //
        // firstly mutate the entity so the ids are generated
        // and relevant data is sanitized
        //

        if self.config.options.sanitize {
            let mut adapter = crate::orm::visit::EntityAdapterMut(entity);
            crate::orm::sanitize(&mut adapter);
        }

        //
        // build key / value
        //

        let ck = entity.composite_key_dyn();
        let resolver = Resolver::new(&entity.path_dyn());
        let key = resolver.data_key(&ck).map(DataKey::from)?;

        // debug
        self.config.debug.println(&format!(
            "store.{}: {}",
            mode.to_string().to_lowercase(),
            key
        ));

        // validate
        if self.config.options.validate {
            let adapter = crate::orm::visit::EntityAdapter(entity);
            crate::orm::validate(&adapter).map_err(|e| Error::Validation {
                path: entity.path_dyn(),
                source: e,
            })?;
        }

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
                    Err(Error::KeyExists { key: key.clone() })?;
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
                None => Err(Error::KeyNotFound { key: key.clone() })?,
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

        // data row to return
        let result = DataRow::new(key, value);

        Ok(result)
    }
}

///
/// SaveResponse
///

pub struct SaveResponse {
    pub rows: Vec<DataRow>,
}

impl SaveResponse {
    #[must_use]
    pub const fn new(rows: Vec<DataRow>) -> Self {
        Self { rows }
    }

    // ok
    pub const fn ok(&self) -> Result<(), QueryError> {
        Ok(())
    }

    // query_row
    pub fn query_row(&self) -> Result<DataRow, QueryError> {
        let res = self
            .rows
            .first()
            .cloned()
            .map(Into::into)
            .ok_or(Error::NoResultsFound)?;

        Ok(res)
    }

    // query_rows
    pub fn query_rows(self) -> impl Iterator<Item = DataRow> {
        self.rows.into_iter().map(Into::into)
    }

    // entity_row
    pub fn entity_row<E: Entity>(self) -> Result<EntityRow<E>, QueryError> {
        let row = self.rows.first().ok_or(Error::NoResultsFound)?.clone();

        let entity_row: EntityRow<E> = row.try_into()?;

        Ok(entity_row)
    }

    // entity_rows
    pub fn entity_rows<E: Entity>(self) -> impl Iterator<Item = Result<EntityRow<E>, QueryError>> {
        self.rows
            .into_iter()
            .map(|row| row.try_into().map_err(QueryError::from))
    }

    // entity
    pub fn entity<E: Entity>(self) -> Result<E, QueryError> {
        let row_ref = self.rows.first().ok_or(Error::NoResultsFound)?;
        let entity_row: EntityRow<E> = row_ref.clone().try_into()?;

        Ok(entity_row.value.entity)
    }

    // entities
    pub fn entities<E: Entity>(self) -> impl Iterator<Item = Result<E, QueryError>> {
        self.rows.into_iter().map(|row| {
            row.try_into()
                .map(|row: EntityRow<E>| row.value.entity)
                .map_err(QueryError::from)
        })
    }
}
