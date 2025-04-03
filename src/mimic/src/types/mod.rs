mod error;

pub use error::ErrorTree;

use crate::orm::traits::EntityDyn;
use derive_more::IntoIterator;

///
/// FixtureList
///

#[derive(Debug, Default, IntoIterator)]
pub struct FixtureList(pub Vec<Box<dyn EntityDyn + 'static>>);

impl FixtureList {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<T: EntityDyn + 'static>(&mut self, entity: T) {
        self.0.push(Box::new(entity));
    }

    pub fn extend(&mut self, list: Self) {
        for entity in list {
            self.0.push(entity);
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<Box<dyn EntityDyn>>> for FixtureList {
    fn into(self) -> Vec<Box<dyn EntityDyn>> {
        self.0
    }
}
