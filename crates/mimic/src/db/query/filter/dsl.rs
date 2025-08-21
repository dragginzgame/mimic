use crate::core::value::Value;
use crate::db::query::{Cmp, FilterClause, FilterExpr};

#[derive(Clone, Copy, Debug, Default)]
pub struct FilterDsl;

macro_rules! cmp_fns {
    ($( $name:ident => $cmp:ident ),*) => {
        $(
            pub fn $name(self, field: impl AsRef<str>, v: impl Into<Value>) -> FilterExpr {
                FilterExpr::Clause(FilterClause::new(field.as_ref(), Cmp::$cmp, v))
            }
        )*
    }
}

impl FilterDsl {}

impl FilterDsl {
    //
    // Comparators
    //

    cmp_fns! {
        eq => Eq,
        eq_ci => EqCi,
        ne => Ne,
        ne_ci => NeCi,
        lt => Lt,
        lte => Lte,
        gt => Gt,
        gte => Gte,
        contains => Contains,
        contains_ci => ContainsCi,
        starts_with => StartsWith,
        starts_with_ci => StartsWithCi,
        ends_with => EndsWith,
        ends_with_ci => EndsWithCi
    }

    //
    // Always / Never
    // (good placeholders for constructing queries)
    //

    #[must_use]
    pub const fn always(self) -> FilterExpr {
        FilterExpr::True
    }
    #[must_use]
    pub const fn never(self) -> FilterExpr {
        FilterExpr::False
    }

    //
    // Presence / Null
    //

    pub fn is_some(self, field: impl AsRef<str>) -> FilterExpr {
        FilterExpr::Clause(FilterClause::new(field.as_ref(), Cmp::IsSome, ()))
    }
    pub fn is_none(self, field: impl AsRef<str>) -> FilterExpr {
        FilterExpr::Clause(FilterClause::new(field.as_ref(), Cmp::IsNone, ()))
    }

    //
    // Collections
    //

    fn cmp_iter<I>(field: impl AsRef<str>, cmp: Cmp, vals: I) -> FilterExpr
    where
        I: IntoIterator,
        I::Item: Into<Value>,
    {
        let list = vals.into_iter().map(Into::into).collect::<Vec<_>>();

        FilterExpr::Clause(FilterClause::new(field.as_ref(), cmp, Value::List(list)))
    }

    /// field IN (v1, v2, ...)
    #[inline]
    pub fn in_iter<I>(self, field: impl AsRef<str>, vals: I) -> FilterExpr
    where
        I: IntoIterator,
        I::Item: Into<Value>,
    {
        Self::cmp_iter(field, Cmp::In, vals)
    }

    /// ANY element of `vals` is contained in the collection field.
    #[inline]
    pub fn any_in<I>(self, field: impl AsRef<str>, vals: I) -> FilterExpr
    where
        I: IntoIterator,
        I::Item: Into<Value>,
    {
        Self::cmp_iter(field, Cmp::AnyIn, vals)
    }

    /// ALL elements of `vals` are contained in the collection field.
    #[inline]
    pub fn all_in<I>(self, field: impl AsRef<str>, vals: I) -> FilterExpr
    where
        I: IntoIterator,
        I::Item: Into<Value>,
    {
        Self::cmp_iter(field, Cmp::AllIn, vals)
    }

    //
    // Collectors
    //

    pub fn all<I, F>(items: I) -> Option<FilterExpr>
    where
        I: IntoIterator<Item = F>,
        F: IntoFilterOpt,
    {
        let mut it = items.into_iter().filter_map(IntoFilterOpt::into_filter_opt);
        let first = it.next()?;
        Some(it.fold(first, FilterExpr::and))
    }

    pub fn any<I, F>(items: I) -> Option<FilterExpr>
    where
        I: IntoIterator<Item = F>,
        F: IntoFilterOpt,
    {
        let mut it = items.into_iter().filter_map(IntoFilterOpt::into_filter_opt);
        let first = it.next()?;
        Some(it.fold(first, FilterExpr::or))
    }

    //
    // Conditionals
    //

    pub fn when(self, cond: bool, f: impl FnOnce() -> FilterExpr) -> Option<FilterExpr> {
        cond.then(f)
    }

    pub fn when_some<T>(
        self,
        opt: Option<T>,
        f: impl FnOnce(T) -> FilterExpr,
    ) -> Option<FilterExpr> {
        opt.map(f)
    }
}

///
/// IntoFilterOpt
///

pub trait IntoFilterOpt {
    fn into_filter_opt(self) -> Option<FilterExpr>;
}

impl IntoFilterOpt for FilterExpr {
    #[inline]
    fn into_filter_opt(self) -> Option<FilterExpr> {
        Some(self)
    }
}

impl IntoFilterOpt for Option<FilterExpr> {
    #[inline]
    fn into_filter_opt(self) -> Option<FilterExpr> {
        self
    }
}
