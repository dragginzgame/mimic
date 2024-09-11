use crate::Error;
use db::Db;
use db_query::types::{
    CreateResponse, DeleteRequest, DeleteResponse, LoadFormat, LoadRequest, LoadResponse, QueryRow,
    SaveRequest, SaveRequestAction, SaveResponse, UpdateResponse,
};
use orm::traits::Entity;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// CrudError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum CrudError {
    #[snafu(display("entity '{path}' not found"))]
    EntityNotFound { path: String },

    #[snafu(transparent)]
    Query { source: db_query::Error },

    #[snafu(transparent)]
    Orm { source: orm::Error },
}

impl CrudError {
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
    let iter = db_query::load::<E>(db)
        .method(request.method)
        .order_option(request.order)
        .filter_option(request.filter)
        .limit_option(request.limit)
        .offset(request.offset)
        .execute()
        .map_err(CrudError::from)?;

    let res = match request.format {
        LoadFormat::Rows => {
            // convert to query rows
            let rows = iter
                .into_iter()
                .map(QueryRow::try_from)
                .collect::<Result<Vec<_>, _>>();
            let rows = rows.map_err(CrudError::from)?;

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
    let keys = db_query::delete::<E>(db)
        .one(&request.key)
        .map_err(CrudError::from)?
        .keys()
        .map_err(CrudError::from)?;

    Ok(DeleteResponse { keys })
}

// save
pub fn save<E>(db: &Db, request: &SaveRequest) -> Result<SaveResponse, Error>
where
    E: Entity + 'static,
{
    // convert data into entity
    let entity: E = orm::deserialize(&request.data).map_err(CrudError::from)?;
    let boxed_entity = Box::new(entity) as Box<dyn orm::traits::EntityDynamic>;

    match request.action {
        SaveRequestAction::Create => {
            let row = db_query::create(db)
                .from_entity_dynamic(boxed_entity)
                .map_err(CrudError::from)?
                .query_row()
                .map_err(CrudError::from)?;

            Ok(SaveResponse::Create(CreateResponse { row }))
        }

        SaveRequestAction::Update => {
            let row = db_query::update(db)
                .from_entity_dynamic(boxed_entity)
                .map_err(CrudError::from)?
                .query_row()
                .map_err(CrudError::from)?;

            Ok(SaveResponse::Update(UpdateResponse { row }))
        }
    }
}
