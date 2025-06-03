use crate::traits::EntityKindDyn;
use derive_more::{Deref, DerefMut, IntoIterator};

///
/// FixtureList
///

pub type FixtureList = Vec<Box<dyn EntityKindDyn + 'static>>;

///
/// FixtureBuilder
///

#[derive(Debug, Default, Deref, DerefMut, IntoIterator)]
pub struct FixtureBuilder(FixtureList);

impl FixtureBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<T: EntityKindDyn + 'static>(&mut self, entity: T) {
        self.0.push(Box::new(entity));
    }

    /// Extend this fixture list with another builder
    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

#[allow(clippy::from_over_into)]
impl Into<FixtureList> for FixtureBuilder {
    fn into(self) -> Vec<Box<dyn EntityKindDyn>> {
        self.0
    }
}
