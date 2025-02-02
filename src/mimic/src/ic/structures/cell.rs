use crate::{
    ic::{
        structures::{DefaultMemory, Storable},
        IcError,
    },
    Error,
};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use ic_stable_structures::cell::Cell as WrappedCell;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

///
/// CellError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum CellError {
    #[error("{0}")]
    Init(String),

    #[error("value too large")]
    ValueTooLarge,
}

///
/// Cell
/// a wrapper around Cell that uses the default DefaultMemory
///

#[derive(Deref, DerefMut)]
pub struct Cell<T>
where
    T: Clone + Storable,
{
    data: WrappedCell<T, DefaultMemory>,
}

impl<T> Cell<T>
where
    T: Clone + Storable,
{
    // new
    pub fn new(memory: DefaultMemory, value: T) -> Result<Self, Error> {
        let data = WrappedCell::new(memory, value)
            .map_err(|_| CellError::ValueTooLarge)
            .map_err(IcError::CellError)?;

        Ok(Self { data })
    }

    // init
    pub fn init(memory: DefaultMemory, default_value: T) -> Result<Self, Error> {
        let data = WrappedCell::init(memory, default_value)
            .map_err(|e| CellError::Init(e.to_string()))
            .map_err(IcError::CellError)?;

        Ok(Self { data })
    }

    // get
    // clones to make non-Copy structures easier to use
    pub fn get(&self) -> T {
        self.data.get().clone()
    }

    // set
    pub fn set(&mut self, value: T) -> Result<T, Error> {
        let res = self
            .data
            .set(value)
            .map_err(|_| CellError::ValueTooLarge)
            .map_err(IcError::CellError)?;

        Ok(res)
    }
}
