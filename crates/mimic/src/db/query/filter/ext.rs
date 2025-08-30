use crate::{
    core::traits::{EntityKind, FieldValue},
    db::query::filter::{FilterDsl, FilterExpr, IntoFilterOpt},
};

///
/// FilterSlot
///

pub trait FilterSlot {
    fn filter_slot(&mut self) -> &mut Option<FilterExpr>;
}

///
/// FilterExt
///

impl<T: FilterSlot> FilterExt for T {}

pub trait FilterExt: FilterSlot + Sized {
    #[must_use]
    fn filter<F, R>(mut self, f: F) -> Self
    where
        F: FnOnce(FilterDsl) -> R,
        R: IntoFilterOpt,
    {
        if let Some(expr) = f(FilterDsl).into_filter_opt() {
            let slot = self.filter_slot();
            let newf = match slot.take() {
                Some(existing) => existing.and(expr),
                None => expr,
            };
            *slot = Some(newf);
        }

        self
    }

    #[must_use]
    fn filter_opt(mut self, expr: Option<FilterExpr>) -> Self {
        if let Some(e) = expr {
            self = self.filter(|_| e);
        }

        self
    }

    #[must_use]
    fn or_filter<F, R>(mut self, f: F) -> Self
    where
        F: FnOnce(FilterDsl) -> R,
        R: IntoFilterOpt,
    {
        if let Some(expr) = f(FilterDsl).into_filter_opt() {
            let slot = self.filter_slot();
            let newf = match slot.take() {
                Some(existing) => existing.or(expr),
                None => expr,
            };
            *slot = Some(newf);
        }

        self
    }

    #[must_use]
    fn or_filter_opt(mut self, expr: Option<FilterExpr>) -> Self {
        if let Some(e) = expr {
            self = self.or_filter(|_| e);
        }

        self
    }

    #[must_use]
    fn filter_expr(self, expr: FilterExpr) -> Self {
        self.filter(|_| expr)
    }

    #[must_use]
    fn simplify(self) -> Self {
        let mut me = self;
        let slot = me.filter_slot();
        if let Some(f) = slot.take() {
            *slot = Some(f.simplify());
        }

        me
    }

    // Shapes (primary key helpers)
    #[must_use]
    fn one<E: EntityKind>(self, value: impl FieldValue) -> Self {
        self.filter(|f| f.eq(E::PRIMARY_KEY, value))
    }

    #[must_use]
    fn many<E, I>(self, values: I) -> Self
    where
        E: EntityKind,
        I: IntoIterator,
        I::Item: FieldValue,
    {
        self.filter(move |f| f.in_iter(E::PRIMARY_KEY, values))
    }
}
