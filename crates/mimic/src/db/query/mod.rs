mod delete;
mod filter;
mod limit;
mod load;
mod planner;
mod save;
mod sort;

pub use delete::*;
pub use filter::*;
pub use limit::*;
pub use load::*;
pub use planner::*;
pub use save::*;
pub use sort::*;

///
/// Query Prelude
///

pub mod prelude {
    pub use crate::db::query::{
        self,
        filter::{FilterDsl, FilterExt as _},
        limit::LimitExt as _,
        sort::SortExt as _,
    };
}

use crate::core::traits::EntityKind;
use thiserror::Error as ThisError;

///
/// QueryError
///

#[derive(Debug, ThisError)]
pub enum QueryError {
    #[error("invalid filter field '{0}'")]
    InvalidFilterField(String),

    #[error("invalid index field '{0}'")]
    InvalidIndexField(String),

    #[error("invalid sort field '{0}'")]
    InvalidSortField(String),

    #[error("invalid filter value: {0}")]
    InvalidFilterValue(String),

    #[error("invalid comparator usage: {0}")]
    InvalidComparator(String),
}

///
/// QueryValidate Trait
///

pub trait QueryValidate<E: EntityKind> {
    fn validate(&self) -> Result<(), QueryError>;
}

impl<E: EntityKind, T: QueryValidate<E>> QueryValidate<E> for Box<T> {
    fn validate(&self) -> Result<(), QueryError> {
        (**self).validate()
    }
}

// load
#[must_use]
pub fn load() -> LoadQuery {
    LoadQuery::new()
}

// delete
#[must_use]
pub fn delete() -> DeleteQuery {
    DeleteQuery::new()
}

// create
#[must_use]
pub fn create() -> SaveQuery {
    SaveQuery::new(SaveMode::Create)
}

// update
#[must_use]
pub fn update() -> SaveQuery {
    SaveQuery::new(SaveMode::Update)
}

// replace
#[must_use]
pub fn replace() -> SaveQuery {
    SaveQuery::new(SaveMode::Replace)
}
