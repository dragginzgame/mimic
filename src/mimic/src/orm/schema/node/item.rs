use crate::orm::{
    schema::{
        build::schema_read,
        node::{
            Entity, Enum, Map, Newtype, Primitive, Record, Selector, Tuple, ValidateNode,
            VisitableNode,
        },
    },
    types::ErrorVec,
};
use serde::{Deserialize, Serialize};
use std::{any::TypeId, collections::HashSet, ops::Not, sync::LazyLock};

///
/// Item
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(default, skip_serializing_if = "Not::not")]
    pub indirect: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

impl Item {
    // is_relation
    #[must_use]
    pub const fn is_relation(&self) -> bool {
        self.relation.is_some()
    }
}

// define acceptable types for an 'is' Item
static ACCEPTABLE_TYPES: LazyLock<HashSet<TypeId>> = LazyLock::new(|| {
    let mut acceptable_types = HashSet::new();
    acceptable_types.extend(vec![
        TypeId::of::<Entity>(),
        TypeId::of::<Enum>(),
        TypeId::of::<Map>(),
        TypeId::of::<Newtype>(),
        TypeId::of::<Primitive>(),
        TypeId::of::<Record>(),
        TypeId::of::<Tuple>(),
    ]);
    acceptable_types
});

impl ValidateNode for Item {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // both
        if self.is.is_some() == self.relation.is_some() {
            errs.add("only one of is or relation should be set");
        }

        // is
        if let Some(path) = &self.is {
            errs.add_result(schema_read().check_node_types(path, &ACCEPTABLE_TYPES));
        }

        // relation
        if let Some(path) = &self.relation {
            if self.indirect {
                errs.add("relations cannot be set to indirect");
            }

            errs.add_result(schema_read().check_node::<Entity>(path));
        }

        // selector
        if let Some(selector) = &self.selector {
            if schema_read().get_node::<Selector>(selector).is_none() {
                errs.add(format!("selector path '{selector}' not found"));
            }
        }

        errs.result()
    }
}

impl VisitableNode for Item {}
