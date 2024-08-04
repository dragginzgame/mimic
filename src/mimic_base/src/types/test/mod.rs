pub mod admin;
pub mod default;
pub mod sanitize;
pub mod store;
pub mod validate;

use crate::{canister, types};
use mimic::orm::prelude::*;

pub mod has_map {
    use super::*;

    ///
    /// HasMap
    ///

    #[entity(
        store = "canister::test::store::Data",
        fields(field(name = "map_int_string", value(item(is = "types::test::MapIntString"))))
    )]
    pub struct HasMap {}
}

///
/// MapIntString
///

#[map(key(is = "types::I32"), value(item(is = "types::String")))]
pub struct MapIntString {}
