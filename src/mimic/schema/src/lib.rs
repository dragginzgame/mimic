pub mod auth;
pub mod entity;

pub use auth::AuthService;

use schema::node::Schema;
use std::sync::LazyLock;

///
/// SCHEMA
///

pub static SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    let json: &'static str = include_str!("../../../../generated/schema/schema.json");

    serde_json::from_str::<Schema>(json).unwrap()
});
