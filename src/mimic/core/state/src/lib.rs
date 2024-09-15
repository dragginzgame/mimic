pub mod app_state;
pub mod canister_state;
pub mod child_index;
pub mod subnet_index;
pub mod user_index;

pub use {
    app_state::{AppCommand, AppMode, AppState, AppStateManager},
    canister_state::{CanisterState, CanisterStateManager},
    child_index::{ChildIndex, ChildIndexManager},
    subnet_index::{SubnetIndex, SubnetIndexManager},
    user_index::{User, UserIndex, UserIndexManager},
};

use ic::structures::{
    memory::{MemoryId, MemoryManager},
    DefaultMemoryImpl,
};
use std::cell::RefCell;
use {
    app_state::AppStateStable, canister_state::CanisterStateStable, child_index::ChildIndexStable,
    subnet_index::SubnetIndexStable, user_index::UserIndexStable,
};

///
/// RUNTIME STATE
/// shared between all canisters
///
/// AppState and SubnetIndex live on root, and can be cached on other canisters
/// Every canister has its own CanisterState
///

// app
const APP_STATE_MEMORY_ID: u8 = 1;

// subnet
const SUBNET_INDEX_MEMORY_ID: u8 = 2;
const USER_INDEX_MEMORY_ID: u8 = 3;

// canister
const CANISTER_STATE_MEMORY_ID: u8 = 4;
const CHILD_INDEX_MEMORY_ID: u8 = 5;

thread_local! {

    ///
    /// MEMORY_MANAGER
    ///

    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    ///
    /// APP_STATE (root)
    ///
    /// Scope     : Application
    /// Structure : Cell
    ///
    /// a Cell that's only really meant for small data structures
    /// used for the global app state
    ///

    pub(crate) static APP_STATE: RefCell<AppStateStable> = RefCell::new(AppStateStable::init(
        MEMORY_MANAGER.with_borrow(|mm| mm.get(MemoryId::new(APP_STATE_MEMORY_ID))),
    ));

    ///
    /// SUBNET_INDEX (root)
    ///
    /// Scope     : Subnet
    /// Structure : BTreeMap
    ///

    pub(crate) static SUBNET_INDEX: RefCell<SubnetIndexStable> = RefCell::new(SubnetIndexStable::init(
        MEMORY_MANAGER.with_borrow(|mm| mm.get(MemoryId::new(SUBNET_INDEX_MEMORY_ID))),
    ));

    ///
    /// USER_INDEX
    ///
    /// Scope     : Subnet
    /// Structure : BTreeMap
    ///

    pub(crate) static USER_INDEX: RefCell<UserIndexStable> = RefCell::new(UserIndexStable::init(
        MEMORY_MANAGER.with_borrow(|mm| mm.get(MemoryId::new(USER_INDEX_MEMORY_ID))),
    ));

    ///
    /// CHILD_INDEX
    ///
    /// Scope     : Canister
    /// Structure : BTreeMap
    ///

    pub(crate) static CHILD_INDEX: RefCell<ChildIndexStable> = RefCell::new(ChildIndexStable::init(
        MEMORY_MANAGER.with_borrow(|mm| mm.get(MemoryId::new(CHILD_INDEX_MEMORY_ID))),
    ));

    ///
    /// CANISTER_STATE
    ///
    /// Scope     : Canister
    /// Structure : Cell
    ///

    pub(crate) static CANISTER_STATE: RefCell<CanisterStateStable> = RefCell::new(CanisterStateStable::init(
        MEMORY_MANAGER.with_borrow(|mm| mm.get(MemoryId::new(CANISTER_STATE_MEMORY_ID))),
    ));
}
