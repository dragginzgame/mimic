use crate::orm::prelude::*;

pub mod role {
    use super::*;

    ///
    /// Basic
    ///

    #[role]
    pub struct Basic {}
}

///
/// PERMISSIONS
///

pub mod permission {
    use super::*;

    ///
    /// Basic
    ///

    #[permission]
    pub struct Basic {}
}
