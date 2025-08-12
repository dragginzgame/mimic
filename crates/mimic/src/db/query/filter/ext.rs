use crate::{
    core::{Value, traits::EntityKind},
    db::query::filter::{FilterDsl, FilterExpr},
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
    fn filter(self, f: impl FnOnce(FilterDsl) -> FilterExpr) -> Self {
        let mut me = self;
        let slot = me.filter_slot();
        let expr = f(FilterDsl);
        let newf = match slot.take() {
            Some(existing) => existing.and(expr),
            None => expr,
        };
        *slot = Some(newf);

        me
    }

    #[must_use]
    fn filter_opt(self, f: impl FnOnce(FilterDsl) -> Option<FilterExpr>) -> Self {
        let mut me = self;
        if let Some(expr) = f(FilterDsl) {
            let slot = me.filter_slot();
            let newf = match slot.take() {
                Some(existing) => existing.and(expr),
                None => expr,
            };
            *slot = Some(newf);
        }

        me
    }

    #[must_use]
    fn or_filter(self, f: impl FnOnce(FilterDsl) -> FilterExpr) -> Self {
        let mut me = self;
        let slot = me.filter_slot();
        let expr = f(FilterDsl);
        let newf = match slot.take() {
            Some(existing) => existing.or(expr),
            None => expr,
        };
        *slot = Some(newf);

        me
    }

    #[must_use]
    fn or_filter_opt(self, f: impl FnOnce(FilterDsl) -> Option<FilterExpr>) -> Self {
        let mut me = self;
        if let Some(expr) = f(FilterDsl) {
            let slot = me.filter_slot();
            let newf = match slot.take() {
                Some(existing) => existing.or(expr),
                None => expr,
            };
            *slot = Some(newf);
        }

        me
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
    fn one<E: EntityKind>(self, value: impl Into<Value>) -> Self {
        self.filter(|f| f.eq(E::PRIMARY_KEY, value))
    }

    #[must_use]
    fn many<E, I>(self, values: I) -> Self
    where
        E: EntityKind,
        I: IntoIterator,
        I::Item: Into<Value>,
    {
        self.filter(move |f| f.in_iter(E::PRIMARY_KEY, values))
    }
}
