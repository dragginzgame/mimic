use crate::{
    build::schema_read,
    node::{Entity, Enum, Map, Newtype, Primitive, Record, Tuple, ValidateNode, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};
use std::{any::TypeId, collections::HashSet};
use types::ErrorVec;

///
/// Item
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Item {
    Id,
    Is(ItemIs),
    Relation(ItemRelation),
}

impl Item {
    // is_id
    #[must_use]
    pub const fn is_id(&self) -> bool {
        matches!(self, Self::Id)
    }

    // is_relation
    #[must_use]
    pub const fn is_relation(&self) -> bool {
        matches!(self, Self::Relation(_))
    }
}

impl ValidateNode for Item {}

impl VisitableNode for Item {
    fn drive<V: Visitor>(&self, v: &mut V) {
        match self {
            Self::Id => {}
            Self::Is(node) => node.accept(v),
            Self::Relation(node) => node.accept(v),
        }
    }
}

///
/// ItemIs
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemIs {
    pub path: String,
}

impl ValidateNode for ItemIs {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // check path type
        let mut acceptable_types = HashSet::default();
        acceptable_types.extend(vec![
            TypeId::of::<Entity>(),
            TypeId::of::<Enum>(),
            TypeId::of::<Map>(),
            TypeId::of::<Newtype>(),
            TypeId::of::<Primitive>(),
            TypeId::of::<Record>(),
            TypeId::of::<Tuple>(),
        ]);
        errs.add_result(schema_read().check_node_types(&self.path, &acceptable_types));

        errs.result()
    }
}

impl VisitableNode for ItemIs {}

///
/// ItemRelation
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemRelation {
    pub path: String,
}

impl ValidateNode for ItemRelation {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // entity
        if let Some(entity) = schema_read().get_node::<Entity>(&self.path) {
            if !entity.is_relatable() {
                errs.add("entity does not meet the criteria to create a relation with");
            }
        }
        errs.add_result(schema_read().check_node::<Entity>(&self.path));

        errs.result()
    }
}

impl VisitableNode for ItemRelation {}
