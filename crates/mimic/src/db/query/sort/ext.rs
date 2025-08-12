use crate::db::query::{SortDirection, SortExpr};

///
/// SortSlot
///

pub trait SortSlot {
    fn sort_slot(&mut self) -> &mut Option<SortExpr>;
}

pub trait IntoSortKey {
    fn into_sort_key(self) -> (String, SortDirection);
}

///
/// Builders
///

pub struct Asc<T: Into<String>>(pub T);
pub struct Desc<T: Into<String>>(pub T);

impl<T: Into<String>> IntoSortKey for Asc<T> {
    fn into_sort_key(self) -> (String, SortDirection) {
        (self.0.into(), SortDirection::Asc)
    }
}
impl<T: Into<String>> IntoSortKey for Desc<T> {
    fn into_sort_key(self) -> (String, SortDirection) {
        (self.0.into(), SortDirection::Desc)
    }
}

///
/// SortExt
///

impl<T: SortSlot> SortExt for T {}

pub trait SortExt: SortSlot + Sized {
    /// Add many sort keys at once.
    #[must_use]
    fn sort<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: IntoSortKey,
    {
        let iter = items.into_iter().map(IntoSortKey::into_sort_key);
        let slot = self.sort_slot();
        if let Some(expr) = slot.as_mut() {
            expr.extend(iter);
        } else {
            *slot = Some(iter.collect::<SortExpr>());
        }
        self
    }

    /// Add a single ascending key: `.sort_asc("name")`
    #[must_use]
    fn sort_asc(self, field: impl Into<String>) -> Self {
        self.sort(std::iter::once(Asc(field)))
    }

    /// Add a single descending key: `.sort_desc("level")`
    #[must_use]
    fn sort_desc(self, field: impl Into<String>) -> Self {
        self.sort(std::iter::once(Desc(field)))
    }
}
