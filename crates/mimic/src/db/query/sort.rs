use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// SortExpr
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct SortExpr(pub Vec<(String, SortDirection)>);

impl SortExpr {
    /// Add a single field + direction
    pub fn push<T: Into<String>>(&mut self, field: T, dir: SortDirection) {
        self.0.push((field.into(), dir));
    }

    /// Extend with multiple fields
    pub fn extend<T, I>(&mut self, iter: I)
    where
        T: Into<String>,
        I: IntoIterator<Item = (T, SortDirection)>,
    {
        self.0.extend(iter.into_iter().map(|(f, d)| (f.into(), d)));
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &(String, SortDirection)> {
        self.0.iter()
    }
}

impl<T> FromIterator<(T, SortDirection)> for SortExpr
where
    T: Into<String>,
{
    fn from_iter<I: IntoIterator<Item = (T, SortDirection)>>(iter: I) -> Self {
        SortExpr(iter.into_iter().map(|(f, d)| (f.into(), d)).collect())
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
