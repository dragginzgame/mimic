use crate::{core::traits::ValidatorString, design::prelude::*};

///
/// MimeType
///

#[validator]
pub struct MimeType {}

impl ValidatorString for MimeType {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(format!("MIME type '{s}' must contain exactly one '/'"));
        }

        let is_valid_part = |part: &str| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "+.-".contains(c))
        };

        if !is_valid_part(parts[0]) || !is_valid_part(parts[1]) {
            return Err(format!(
                "MIME type '{s}' contains invalid characters; only alphanumeric, '+', '-', '.' allowed"
            ));
        }

        Ok(())
    }
}

///
/// Url
///

#[cfg(not(target_arch = "wasm32"))]
#[::mimic::export::ctor::ctor]
fn ctor_1913848862() {
    ::mimic::schema::build::schema_write().insert_node(
        ::mimic::schema::node::SchemaNode::Validator(::mimic::schema::node::Validator {
            def: ::mimic::schema::node::Def {
                module_path: module_path!(),
                comments: Some("Url"),
                ident: "Url",
            },
            fields: ::mimic::schema::node::FieldList { fields: &[] },
        }),
    );
}
#[derive(Clone, Debug, Default)]
pub struct Url {}
const PATH: &'static str = concat!(module_path!(), "::", "Url");

impl ValidatorString for Url {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        // Very basic check — can be expanded
        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(())
        } else {
            Err(format!("URL '{s}' must start with 'http://' or 'https://'"))
        }
    }
}
