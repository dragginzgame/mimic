use crate::{
    ThisError,
    db::types::SortKey,
    query::Selector,
    schema::{
        node::{Entity, EntityIndex, Schema},
        state::{StateError as SchemaStateError, get_schema},
    },
};
use std::cell::RefCell;

thread_local! {
    pub static RESOLVER: RefCell<Resolver> = RefCell::new(
        Resolver::new().expect("failed to init schema resolver")
    );
}

// Public helper
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

        // compute the sort key labels
        let sort_key_labels = entity
            .sort_keys
            .iter()
            .enumerate()
            .map(|(i, sk)| {
                let sk_entity = self
                    .schema
                    .get_node_as::<Entity>(&sk.entity)
                    .ok_or_else(|| ResolverError::EntityNotFound(sk.entity.clone()))?;

                Ok(if i == 0 {
                    sk_entity.def.path()
                } else {
                    sk_entity.def.ident.to_string()
                })
            })
            .collect::<Result<Vec<_>, ResolverError>>()?;

        Ok(ResolvedEntity::new(entity.clone(), sort_key_labels))
    }
}

///
/// ResolvedEntity
///

pub struct ResolvedEntity {
    entity: Entity,
    sort_key_labels: Vec<String>,
}

impl ResolvedEntity {
    // new
    #[must_use]
    pub fn new(entity: Entity, sort_key_labels: Vec<String>) -> Self {
        Self {
            entity,
            sort_key_labels,
        }
    }

    // id
    // returns the optional id field
    #[must_use]
    pub fn id(&self) -> Option<&str> {
        self.entity.sort_keys.last()?.field.as_deref()
    }

    // selector
    pub fn selector(&self, selector: &Selector) -> Result<ResolvedSelector, ResolverError> {
        match selector {
            Selector::Only => Ok(ResolvedSelector::One(self.sort_key(&[])?)),
            Selector::One(ck) => Ok(ResolvedSelector::One(self.sort_key(ck)?)),
            Selector::Many(cks) => {
                let keys = cks
                    .iter()
                    .map(|ck| self.sort_key(ck))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(ResolvedSelector::Many(keys))
            }
            Selector::Prefix(prefix) => {
                let start = self.sort_key(prefix)?;
                let end = start.create_upper_bound();

                Ok(ResolvedSelector::Range(start, end))
            }
            Selector::Range(start_ck, end_ck) => {
                let start = self.sort_key(start_ck)?;
                let end = self.sort_key(end_ck)?;

                Ok(ResolvedSelector::Range(start, end))
            }
            Selector::All => {
                let start = self.sort_key(&[])?;
                let end = start.create_upper_bound();

                Ok(ResolvedSelector::Range(start, end))
            }
        }
    }

    // composite_key
    #[must_use]
    pub fn composite_key(&self) -> Vec<&str> {
        self.entity
            .sort_keys
            .iter()
            .filter_map(|sk| sk.field.as_deref())
            .collect()
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

    // sort_key
    // pass in the values of a Composite Key
    pub fn sort_key(&self, values: &[String]) -> Result<SortKey, ResolverError> {
        let key_parts = self
            .sort_key_labels
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, part)| (part, values.get(i).cloned()))
            .collect();

        Ok(SortKey::new(key_parts))
    }
}
