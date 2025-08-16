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
/// SortKey Wrappers
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
/// SortExprBuilder
///

#[derive(Default)]
pub struct SortExprBuilder {
    keys: Vec<(String, SortDirection)>,
}

impl SortExprBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn asc(mut self, field: impl Into<String>) -> Self {
        self.keys.push((field.into(), SortDirection::Asc));
        self
    }

    #[must_use]
    pub fn desc(mut self, field: impl Into<String>) -> Self {
        self.keys.push((field.into(), SortDirection::Desc));
        self
    }
}

impl From<SortExprBuilder> for SortExpr {
    fn from(b: SortExprBuilder) -> Self {
        b.keys.into()
    }
}

///
/// SortExt
///

impl<T: SortSlot> SortExt for T {}

pub trait SortExt: SortSlot + Sized {
    /// Closure-based DSL (matches `.filter(|f| ...)`)
    #[must_use]
    fn sort<F>(mut self, f: F) -> Self
    where
        F: FnOnce(SortExprBuilder) -> SortExprBuilder,
    {
        let slot = self.sort_slot();
        let expr: SortExpr = f(SortExprBuilder::new()).into();

        if let Some(existing) = slot.as_mut() {
            existing.extend(expr.iter().cloned());
        } else {
            *slot = Some(expr);
        }

        self
    }
}
