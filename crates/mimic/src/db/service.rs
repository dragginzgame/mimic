use crate::{
    db::{
        executor::SaveExecutor,
        query::{SaveMode, SaveQueryTyped},
    },
    ops::traits::EntityKind,
};

///
/// EntityService
///

pub struct EntityService {}

impl EntityService {
    pub fn save_fixture<E: EntityKind>(exec: &mut SaveExecutor, entity: E) {
        let q = SaveQueryTyped::new(SaveMode::Replace, entity);

        exec.execute(q).unwrap();
    }
}
