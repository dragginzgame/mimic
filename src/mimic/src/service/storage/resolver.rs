use crate::{
    ThisError,
    db::types::SortKey,
    query::Selector,
    schema::{
        node::{Entity, Schema},
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
    pub fn new() -> Result<Self, ResolverError> {
        let schema = get_schema()?;

        Ok(Self { schema })
    }

    // resolve_store
    pub fn resolve_store(&self, path: &str) -> Result<String, ResolverError> {
        let entity = self
            .schema
            .get_node_as::<Entity>(path)
            .ok_or_else(|| ResolverError::EntityNotFound(path.to_string()))?;

        Ok(entity.store.clone())
    }

    // resolve_method
    pub fn resolve_selector(
        &self,
        path: &str,
        selector: &Selector,
    ) -> Result<ResolvedSelector, ResolverError> {
        match selector {
            Selector::Only => {
                let key = self.resolve_sort_key(path, &[])?;

                Ok(ResolvedSelector::One(key))
            }
            Selector::One(ck) => {
                let key = self.resolve_sort_key(path, ck)?;

                Ok(ResolvedSelector::One(key))
            }
            Selector::Many(cks) => {
                let keys = cks
                    .iter()
                    .map(|ck| self.resolve_sort_key(path, ck))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(ResolvedSelector::Many(keys))
            }
            Selector::Prefix(prefix) => {
                let start = self.resolve_sort_key(path, prefix)?;
                let end = start.create_upper_bound();

                Ok(ResolvedSelector::Range(start, end))
            }
            Selector::Range(start_ck, end_ck) => {
                let start = self.resolve_sort_key(path, start_ck)?;
                let end = self.resolve_sort_key(path, end_ck)?;

                Ok(ResolvedSelector::Range(start, end))
            }
            Selector::All => {
                let start = self.resolve_sort_key(path, &[])?;
                let end = start.create_upper_bound();

                Ok(ResolvedSelector::Range(start, end))
            }
        }
    }

    // resolve_sort_key
    pub fn resolve_sort_key(
        &self,
        path: &str,
        composite_key: &[String],
    ) -> Result<SortKey, ResolverError> {
        let chain_format = self.chain_format(path)?;

        let key_parts = chain_format
            .into_iter()
            .enumerate()
            .map(|(i, part)| (part, composite_key.get(i).cloned()))
            .collect();

        Ok(SortKey::new(key_parts))
    }

    // chain_format
    fn chain_format(&self, path: &str) -> Result<Vec<String>, ResolverError> {
        let entity = self
            .schema
            .get_node_as::<Entity>(path)
            .ok_or_else(|| ResolverError::EntityNotFound(path.to_string()))?;

        let mut format = Vec::new();
        for (i, sk) in entity.sort_keys.iter().enumerate() {
            let sk_entity = self
                .schema
                .get_node_as::<Entity>(&sk.entity)
                .ok_or_else(|| ResolverError::EntityNotFound(sk.entity.clone()))?;

            format.push(if i == 0 {
                sk_entity.def.path()
            } else {
                sk_entity.def.ident.to_string()
            });
        }

        Ok(format)
    }
}
