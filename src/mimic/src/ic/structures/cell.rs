use crate::ic::structures::{memory::VirtualMemory, Storable};
use derive_more::{Deref, DerefMut};
use ic_stable_structures::cell::{Cell as WrappedCell, InitError, ValueError};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// CellError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum CellError {
    #[snafu(display("init error: {error}"))]
    Init { error: String },

    #[snafu(display("value too large: {size}"))]
    ValueTooLarge { size: u64 },
}

impl From<InitError> for CellError {
    fn from(error: InitError) -> Self {
        Self::Init {
            error: error.to_string(),
        }
    }
}

impl From<ValueError> for CellError {
    fn from(error: ValueError) -> Self {
        match error {
            ValueError::ValueTooLarge { value_size } => Self::ValueTooLarge { size: value_size },
        }
    }
}

///
/// Cell
/// a wrapper around Cell that uses the default VirtualMemory
///

#[derive(Deref, DerefMut)]
pub struct Cell<T>
where
    T: Clone + Storable,
{
    data: WrappedCell<T, VirtualMemory>,
}

impl<T> Cell<T>
where
    T: Clone + Storable,
{
    // new
    pub fn new(memory: VirtualMemory, value: T) -> Result<Self, CellError> {
        let data = WrappedCell::new(memory, value)?;

        Ok(Self { data })
    }

    // init
    pub fn init(memory: VirtualMemory, default_value: T) -> Result<Self, CellError> {
        let data = WrappedCell::init(memory, default_value)?;

        Ok(Self { data })
    }

    // get
    // clones to make non-Copy structures easier to use
    pub fn get(&self) -> T {
        self.data.get().clone()
    }

    // set
    pub fn set(&mut self, value: T) -> Result<T, CellError> {
        let res = self.data.set(value)?;

        Ok(res)
    }
}
