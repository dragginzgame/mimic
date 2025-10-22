use crate::prelude::*;

///
/// EntityUpdateTrait
///

pub struct EntityUpdateTrait {}

///
/// Entity
///

impl Imp<Entity> for EntityUpdateTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        None
    }
}
