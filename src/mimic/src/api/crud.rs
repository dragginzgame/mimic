use crate::{
    core::schema::entity_crud::EntityCrudManager,
    db::{
        query::types::{
            CreateResponse, DeleteRequest, DeleteResponse, LoadFormat, LoadRequest, LoadResponse,
            SaveRequest, SaveRequestAction, SaveResponse, UpdateResponse,
        },
        types::DataRow,
        Db,
    },
    orm::{
        schema::node::Crud,
        traits::{Entity, EntityDyn},
    },
};
use serde::{Deserialize, Serialize};
use snafu::prelude::*;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("entity '{path}' not found"))]
    EntityNotFound { path: String },

    #[snafu(transparent)]
    Db { source: crate::db::Error },

    #[snafu(transparent)]
    Query { source: crate::db::query::Error },

    #[snafu(transparent)]
    Orm { source: crate::orm::Error },
}

impl Error {
    #[must_use]
    pub fn entity_not_found(path: &str) -> Self {
        Self::EntityNotFound {
            path: path.to_string(),
        }
    }
}

// get_entity
pub fn get_entity(entity: &str) -> Result<&Crud, Error> {
    match EntityCrudManager::get(entity) {
        Some(crud) => Ok(crud),
        None => Err(Error::EntityNotFound {
            path: entity.to_string(),
        }),
    }
}

///
/// CRUD
/// Crud endpoints are directed through these functions to the store builders
///

// load
#[allow(clippy::cast_possible_truncation)]
pub fn load<E>(db: &Db, request: LoadRequest) -> Result<LoadResponse, Error>
where
    E: Entity + 'static,
{
    let iter = crate::db::query::load::<E>()
        .method(request.method)
        .order_option(request.order)
        .filter_option(request.filter)
        .limit_option(request.limit)
        .offset(request.offset)
        .execute(db)?;

    let res = match request.format {
        LoadFormat::Rows => {
            // convert to query rows
            let rows = iter
                .into_iter()
                .map(DataRow::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(Error::from)?;

            LoadResponse::Rows(rows)
        }
        LoadFormat::Count => LoadResponse::Count(iter.count() as u32),
    };

    Ok(res)
}

// delete
pub fn delete<E>(db: &Db, request: &DeleteRequest) -> Result<DeleteResponse, Error>
where
    E: Entity,
{
    let keys = crate::db::query::delete::<E>()
        .one(&request.key)?
        .execute(db)?
        .keys()?;

    Ok(DeleteResponse { keys })
}

// save
pub fn save<E>(db: &Db, request: &SaveRequest) -> Result<SaveResponse, Error>
where
    E: Entity + 'static,
{
    // convert data into entity
    let entity: E = crate::orm::deserialize(&request.data)?;
    let boxed_entity = Box::new(entity) as Box<dyn EntityDyn>;

    match request.action {
        SaveRequestAction::Create => {
            let row = crate::db::query::create()
                .from_entity_dynamic(boxed_entity)
                .execute(db)?
                .query_row()?;

            Ok(SaveResponse::Create(CreateResponse { row }))
        }

        SaveRequestAction::Update => {
            let row = crate::db::query::update()
                .from_entity_dynamic(boxed_entity)
                .execute(db)?
                .query_row()?;

            Ok(SaveResponse::Update(UpdateResponse { row }))
        }
    }
}
