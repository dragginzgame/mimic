mod delete;
mod load;
mod save;

pub use delete::*;
pub use load::*;
pub use save::*;

use thiserror::Error as ThisError;

//
// Collections are for internal data
// Response is external
//

///
/// ResponseError
///

#[derive(Debug, ThisError)]
pub enum ResponseError {
    #[error("no data found in collection")]
    EmptyCollection,
}
