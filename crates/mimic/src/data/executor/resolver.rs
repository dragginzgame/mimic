use crate::{
    ThisError,
    schema::{
        node::{Entity, EntityIndex, Schema},
        state::{StateError as SchemaStateError, get_schema},
    },
    traits::EntityKind,
};
use std::cell::RefCell;

thread_local! {
    pub static RESOLVER: RefCell<Resolver> = RefCell::new(
        Resolver::new().expect("failed to init schema resolver")
    );
}

// with_resolver
fn with_resolver<R>(f: impl FnOnce(&Resolver) -> R) -> R {
    RESOLVER.with_borrow(|r| f(r))
}

// resolve_entity
// public helper
pub fn resolve_entity<E: EntityKind>() -> Result<ResolvedEntity, ResolverError> {
    with_resolver(|r| r.resolve::<E>())
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

    // resolve
    pub fn resolve<E: EntityKind>(&self) -> Result<ResolvedEntity, ResolverError> {
        let entity = self
            .schema
            .get_node_as::<Entity>(E::PATH)
            .ok_or_else(|| ResolverError::EntityNotFound(E::PATH.to_string()))?;

        let sk_fields = entity
            .sort_keys
            .iter()
            .map(|sk| {
                let field = sk.field.clone();
                let path = {
                    let sk_entity = self
                        .schema
                        .get_node_as::<Entity>(&sk.entity)
                        .ok_or_else(|| ResolverError::EntityNotFound(sk.entity.clone()))?;

                    sk_entity.def.path()
                };

                Ok(SortKeyField { path, field })
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
    path: String,          // SortKeyPart path
    field: Option<String>, // actual field name to fetch value from
}

///
/// ResolvedEntity
///

#[derive(Debug)]
pub struct ResolvedEntity {
    pub entity: Entity,
    pub sk_fields: Vec<SortKeyField>,
}

impl ResolvedEntity {
    /// new
    #[must_use]
    pub const fn new(entity: Entity, sk_fields: Vec<SortKeyField>) -> Self {
        Self { entity, sk_fields }
    }

    // indexes
    #[must_use]
    pub fn indexes(&self) -> &[EntityIndex] {
        &self.entity.indexes
    }
}
