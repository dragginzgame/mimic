pub mod role {
    use super::*;

    ///
    /// Basic
    ///

    #[role()]
    pub struct Basic {}
}

///
/// PERMISSIONS
///

pub mod permission {
    pub use super::*;

    ///
    /// Basic
    ///

    #[permission]
    pub struct Basic {}
}
