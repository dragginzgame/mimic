mod ext;

pub use ext::*;

use crate::{
    core::traits::EntityKind,
    db::query::{QueryError, QueryValidate},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// SortExpr
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct SortExpr(Vec<(String, SortDirection)>);

impl SortExpr {
    /// Add a single field + direction
    pub fn push(&mut self, field: &str, dir: SortDirection) {
        self.0.push((field.to_string(), dir));
    }

    /// Extend with multiple fields
    pub fn extend<T, I>(&mut self, iter: I)
    where
        T: Into<String>,
        I: IntoIterator<Item = (T, SortDirection)>,
    {
        self.0.extend(iter.into_iter().map(|(f, d)| (f.into(), d)));
    }

    /// Check if no sort fields are defined
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterate over the fields
    pub fn iter(&self) -> impl Iterator<Item = &(String, SortDirection)> {
        self.0.iter()
    }
}

impl<E: EntityKind> QueryValidate<E> for SortExpr {
    fn validate(&self) -> Result<(), QueryError> {
        for (field, _) in self.iter() {
            if !E::FIELDS.contains(&field.as_str()) {
                return Err(QueryError::InvalidSortField(field.clone()));
            }
        }
        Ok(())
    }
}

impl From<Vec<(String, SortDirection)>> for SortExpr {
    fn from(v: Vec<(String, SortDirection)>) -> Self {
        Self(v)
    }
}

///
/// SortDirection
///

#[derive(CandidType, Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}
