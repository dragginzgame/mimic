use crate::traits::EntityDynamic;

///
/// FixtureList
///

pub struct FixtureList(Vec<Box<dyn EntityDynamic + 'static>>);

impl FixtureList {
    pub fn push(&mut self, entity: impl EntityDynamic + 'static) {
        self.0.push(Box::new(entity));
    }
}
