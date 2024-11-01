use crate::orm::schema::build::schema_builder;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("empty ident"))]
    EmptyIdent,

    #[snafu(display("'{prefix}' is a reserved prefix"))]
    ReservedPrefix { prefix: String },

    #[snafu(display("'{word}' is a reserved word"))]
    ReservedWord { word: String },
}

// validate_ident
pub fn validate_ident(ident: &str) -> Result<(), Error> {
    if ident.is_empty() {
        return Err(Error::EmptyIdent);
    }

    // reserved?
    is_reserved(ident)?;

    Ok(())
}

// is_reserved
pub fn is_reserved(word: &str) -> Result<(), Error> {
    if has_reserved_prefix(word) {
        return Err(Error::ReservedPrefix {
            prefix: word.to_string(),
        });
    }

    if is_reserved_word(word) {
        return Err(Error::ReservedWord {
            word: word.to_string(),
        });
    }

    Ok(())
}

// has_reserved_prefix
fn has_reserved_prefix(s: &str) -> bool {
    schema_builder()
        .reserved_prefixes
        .iter()
        .any(|&prefix| s.starts_with(prefix))
}

// is_reserved_word
fn is_reserved_word(s: &str) -> bool {
    schema_builder().reserved_words.contains(s)
}
