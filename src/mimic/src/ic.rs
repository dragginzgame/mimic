///
/// IMPORT IC CRATES
///

pub mod api {
    pub use ic_cdk::api::*;
}
pub mod mgmt {
    pub use ic_cdk::management_canister::*;
}
pub use ic_cdk::*;
pub mod structures {
    pub use ic_stable_structures::*;

    // helper
    pub type DefaultMemory = ic_stable_structures::memory_manager::VirtualMemory<
        ic_stable_structures::DefaultMemoryImpl,
    >;

    pub mod memory {
        pub use ic_stable_structures::memory_manager::*;
    }
}
