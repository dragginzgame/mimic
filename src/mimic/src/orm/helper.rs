use super::traits::EntityDyn;

///
/// FixtureList
///

#[derive(Debug, Default)]
pub struct FixtureList(Vec<Box<dyn EntityDyn + 'static>>);

impl FixtureList {
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, entity: impl EntityDyn + 'static) {
        self.0.push(Box::new(entity));
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<Box<dyn EntityDyn>>> for FixtureList {
    fn into(self) -> Vec<Box<dyn EntityDyn>> {
        self.0
    }
}
