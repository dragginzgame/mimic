pub mod case;

use crate::{core::traits::Sanitizer, design::prelude::*};

///
/// Trim
///

#[sanitizer]
pub struct Trim;

impl Sanitizer<String> for Trim {
    fn sanitize(&self, value: String) -> String {
        value.trim().to_string()
    }
}
