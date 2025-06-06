use crate::{
    ThisError,
    data::{
        query::Selector,
        store::{IndexKey, SortKey},
    },
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
/// A runtime-resolved entity with structural metadata needed for key generation
///

#[derive(Debug)]
pub struct ResolvedEntity {
    entity: Entity,
    sk_fields: Vec<SortKeyField>,
}

impl ResolvedEntity {
    /// new
    /// create a new ResolvedEntity from a schema Entity and its resolved sort key fields
    #[must_use]
    pub const fn new(entity: Entity, sk_fields: Vec<SortKeyField>) -> Self {
        Self { entity, sk_fields }
    }

    // id
    // returns the value of the id field (optional)
    #[must_use]
    pub fn id(&self, field_values: &HashMap<String, Option<String>>) -> Option<String> {
        self.sort_key(field_values)
            .0
            .last()
            .and_then(|(_, val)| val.clone())
    }

    // composite_key
    // returns the composite key ie. ["1", "25", "0xb4af..."]
    #[must_use]
    pub fn composite_key(&self, field_values: &HashMap<String, Option<String>>) -> Vec<String> {
        self.sort_key(field_values)
            .0
            .iter()
            .filter_map(|(_, v)| v.clone())
            .collect()
    }

    // sort_key
    // returns a sort key based on field values
    #[must_use]
    pub fn sort_key(&self, field_values: &HashMap<String, Option<String>>) -> SortKey {
        let key_parts = self
            .sk_fields
            .iter()
            .map(|sk| {
                let value = sk
                    .field
                    .as_ref()
                    .and_then(|f| field_values.get(f))
                    .cloned()
                    .flatten(); // unwraps Option<Option<String>> into Option<String>

                (sk.label.clone(), value)
            })
            .collect();

        SortKey::new(key_parts)
    }

    // build_sort_key
    // builds a sort key based on a specific composite key
    #[must_use]
    pub fn build_sort_key(&self, values: &[String]) -> SortKey {
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

        SortKey::new(key_parts)
    }

    // build_index_key
    //
    // field_values are UNORDERED, it's the index.fields that is ORDERED
    // returning None means 'do not index'
    #[must_use]
    pub fn build_index_key(
        &self,
        index: &EntityIndex,
        field_values: &HashMap<String, Option<String>>,
    ) -> Option<IndexKey> {
        let mut values = Vec::with_capacity(index.fields.len());

        for field in &index.fields {
            match field_values.get(field) {
                Some(Some(value)) if !value.is_empty() => values.push(value.clone()),
                _ => return None,
            }
        }

        Some(IndexKey {
            entity: self.entity.def.path(),
            fields: index.fields.clone(),
            values,
        })
    }

    // selector
    #[must_use]
    pub fn selector(&self, selector: &Selector) -> ResolvedSelector {
        match selector {
            Selector::All => {
                let start = self.build_sort_key(&[]);
                let end = start.create_upper_bound();

                ResolvedSelector::Range(start, end)
            }
            Selector::Only => ResolvedSelector::One(self.build_sort_key(&[])),
            Selector::One(ck) => ResolvedSelector::One(self.build_sort_key(ck)),
            Selector::Many(cks) => {
                let keys = cks.iter().map(|ck| self.build_sort_key(ck)).collect();

                ResolvedSelector::Many(keys)
            }
            Selector::Prefix(prefix) => {
                let start = self.build_sort_key(prefix);
                let end = start.create_upper_bound();

                ResolvedSelector::Range(start, end)
            }
            Selector::Range(start_ck, end_ck) => {
                let start = self.build_sort_key(start_ck);
                let end = self.build_sort_key(end_ck);

                ResolvedSelector::Range(start, end)
            }
        }
    }

    // indexes
    #[must_use]
    pub fn indexes(&self) -> &[EntityIndex] {
        &self.entity.indexes
    }

    // store_path
    // returns the store path
    #[must_use]
    pub fn store_path(&self) -> &str {
        &self.entity.store
    }
}
