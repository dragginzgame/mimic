use db::{
    db::Db,
    query::types::{
        CreateResponse, DeleteRequest, DeleteResponse, LoadFormat, LoadRequest, LoadResponse,
        QueryRow, SaveRequest, SaveRequestAction, SaveResponse, UpdateResponse,
    },
};
use orm::traits::Entity;
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
    Query { source: db::query::Error },

    #[snafu(transparent)]
    Orm { source: orm::Error },
}

impl Error {
    #[must_use]
    pub fn entity_not_found(path: &str) -> Self {
        Self::EntityNotFound {
            path: path.to_string(),
        }
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
    let iter = db::query::load::<E>(db)
        .method(request.method)
        .order_option(request.order)
        .filter_option(request.filter)
        .limit_option(request.limit)
        .offset(request.offset)
        .execute()?;

    let res = match request.format {
        LoadFormat::Rows => {
            // convert to query rows
            let rows = iter
                .into_iter()
                .map(QueryRow::try_from)
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
    let keys = db::query::delete::<E>(db).one(&request.key)?.keys()?;

    Ok(DeleteResponse { keys })
}

// save
pub fn save<E>(db: &Db, request: &SaveRequest) -> Result<SaveResponse, Error>
where
    E: Entity + 'static,
{
    // convert data into entity
    let entity: E = orm::deserialize(&request.data)?;
    let boxed_entity = Box::new(entity) as Box<dyn orm::traits::EntityDynamic>;

    match request.action {
        SaveRequestAction::Create => {
            let row = db::query::create(db)
                .from_entity_dynamic(boxed_entity)?
                .query_row()?;

            Ok(SaveResponse::Create(CreateResponse { row }))
        }

        SaveRequestAction::Update => {
            let row = db::query::update(db)
                .from_entity_dynamic(boxed_entity)?
                .query_row()?;

            Ok(SaveResponse::Update(UpdateResponse { row }))
        }
    }
}
