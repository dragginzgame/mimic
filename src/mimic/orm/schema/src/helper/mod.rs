mod reserved;

pub use reserved::is_reserved;

use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("empty ident"))]
    EmptyIdent,

    #[snafu(display("'{word}' is a reserved word"))]
    ReservedWord { word: String },
}

// validate_ident
pub fn validate_ident(ident: &str) -> Result<(), Error> {
    if ident.is_empty() {
        return Err(Error::EmptyIdent);
    }

    if reserved::is_reserved(ident) {
        return Err(Error::ReservedWord {
            word: ident.to_string(),
        });
    }

    Ok(())
}
