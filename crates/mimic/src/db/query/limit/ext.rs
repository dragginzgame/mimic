use crate::db::query::LimitExpr;

///
/// LimitSlot
///

pub trait LimitSlot {
    fn limit_slot(&mut self) -> &mut Option<LimitExpr>;
}

///
/// LimitExt
///

impl<T: LimitSlot> LimitExt for T {}

pub trait LimitExt: LimitSlot + Sized {
    #[must_use]
    fn limit(self, n: u32) -> Self {
        let mut me = self;

        let slot = me.limit_slot();
        let expr = slot.take().unwrap_or_default().limit(n);
        *slot = Some(expr);

        me
    }

    #[must_use]
    fn limit_option(self, n: Option<u32>) -> Self {
        let mut me = self;

        let slot = me.limit_slot();
        let mut expr = slot.take().unwrap_or_default();
        expr.limit = n;
        *slot = Some(expr);

        me
    }

    #[must_use]
    fn offset(self, n: u32) -> Self {
        let mut me = self;

        let slot = me.limit_slot();
        let expr = slot.take().unwrap_or_default().offset(n);
        *slot = Some(expr);

        me
    }
}
