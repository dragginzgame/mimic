use crate::{core::traits::EntityKind, db::Db};

///
/// EntityService
///

pub struct EntityService {}

impl EntityService {
    pub fn save_fixture<E: EntityKind>(db: Db<E::Canister>, entity: E) {
        db.replace(entity).unwrap();
    }
}
