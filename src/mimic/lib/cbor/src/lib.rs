use ciborium::{de::from_reader, ser::into_writer, Value};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Debug;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("serialize error: {msg}"))]
    Serialize { msg: String },

    #[snafu(display("deserialize error: {msg}"))]
    Deserialize { msg: String },
}

// serialize
pub fn serialize<T>(ty: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    let mut writer = Vec::<u8>::new();
    into_writer(ty, &mut writer).map_err(|e| Error::Serialize { msg: e.to_string() })?;

    Ok(writer)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    from_reader(bytes).map_err(|e| {
        // attempt to deserialize into a more generic Value for debugging
        match from_reader::<Value, _>(bytes) {
            Ok(value) => Error::Deserialize {
                msg: format!("failed to deserialize: {e} ({value:?})"),
            },
            Err(debug_error) => Error::Deserialize {
                msg: format!("failed to deserialize: {e}. DEBUG FAILED {debug_error}"),
            },
        }
    })
}
