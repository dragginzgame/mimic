use candid::CandidType;
use derive_more::{Deref, DerefMut, IntoIterator};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display};

///
/// ErrorVec
/// a Vec of Errors to be used by validation or other purposes
///

#[derive(
    CandidType, Debug, Default, Deref, DerefMut, Eq, IntoIterator, PartialEq, Serialize, Deserialize,
)]
pub struct ErrorVec(Vec<String>);

impl ErrorVec {
    // new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // from_result
    pub fn from_result<E: Error>(result: Result<(), E>) -> Self {
        let mut errs = Self::new();
        errs.add_result(result);

        errs
    }

    // add
    pub fn add<S: Into<String>>(&mut self, s: S) {
        self.push(s.into());
    }

    // add_result
    pub fn add_result<E: Error>(&mut self, result: Result<(), E>) {
        if let Err(e) = result {
            self.add(e.to_string());
        }
    }

    // merge
    pub fn merge(&mut self, result: Result<(), Self>) {
        if let Err(errors) = result {
            self.extend(errors);
        }
    }

    // result
    pub fn result(self) -> Result<(), Self> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl<T: Display> From<T> for ErrorVec {
    fn from(item: T) -> Self {
        Self(vec![item.to_string()])
    }
}
