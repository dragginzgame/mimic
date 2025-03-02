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
        if self.is_empty() { Ok(()) } else { Err(self) }
    }
}

impl<T: Display> From<T> for ErrorVec {
    fn from(item: T) -> Self {
        Self(vec![item.to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_empty() {
        let ev = ErrorVec::new();
        // Ensure the new ErrorVec is empty.
        assert!(ev.is_empty());
        // And that calling result() returns Ok(()), since there are no errors.
        assert_eq!(ev.result(), Ok(()));
    }

    #[test]
    fn test_add_error() {
        let mut ev = ErrorVec::new();
        ev.add("error message");
        // Check that the error message was added.
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0], "error message".to_string());
        // result() should now return an error.
        let result = ev.result();
        assert!(result.is_err());
    }

    #[test]
    fn test_add_result() {
        let mut ev = ErrorVec::new();
        // Adding an Ok result should not add any error.
        let ok_res: Result<(), &str> = Ok(());
        ev.add_result(ok_res);
        assert!(ev.is_empty());
        // Adding an Err result should add the error message.
        let err_res: Result<(), &str> = Err("failed");
        ev.add_result(err_res);
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0], "failed".to_string());
    }

    #[test]
    fn test_merge_errors() {
        let mut ev = ErrorVec::new();
        ev.add("first error");
        // Create another ErrorVec as an Err variant.
        let merge_err: Result<(), ErrorVec> = Err(ErrorVec(vec![
            "second error".to_string(),
            "third error".to_string(),
        ]));
        ev.merge(merge_err);
        // Check that errors from both sources are present.
        assert_eq!(ev.len(), 3);
        assert_eq!(ev[0], "first error".to_string());
        assert_eq!(ev[1], "second error".to_string());
        assert_eq!(ev[2], "third error".to_string());
    }

    #[test]
    fn test_from_display() {
        // Test that the From<T: Display> implementation works.
        let ev: ErrorVec = 42.into();
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0], "42".to_string());
    }
}
