mod arg;
mod constant;
mod def;
mod entity;
mod r#enum;
mod enum_value;
mod field;
mod index;
mod item;
mod map;
mod newtype;
mod primitive;
mod record;
mod schema;
mod selector;
mod sort_key;
mod tuple;
mod r#type;
mod validator;
mod value;

pub use self::arg::*;
pub use self::constant::*;
pub use self::def::*;
pub use self::entity::*;
pub use self::enum_value::*;
pub use self::field::*;
pub use self::index::*;
pub use self::item::*;
pub use self::map::*;
pub use self::newtype::*;
pub use self::primitive::*;
pub use self::r#enum::*;
pub use self::r#type::*;
pub use self::record::*;
pub use self::schema::*;
pub use self::selector::*;
pub use self::sort_key::*;
pub use self::tuple::*;
pub use self::validator::*;
pub use self::value::*;

use crate::orm::{
    schema::visit::{Event, Visitor},
    types::ErrorVec,
};
use std::any::Any;

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
