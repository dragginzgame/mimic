pub use crate::prelude::*;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("length of {len} is not equal to {to}"))]
    NotEqual { len: usize, to: usize },

    #[snafu(display("length of {len} is lower than minimum of {min}"))]
    BelowMinimum { len: usize, min: usize },

    #[snafu(display("length of {len} exceeds the maximum of {max}"))]
    AboveMaximum { len: usize, max: usize },

    #[snafu(display("conversion error"))]
    Conversion,
}

///
/// Len
/// Trait implemented on foreign types
///

pub trait Len {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Len for String {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for [T] {
    fn len(&self) -> usize {
        self.len()
    }
}

///
/// Equal
///

#[validator]
pub struct Equal {}

impl Equal {
    pub fn validate<T: Len>(t: &T, to: isize) -> Result<(), Error> {
        let len = t.len();
        let to = usize::try_from(to).map_err(|_| Error::Conversion)?;

        if len == to {
            Ok(())
        } else {
            Err(Error::NotEqual { len, to })
        }
    }
}

///
/// Min
///

#[validator]
pub struct Min {}

impl Min {
    pub fn validate<T: Len>(t: &T, min: isize) -> Result<(), Error> {
        let len = t.len();
        let min = usize::try_from(min).map_err(|_| Error::Conversion)?;

        if len < min {
            Err(Error::BelowMinimum { len, min })
        } else {
            Ok(())
        }
    }
}

///
/// Max
///

#[validator]
pub struct Max {}

impl Max {
    pub fn validate<T: Len>(t: &T, max: isize) -> Result<(), Error> {
        let len = t.len();
        let max = usize::try_from(max).map_err(|_| Error::Conversion)?;

        if len > max {
            Err(Error::AboveMaximum { len, max })
        } else {
            Ok(())
        }
    }
}

///
/// Range
///

#[validator]
pub struct Range {}

impl Range {
    pub fn validate<T: Len>(t: &T, min: isize, max: isize) -> Result<(), Error> {
        let len = t.len();
        let min = usize::try_from(min).map_err(|_| Error::Conversion)?;
        let max = usize::try_from(max).map_err(|_| Error::Conversion)?;

        if len < min {
            Err(Error::BelowMinimum { len, min })
        } else if len > max {
            Err(Error::AboveMaximum { len, max })
        } else {
            Ok(())
        }
    }
}
