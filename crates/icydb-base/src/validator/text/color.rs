use crate::{core::traits::Validator, design::prelude::*};

///
/// RgbHex
///

const RGB_HEX_CONST: ::icydb_core::schema::node::Validator =
    ::icydb_core::schema::node::Validator {
        def: ::icydb_core::schema::node::Def {
            module_path: module_path!(),
            comments: Some("RgbHex"),
            ident: "RgbHex",
        },
    };
#[cfg(not(target_arch = "wasm32"))]
# [:: icydb_core :: export :: ctor :: ctor (anonymous , crate_path = :: icydb_core :: export :: ctor)]
fn __ctor() {
    ::icydb_schema::build::schema_write()
        .insert_node(::icydb_schema::node::SchemaNode::Validator(RGB_HEX_CONST));
}
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, Eq, PartialEq)]
#[doc = ""]
#[doc = " RgbHex"]
#[doc = ""]
#[debug]
pub struct RgbHex {}
impl ::icydb_core::core::traits::Visitable for RgbHex {}
impl ::icydb_core::core::traits::ValidateCustom for RgbHex {}
impl ::icydb_core::core::traits::SanitizeAuto for RgbHex {}
impl ::icydb_core::core::traits::Path for RgbHex {
    const PATH: &'static str = concat!(module_path!(), "::", stringify!(RgbHex));
}
impl ::icydb_core::core::traits::FieldValue for RgbHex {}
impl ::icydb_core::core::traits::ValidateAuto for RgbHex {}
impl ::icydb_core::core::traits::SanitizeCustom for RgbHex {}

///
/// RgbaHex
///

#[validator]
pub struct RgbaHex {}

impl Validator<str> for RgbaHex {
    fn validate(&self, s: &str) -> Result<(), String> {
        if s.len() == 8 && s.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(())
        } else {
            Err(format!(
                "RGBA string '{s}' should be 8 hexadecimal characters"
            ))
        }
    }
}
