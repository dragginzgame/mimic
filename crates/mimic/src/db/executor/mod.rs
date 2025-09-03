mod context;
mod delete;
mod filter;
mod load;
mod save;

pub use context::*;
pub use delete::*;
pub use filter::*;
pub use load::*;
pub use save::*;

use crate::{
    core::traits::EntityKind,
    db::{
        query::{FilterExpr, QueryPlan, QueryPlanner},
        store::DataKey,
    },
    obs::metrics::Span,
};
use thiserror::Error as ThisError;

///
/// ExecutorError
///

#[derive(Debug, ThisError)]
pub enum ExecutorError {
    #[error("data key exists: {0}")]
    KeyExists(DataKey),

    #[error("data key not found: {0}")]
    KeyNotFound(DataKey),

    #[error("index constraint violation: {0} ({1})")]
    IndexViolation(String, String),
}

impl ExecutorError {
    #[must_use]
    pub fn index_violation(path: &str, index_fields: &[&str]) -> Self {
        Self::IndexViolation(path.to_string(), index_fields.join(", "))
    }
}

/// Plan a query for an entity given an optional filter.
#[must_use]
pub fn plan_for<E: EntityKind>(filter: Option<&FilterExpr>) -> QueryPlan {
    QueryPlanner::new(filter).plan::<E>()
}

/// Convenience: set span rows from a usize length.
pub const fn set_rows_from_len<E: EntityKind>(span: &mut Span<E>, len: usize) {
    span.set_rows(len as u64);
}
