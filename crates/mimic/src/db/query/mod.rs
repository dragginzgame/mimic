mod delete;
mod filter;
mod load;
mod planner;
mod save;
mod sort;

pub use delete::*;
pub use filter::*;
pub use load::*;
pub use planner::*;
pub use save::*;
pub use sort::*;

// load
#[must_use]
pub fn load() -> LoadQuery {
    LoadQuery::new()
}

// delete
#[must_use]
pub fn delete() -> DeleteQuery {
    DeleteQuery::new()
}

// create
#[must_use]
pub fn create() -> SaveQuery {
    SaveQuery::new(SaveMode::Create)
}

// update
#[must_use]
pub fn update() -> SaveQuery {
    SaveQuery::new(SaveMode::Update)
}

// replace
#[must_use]
pub fn replace() -> SaveQuery {
    SaveQuery::new(SaveMode::Replace)
}
