use crate::prelude::*;

///
/// UpdateViewTrait
///

pub struct UpdateViewTrait {}

///
/// Entity
///

impl Imp<Entity> for UpdateViewTrait {
    fn strategy(_: &Entity) -> Option<TraitStrategy> {
        None
    }
}
