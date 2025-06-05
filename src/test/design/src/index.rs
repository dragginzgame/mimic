use crate::prelude::*;

///
/// Index
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "Index", field = "id"),
    index(store = "crate::schema::TestIndex", fields = "x"),
    index(store = "crate::schema::TestIndex", fields = "y", unique),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "x", value(item(prim = "Int32"))),
    field(name = "y", value(item(prim = "Int32")))
)]
pub struct Index {}

impl Index {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }
}
