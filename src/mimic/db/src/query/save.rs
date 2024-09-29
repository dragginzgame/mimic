use crate::{
    query::{
        types::{EntityRow, QueryRow},
        DebugContext, Error as QueryError, Resolver,
    },
    types::{DataKey, DataRow, DataValue, Metadata},
    Db,
};
use orm::traits::{Entity, EntityDynamic};
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

    #[snafu(display("validaton failed: {path} {source}"))]
    Validation { path: String, source: orm::Error },

    #[snafu(transparent)]
    Db { source: crate::db::Error },

    #[snafu(transparent)]
    Orm { source: orm::Error },

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

pub struct SaveBuilder<'a> {
    db: &'a Db,
    config: SaveBuilderConfig,
}

impl<'a> SaveBuilder<'a> {
    // new
    #[must_use]
    pub fn new(db: &'a Db, mode: SaveMode) -> Self {
        Self {
            db,
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
    pub fn from_data<E: Entity + 'static>(
        self,
        data: &[u8],
    ) -> Result<SaveBuilderResult, QueryError> {
        let entity: E = orm::deserialize(data)?;

        self.execute(vec![Box::new(entity)])
    }

    // from_entity
    pub fn from_entity<E: EntityDynamic + 'static>(
        self,
        entity: E,
    ) -> Result<SaveBuilderResult, QueryError> {
        let res = self.execute(vec![Box::new(entity)])?;

        Ok(res)
    }

    // from_entities
    pub fn from_entities<E: EntityDynamic + 'static>(
        self,
        entities: Vec<E>,
    ) -> Result<SaveBuilderResult, QueryError> {
        let boxed_entities = entities
            .into_iter()
            .map(|entity| Box::new(entity) as Box<dyn EntityDynamic>)
            .collect();

        let res = self.execute(boxed_entities)?;

        Ok(res)
    }

    // from_entity_dynamic
    pub fn from_entity_dynamic(
        self,
        entity: Box<dyn EntityDynamic>,
    ) -> Result<SaveBuilderResult, QueryError> {
        let res = self.execute(vec![entity])?;

        Ok(res)
    }

    // from_entities_dynamic
    pub fn from_entities_dynamic(
        self,
        entities: Vec<Box<dyn EntityDynamic>>,
    ) -> Result<SaveBuilderResult, QueryError> {
        let res = self.execute(entities)?;

        Ok(res)
    }

    // execute
    fn execute(
        self,
        entities: Vec<Box<dyn EntityDynamic>>,
    ) -> Result<SaveBuilderResult, QueryError> {
        let mut executor = SaveBuilderExecutor::new(self, entities);
        let results = executor.execute()?;

        Ok(SaveBuilderResult::new(results))
    }
}

///
/// SaveBuilderExecutor
///

pub struct SaveBuilderExecutor<'a> {
    db: &'a Db,
    config: SaveBuilderConfig,
    entities: Vec<Box<dyn EntityDynamic>>,
}

impl<'a> SaveBuilderExecutor<'a> {
    #[must_use]
    pub fn new(prev: SaveBuilder<'a>, entities: Vec<Box<dyn EntityDynamic>>) -> Self {
        Self {
            db: prev.db,
            config: prev.config,
            entities,
        }
    }

    // execute
    pub fn execute(&mut self) -> Result<Vec<DataRow>, QueryError> {
        // Temporarily take the entities out of self to avoid multiple mutable borrows
        let mut entities = mem::take(&mut self.entities);

        // get results
        let mut results = Vec::new();
        for entity in &mut entities {
            let data_row = self.execute_one(&mut **entity)?;
            results.push(data_row);
        }

        Ok(results)
    }

    // execute_one
    fn execute_one(&self, entity: &mut dyn EntityDynamic) -> Result<DataRow, Error> {
        let mode = &self.config.mode;

        //
        // firstly mutate the entity so the ids are generated
        // and relevant data is sanitized
        //

        if matches!(mode, SaveMode::Create) {
            entity.on_create();
        }
        if self.config.options.sanitize {
            let mut adapter = orm::visit::EntityAdapterMut(entity);
            orm::sanitize(&mut adapter);
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
            let adapter = orm::visit::EntityAdapter(entity);
            orm::validate(&adapter).map_err(|e| Error::Validation {
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

        let now = types::Timestamp::now();
        let store_path = resolver.store()?;
        let result = self
            .db
            .with_store(&store_path, |store| Ok(store.get(&key)))?;

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
            metadata: Metadata { created, modified },
        };
        self.db.with_store_mut(&store_path, |store| {
            store.data.insert(key.clone(), value.clone());

            Ok(())
        })?;

        // data row to return
        let result = DataRow::new(key, value);

        Ok(result)
    }
}

///
/// SaveBuilderResult
///

pub struct SaveBuilderResult {
    pub results: Vec<DataRow>,
}

impl SaveBuilderResult {
    #[must_use]
    pub const fn new(results: Vec<DataRow>) -> Self {
        Self { results }
    }

    // ok
    pub const fn ok(&self) -> Result<(), QueryError> {
        Ok(())
    }

    // query_row
    pub fn query_row(&self) -> Result<QueryRow, QueryError> {
        let res = self
            .results
            .first()
            .cloned()
            .map(Into::into)
            .ok_or(Error::NoResultsFound)?;

        Ok(res)
    }

    // query_rows
    pub fn query_rows(self) -> impl Iterator<Item = QueryRow> {
        self.results.into_iter().map(Into::into)
    }

    // entity_row
    pub fn entity_row<E: Entity>(self) -> Result<EntityRow<E>, QueryError> {
        let row = self.results.first().ok_or(Error::NoResultsFound)?.clone();

        let entity_row: EntityRow<E> = row.try_into()?;

        Ok(entity_row)
    }

    // entity_rows
    pub fn entity_rows<E: Entity>(self) -> impl Iterator<Item = Result<EntityRow<E>, QueryError>> {
        self.results
            .into_iter()
            .map(|row| row.try_into().map_err(QueryError::from))
    }

    // entity
    pub fn entity<E: Entity>(self) -> Result<E, QueryError> {
        let row_ref = self.results.first().ok_or(Error::NoResultsFound)?;

        let entity_row: EntityRow<E> = row_ref.clone().try_into()?;

        Ok(entity_row.value.entity)
    }

    // entities
    pub fn entities<E: Entity>(self) -> impl Iterator<Item = Result<E, QueryError>> {
        self.results.into_iter().map(|row| {
            row.try_into()
                .map(|row: EntityRow<E>| row.value.entity)
                .map_err(QueryError::from)
        })
    }
}
