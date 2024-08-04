///
/// mimic_common
/// common libraries used by both App and Framework
///
/// TODO - this should mirror mimi:: as much as possible
///
pub use ic;

pub mod case {
    pub use lib_case::*;
}

pub mod cbor {
    pub use lib_cbor::*;
}

pub mod rand {
    pub use lib_rand::*;
}

pub mod time {
    pub use lib_time::*;
}
