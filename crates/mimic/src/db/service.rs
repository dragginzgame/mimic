use crate::{core::traits::EntityKind, db::Db};

///
/// EntityService
///

pub struct EntityService {}

impl EntityService {
    pub fn save_fixture<E: EntityKind>(db: Db, entity: E) {
        db.save().replace::<E>(entity).unwrap();
    }
}
