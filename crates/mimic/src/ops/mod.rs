pub mod serialize;
pub mod traits;
pub mod validate;
pub mod visit;

use std::collections::HashMap;

///
/// EntityValues
///

#[derive(Debug)]
pub struct EntityValues {
    map: HashMap<&'static str, Option<String>>,
}

impl EntityValues {
    /// Returns Some(values) if all fields are present, or None if any are missing or None
    #[must_use]
    pub fn collect_all(&self, fields: &[&'static str]) -> Option<Vec<String>> {
        let mut values = Vec::with_capacity(fields.len());

        for &field in fields {
            match self.map.get(field) {
                Some(Some(v)) => values.push(v.clone()),
                _ => return None, // required field missing or None
            }
        }

        Some(values)
    }

    /// Checks if all given fields are present
    #[must_use]
    pub fn has_all(&self, fields: &[&'static str]) -> bool {
        fields.iter().all(|f| self.map.contains_key(f))
    }

    /// Access a field directly
    #[must_use]
    pub fn get(&self, field: &str) -> Option<&Option<String>> {
        self.map.get(field)
    }
}

impl From<HashMap<&'static str, Option<String>>> for EntityValues {
    fn from(map: HashMap<&'static str, Option<String>>) -> Self {
        EntityValues { map }
    }
}
