use crate::traits::EntityDynamic;

///
/// FixtureList
///

#[derive(Debug, Default)]
pub struct FixtureList(Vec<Box<dyn EntityDynamic + 'static>>);

impl FixtureList {
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, entity: impl EntityDynamic + 'static) {
        self.0.push(Box::new(entity));
    }
}
