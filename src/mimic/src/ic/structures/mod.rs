pub mod btreemap;
pub mod cell;
pub mod serialize;

// helper
pub type DefaultMemory =
    ic_stable_structures::memory_manager::VirtualMemory<ic_stable_structures::DefaultMemoryImpl>;

// re-export
pub use ic_stable_structures::*;
pub mod memory {
    pub use ic_stable_structures::memory_manager::*;
}

// local
pub use btreemap::BTreeMap;
pub use cell::{Cell, CellError};
