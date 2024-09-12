pub use core_state::*;

///
/// STATE LOOKUP
/// immutable clones of the state structures
///

// app_state
#[must_use]
pub fn app_state() -> AppState {
    AppStateManager::get()
}

// canister_state
#[must_use]
pub fn canister_state() -> CanisterState {
    CanisterStateManager::get()
}

// child_index
#[must_use]
pub fn child_index() -> ChildIndex {
    ChildIndexManager::get()
}

// subnet_index
#[must_use]
pub fn subnet_index() -> SubnetIndex {
    SubnetIndexManager::get()
}

// user_index
#[must_use]
pub fn user_index() -> UserIndex {
    UserIndexManager::get()
}
