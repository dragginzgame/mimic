pub mod btreemap;
pub mod cell;

// re-export
pub mod memory {
    pub use ic_stable_structures::memory_manager::*;
    pub type VirtualMemory = ic_stable_structures::memory_manager::VirtualMemory<
        ic_stable_structures::DefaultMemoryImpl,
    >;
}
pub use ic_stable_structures::*;

// local
pub use {
    btreemap::BTreeMap,
    cell::{Cell, CellError},
};
