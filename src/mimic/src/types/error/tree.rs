use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

///
/// ErrorTree
/// structure to hold both the path and the errors, used in Visitor patterns
///

#[derive(
    CandidType, Clone, Debug, Default, Deref, DerefMut, Eq, PartialEq, Serialize, Deserialize,
)]
pub struct ErrorTree(HashMap<String, Vec<String>>);

impl ErrorTree {
    // new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // set
    pub fn set<S: AsRef<str>>(&mut self, key: S, error: S) {
        let key = key.as_ref().to_string();
        let error = error.as_ref().to_string();

        self.0.entry(key).or_default().push(error);
    }

    // set_list
    pub fn set_list(&mut self, key: &str, list: &[String]) {
        self.0
            .entry(key.to_string())
            .or_default()
            .extend(list.to_vec());
    }

    // result
    pub fn result(self) -> Result<(), Self> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }

    // len
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.values().map(Vec::len).sum()
    }

    // is_empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for ErrorTree {
    // written so to avoid trailing newlines
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (property, errors) in &self.0 {
            for error in errors {
                if property.is_empty() {
                    writeln!(f, "{error}")?;
                } else {
                    writeln!(f, "{property}: {error}")?;
                }
            }
        }

        Ok(())
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_length() {
        let mut errs = ErrorTree::new();
        errs.set("a", "error1");
        errs.set("b", "error2");
        errs.set("b", "error3");
        errs.set("b", "error4");
        assert_eq!(errs.len(), 4);
    }

    #[test]
    fn test_empty_check() {
        let mut errs = ErrorTree::new();
        assert!(errs.is_empty());
        errs.set("a", "error");
        assert!(!errs.is_empty());
    }
}
