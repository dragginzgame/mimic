use candid::CandidType;
use derive_more::{Deref, DerefMut, IntoIterator};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
    pub fn from_result<S: ToString>(res: Result<(), S>) -> Self {
        let mut errs = Self::new();
        errs.add_result(res);

        errs
    }

    // add
    pub fn add<S: ToString>(&mut self, s: S) {
        self.push(s.to_string());
    }

    // add_result
    pub fn add_result<S: ToString>(&mut self, res: Result<(), S>) {
        if let Err(s) = res {
            self.add(s.to_string());
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
