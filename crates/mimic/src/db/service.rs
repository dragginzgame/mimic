use crate::{core::traits::EntityKind, db::executor::SaveExecutor};

///
/// EntityService
///

pub struct EntityService {}

impl EntityService {
    pub fn save_fixture<E: EntityKind>(exec: &mut SaveExecutor, entity: E) {
        exec.replace(entity).unwrap();
    }
}
