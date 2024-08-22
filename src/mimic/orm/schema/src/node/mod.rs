mod arg;
mod canister;
mod constant;
mod def;
mod entity;
mod r#enum;
mod enum_hash;
mod enum_value;
mod field;
mod fixture;
mod item;
mod map;
mod newtype;
mod permission;
mod primitive;
mod record;
mod role;
mod sanitizer;
mod schema;
mod sort_key;
mod store;
mod tuple;
mod r#type;
mod validator;
mod value;

pub use self::arg::*;
pub use self::canister::*;
pub use self::constant::*;
pub use self::def::*;
pub use self::entity::*;
pub use self::enum_hash::*;
pub use self::enum_value::*;
pub use self::field::*;
pub use self::fixture::*;
pub use self::item::*;
pub use self::map::*;
pub use self::newtype::*;
pub use self::permission::*;
pub use self::primitive::*;
pub use self::r#enum::*;
pub use self::r#type::*;
pub use self::record::*;
pub use self::role::*;
pub use self::sanitizer::*;
pub use self::schema::*;
pub use self::sort_key::*;
pub use self::store::*;
pub use self::tuple::*;
pub use self::validator::*;
pub use self::value::*;

use crate::{
    build::schema_read,
    visit::{Event, Visitor},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::any::Any;
use types::ErrorVec;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("error downcasting schema node: {path}"))]
    DowncastFail { path: String },

    #[snafu(display("{path} is an incorrect node type"))]
    IncorrectNodeType { path: String },

    #[snafu(display("path not found: {path}"))]
    PathNotFound { path: String },
}

impl Error {
    fn downcast_fail(path: &str) -> Self {
        Self::DowncastFail {
            path: path.to_string(),
        }
    }

    fn incorrect_node_type(path: &str) -> Self {
        Self::IncorrectNodeType {
            path: path.to_string(),
        }
    }

    fn path_not_found(path: &str) -> Self {
        Self::PathNotFound {
            path: path.to_string(),
        }
    }
}

///
/// NODE TRAITS
///

///
/// MacroNode
/// shared traits for every node that is created via a macro
/// as_any has to be implemented on each type manually
///

pub trait MacroNode: Any {
    fn as_any(&self) -> &dyn Any;
}

///
/// ValidateNode
///

pub trait ValidateNode {
    fn validate(&self) -> Result<(), ErrorVec> {
        Ok(())
    }
}

///
/// VisitableNode
///

pub trait VisitableNode: ValidateNode {
    // route_key
    fn route_key(&self) -> String {
        String::new()
    }

    // accept
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.push(&self.route_key());
        visitor.visit(self, Event::Enter);
        self.drive(visitor);
        visitor.visit(self, Event::Exit);
        visitor.pop();
    }

    // drive
    fn drive<V: Visitor>(&self, _: &mut V) {}
}

///
/// NODES
///

///
/// AccessPolicy
///

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AccessPolicy {
    Allow,
    Deny,
    Permission(String),
}

impl ValidateNode for AccessPolicy {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        match self {
            Self::Permission(permission) => {
                errs.add_result(schema_read().check_node::<Permission>(permission));
            }
            Self::Allow | Self::Deny => {}
        }

        errs.result()
    }
}

impl VisitableNode for AccessPolicy {}

///
/// Crud
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Crud {
    pub load: AccessPolicy,
    pub save: AccessPolicy,
    pub delete: AccessPolicy,
}

impl ValidateNode for Crud {}

impl VisitableNode for Crud {
    fn drive<V: Visitor>(&self, v: &mut V) {
        self.load.accept(v);
        self.save.accept(v);
        self.delete.accept(v);
    }
}
