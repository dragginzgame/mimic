use crate::{
    orm::{
        schema::{
            build::schema_read,
            node::{Canister, Def, EntityAcl, MacroNode, ValidateNode, VisitableNode},
            visit::Visitor,
        },
        types::ErrorVec,
    },
    utils::case::{Case, Casing},
};
use quote::format_ident;
use serde::{Deserialize, Serialize};
use std::ops::Range;
use syn::Ident;

///
/// CONSTS
///

pub const RESERVED_MEMORY_RANGE: Range<u8> = 0..19;

///
/// Store
///
/// A stable IC BTreeMap that stores Entity data
/// the name should be snake_case to keep the API consistent
///
/// entity_acl : the default entity ACL permissions for the entire store,
///              can be overwritten at the Entity level
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Store {
    pub def: Def,
    pub canister: String,
    pub memory_id: u8,
    pub entity_acl: EntityAcl,
}

impl Store {
    // cell_ident
    #[must_use]
    pub fn cell_ident(&self) -> Ident {
        format_ident!("STORE_{}", &self.def.ident.to_case(Case::Upper))
    }
}

impl MacroNode for Store {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Store {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // look up canister
        let res = schema_read().check_node::<Canister>(&self.canister);
        if let Err(e) = res {
            errs.add(e.to_string());
        }

        if RESERVED_MEMORY_RANGE.contains(&self.memory_id) {
            errs.add(format!(
                "store memory_id '{}' is within the reserved range {} to {}",
                &self.memory_id,
                RESERVED_MEMORY_RANGE.min().unwrap(),
                RESERVED_MEMORY_RANGE.max().unwrap()
            ));
        }

        errs.result()
    }
}

impl VisitableNode for Store {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        self.entity_acl.accept(v);
    }
}
