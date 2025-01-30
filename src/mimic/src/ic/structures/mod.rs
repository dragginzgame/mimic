pub mod btreemap;
pub mod cell;
pub mod serialize;

pub type DefaultMemory =
    ic_stable_structures::memory_manager::VirtualMemory<ic_stable_structures::DefaultMemoryImpl>;

// re-export
pub use ic_stable_structures::*;

// local
pub use btreemap::BTreeMap;
pub use cell::{Cell, CellError};
