use crate::{
    db::types::IndexKey,
    def::{EntityValues, traits::EntityKind},
};

// resolve_index_key
#[must_use]
pub fn resolve_index_key<E: EntityKind>(
    fields: &[&'static str],
    values: &EntityValues,
) -> Option<IndexKey> {
    values
        .collect_all(fields)
        .map(|collected| IndexKey::new(E::PATH, fields, collected))
}
