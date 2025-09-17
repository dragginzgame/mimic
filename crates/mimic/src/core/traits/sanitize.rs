use std::collections::{HashMap, HashSet};

///
/// Sanitize
///
/// A pre-processing step before persistence
/// (e.g. trimming whitespace, normalizing casing, deduplicating collections,
/// clamping numeric values, etc.)
///

pub trait Sanitize {
    fn sanitize(&mut self) {}
}

impl<T: Sanitize> Sanitize for Box<T> {
    fn sanitize(&mut self) {
        (**self).sanitize();
    }
}

impl<T: Sanitize> Sanitize for Option<T> {
    fn sanitize(&mut self) {
        if let Some(v) = self.as_mut() {
            v.sanitize();
        }
    }
}

impl<T: Sanitize> Sanitize for Vec<T> {
    fn sanitize(&mut self) {
        for v in self.iter_mut() {
            v.sanitize();
        }
    }
}

impl<T: Sanitize, S> Sanitize for HashSet<T, S> {
    fn sanitize(&mut self) {
        // Cannot safely mutate items in place
    }
}

impl<K: Sanitize, V: Sanitize, S> Sanitize for HashMap<K, V, S> {
    fn sanitize(&mut self) {
        for (_, v) in self.iter_mut() {
            // Cannot safely mutate keys in place
            v.sanitize();
        }
    }
}

// Blanket for primitives via macro
impl_primitive!(Sanitize);
