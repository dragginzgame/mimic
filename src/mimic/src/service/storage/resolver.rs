use crate::{
    ThisError,
    db::types::{IndexKey, SortKey},
    query::Selector,
    schema::{
        node::{Entity, EntityIndex, Schema},
        state::{StateError as SchemaStateError, get_schema},
    },
};
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    pub static RESOLVER: RefCell<Resolver> = RefCell::new(
        Resolver::new().expect("failed to init schema resolver")
    );
}

// with_resolver
// public helper
pub fn with_resolver<R>(f: impl FnOnce(&Resolver) -> R) -> R {
    RESOLVER.with_borrow(|r| f(r))
}

///
/// ResolverError
///

#[derive(Debug, ThisError)]
pub enum ResolverError {
    #[error("entity not found: {0}")]
    EntityNotFound(String),

    #[error(transparent)]
    SchemaStateError(#[from] SchemaStateError),
}

///
/// ResolvedSelector
///

#[derive(Debug)]
pub enum ResolvedSelector {
    One(SortKey),
    Many(Vec<SortKey>),
    Range(SortKey, SortKey),
}

///
/// Resolver
///

pub struct Resolver {
    schema: Schema,
}

impl Resolver {
    // new
    pub fn new() -> Result<Self, ResolverError> {
        let schema = get_schema()?;

        Ok(Self { schema })
    }

    // entity
    pub fn entity(&self, path: &str) -> Result<ResolvedEntity, ResolverError> {
        let entity = self
            .schema
            .get_node_as::<Entity>(path)
            .ok_or_else(|| ResolverError::EntityNotFound(path.to_string()))?;

        let sk_fields = entity
            .sort_keys
            .iter()
            .enumerate()
            .map(|(i, sk)| {
                let field = sk.field.clone();
                let label = {
                    let sk_entity = self
                        .schema
                        .get_node_as::<Entity>(&sk.entity)
                        .ok_or_else(|| ResolverError::EntityNotFound(sk.entity.clone()))?;

                    if i == 0 {
                        sk_entity.def.path()
                    } else {
                        sk_entity.def.ident.to_string()
                    }
                };

                Ok(SortKeyField { label, field })
            })
            .collect::<Result<Vec<_>, ResolverError>>()?;

        Ok(ResolvedEntity::new(entity.clone(), sk_fields))
    }
}

///
/// SortKeyField
///

#[derive(Debug)]
pub struct SortKeyField {
    label: String,         // visible label used in SortKey
    field: Option<String>, // actual field name to fetch value from
}

///
/// ResolvedEntity
///

#[derive(Debug)]
pub struct ResolvedEntity {
    entity: Entity,
    sk_fields: Vec<SortKeyField>,
}

impl ResolvedEntity {
    // new
    #[must_use]
    pub const fn new(entity: Entity, sk_fields: Vec<SortKeyField>) -> Self {
        Self { entity, sk_fields }
    }

    // id
    // returns the value of the id field (optional)
    #[must_use]
    pub fn id(&self, field_values: &HashMap<String, String>) -> Option<String> {
        self.sk_fields
            .last()
            .and_then(|sk| sk.field.as_ref())
            .and_then(|field| field_values.get(field))
            .cloned()
    }

    // composite_key
    // returns the composite key ie. ["1", "25", "0xb4af..."]
    #[must_use]
    pub fn composite_key(&self, field_values: &HashMap<String, String>) -> Vec<String> {
        self.sk_fields
            .iter()
            .filter_map(|sk| sk.field.as_ref().and_then(|f| field_values.get(f)).cloned())
            .collect()
    }

    // sort_key
    // returns the full sort key with labels
    #[must_use]
    pub fn sort_key(&self, field_values: &HashMap<String, String>) -> SortKey {
        let key_parts = self
            .sk_fields
            .iter()
            .map(|sk| {
                let value = sk.field.as_ref().and_then(|f| field_values.get(f).cloned());
                (sk.label.clone(), value)
            })
            .collect();

        SortKey::new(key_parts)
    }

    // sort_key_from_composite
    pub fn sort_key_from_composite(&self, values: &[String]) -> Result<SortKey, ResolverError> {
        let key_parts = self
            .sk_fields
            .iter()
            .enumerate()
            .map(|(i, sk)| {
                let value = match sk.field {
                    Some(_) => values.get(i).cloned(),
                    None => None,
                };
                (sk.label.clone(), value)
            })
            .collect();

        Ok(SortKey::new(key_parts))
    }

    // selector
    pub fn selector(&self, selector: &Selector) -> Result<ResolvedSelector, ResolverError> {
        match selector {
            Selector::Only => Ok(ResolvedSelector::One(self.sort_key_from_composite(&[])?)),
            Selector::One(ck) => Ok(ResolvedSelector::One(self.sort_key_from_composite(ck)?)),
            Selector::Many(cks) => {
                let keys = cks
                    .iter()
                    .map(|ck| self.sort_key_from_composite(ck))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(ResolvedSelector::Many(keys))
            }
            Selector::Prefix(prefix) => {
                let start = self.sort_key_from_composite(prefix)?;
                let end = start.create_upper_bound();

                Ok(ResolvedSelector::Range(start, end))
            }
            Selector::Range(start_ck, end_ck) => {
                let start = self.sort_key_from_composite(start_ck)?;
                let end = self.sort_key_from_composite(end_ck)?;

                Ok(ResolvedSelector::Range(start, end))
            }
            Selector::All => {
                let start = self.sort_key_from_composite(&[])?;
                let end = start.create_upper_bound();

                Ok(ResolvedSelector::Range(start, end))
            }
        }
    }

    // indexes
    #[must_use]
    pub fn indexes(&self) -> &[EntityIndex] {
        &self.entity.indexes
    }

    // index_keys_from_values
    #[must_use]
    pub fn index_keys_from_values(&self, field_values: &HashMap<String, String>) -> Vec<IndexKey> {
        self.entity
            .indexes
            .iter()
            .map(|index| {
                let values = index
                    .fields
                    .iter()
                    .map(|f| field_values.get(f).cloned().unwrap_or_default())
                    .collect();

                IndexKey {
                    entity: self.entity.def.path(),
                    fields: index.fields.clone(),
                    values,
                }
            })
            .collect()
    }

    // store_path
    // returns the store path
    #[must_use]
    pub fn store_path(&self) -> &str {
        &self.entity.store
    }
}
