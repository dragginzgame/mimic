use crate::{common::error::ErrorTree, core::traits::Visitable};

///
/// Event
///

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Enter,
    Exit,
}

///
/// PathSegment
///

#[derive(Debug)]
pub enum PathSegment {
    Empty,
    Field(String),
    Index(usize),
}

impl From<&str> for PathSegment {
    fn from(s: &str) -> Self {
        Self::Field(s.to_string())
    }
}

impl From<String> for PathSegment {
    fn from(s: String) -> Self {
        Self::Field(s)
    }
}

impl From<usize> for PathSegment {
    fn from(i: usize) -> Self {
        Self::Index(i)
    }
}

impl From<Option<&str>> for PathSegment {
    fn from(opt: Option<&str>) -> Self {
        match opt {
            Some(s) if !s.is_empty() => Self::Field(s.to_string()),
            _ => Self::Empty,
        }
    }
}

///
/// Visitor
/// plus helper functions that allow navigation of the tree in an object-safe way
///

pub trait Visitor {
    // nodes
    fn visit(&mut self, node: &dyn Visitable, event: Event);

    // path
    fn push(&mut self, _: PathSegment) {}
    fn pop(&mut self) {}
}

// perform_visit
#[inline]
pub fn perform_visit<S: Into<PathSegment>>(
    visitor: &mut dyn Visitor,
    node: &dyn Visitable,
    seg: S,
) {
    let seg = seg.into();
    visitor.push(seg);
    visitor.visit(node, Event::Enter);
    node.drive(visitor);
    visitor.visit(node, Event::Exit);
    visitor.pop();
}

///
/// ValidateVisitor
///

#[derive(Debug, Default)]
pub struct ValidateVisitor {
    pub errors: ErrorTree,
    pub path: Vec<PathSegment>,
}

impl ValidateVisitor {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn current_route(&self) -> String {
        self.path
            .iter()
            .filter_map(|seg| match seg {
                PathSegment::Empty => None,
                PathSegment::Field(s) => Some(s.to_string()),
                PathSegment::Index(i) => Some(i.to_string()),
            })
            .collect::<Vec<_>>()
            .join(".")
    }
}

impl Visitor for ValidateVisitor {
    #[inline]
    fn visit(&mut self, item: &dyn Visitable, event: Event) {
        match event {
            Event::Enter => {
                let mut errs = ErrorTree::new();

                // combine all validation types
                // better to do it here and not in the trait
                if let Err(e) = item.validate_self() {
                    errs.merge(e);
                }
                if let Err(e) = item.validate_children() {
                    errs.merge(e);
                }
                if let Err(e) = item.validate_custom() {
                    errs.merge(e);
                }

                // check for errs
                if !errs.is_empty() {
                    if self.path.is_empty() {
                        // At the current level, merge directly.
                        self.errors.merge(errs);
                    } else {
                        // Add to a child entry under the computed route.
                        let route = self.current_route();
                        self.errors.children.entry(route).or_default().merge(errs);
                    }
                }
            }
            Event::Exit => {}
        }
    }

    #[inline]
    fn push(&mut self, seg: PathSegment) {
        self.path.push(seg);
    }

    #[inline]
    fn pop(&mut self) {
        self.path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        traits::{ValidateAuto, ValidateCustom, Visitable},
        validate::validate,
    };
    use mimic_common::error::ErrorTree;

    const ERR_MSG: &str = "leaf error";

    // A simple leaf type that can emit an error based on a flag.
    #[derive(Debug, Clone, Default)]
    struct Leaf(bool);

    impl ValidateAuto for Leaf {}
    impl ValidateCustom for Leaf {
        fn validate_custom(&self) -> Result<(), ErrorTree> {
            if self.0 {
                Err(ErrorTree::from(ERR_MSG))
            } else {
                Ok(())
            }
        }
    }

    impl Visitable for Leaf {}

    // Helper: get flattened errors from validate()
    fn flatten_errs(node: &dyn Visitable) -> Vec<(String, String)> {
        match validate(node) {
            Ok(()) => Vec::new(),
            Err(crate::core::validate::ValidationError::ValidationFailed(tree)) => {
                tree.flatten_ref()
            }
        }
    }

    // Root-level error should attach to the root (empty route)
    #[test]
    fn root_level_error_is_at_root() {
        let leaf = Leaf(true);
        let flat = flatten_errs(&leaf);

        assert!(flat.iter().any(|(k, v)| k.is_empty() && v == ERR_MSG));
    }

    // Container with a Vec field should index items: "field.1"
    #[test]
    fn record_field_vec_item_path_is_indexed() {
        #[derive(Debug, Default)]
        struct Container {
            nums: Vec<Leaf>,
        }

        impl Visitable for Container {
            fn drive(&self, visitor: &mut dyn Visitor) {
                // Record field key then Vec indices
                perform_visit(visitor, &self.nums, "nums");
            }
        }
        impl ValidateAuto for Container {}
        impl ValidateCustom for Container {}

        let node = Container {
            nums: vec![Leaf(false), Leaf(true), Leaf(false)],
        };

        let flat = flatten_errs(&node);
        assert!(flat.iter().any(|(k, v)| k == "nums.1" && v == ERR_MSG));
    }

    // Nested record, tuple-like, and map-like structures should produce dotted keys.
    #[test]
    fn nested_record_tuple_map_paths_are_dotted() {
        // Inner record with a single leaf field
        #[derive(Debug, Default)]
        struct Inner {
            leaf: Leaf,
        }

        impl Visitable for Inner {
            fn drive(&self, visitor: &mut dyn Visitor) {
                perform_visit(visitor, &self.leaf, "leaf");
            }
        }
        impl ValidateAuto for Inner {}
        impl ValidateCustom for Inner {}

        // Tuple-like struct with two leaves; use indices "0", "1"
        #[derive(Debug, Default)]
        struct Tup2(Leaf, Leaf);

        impl Visitable for Tup2 {
            fn drive(&self, visitor: &mut dyn Visitor) {
                perform_visit(visitor, &self.0, 0);
                perform_visit(visitor, &self.1, 1);
            }
        }
        impl ValidateAuto for Tup2 {}
        impl ValidateCustom for Tup2 {}

        // Simple map-like wrapper iterating key/value pairs
        #[derive(Debug, Default)]
        struct MyMap(Vec<(String, Leaf)>);

        impl Visitable for MyMap {
            fn drive(&self, visitor: &mut dyn Visitor) {
                for (_k, v) in &self.0 {
                    // Align with macro-generated map visitor: push "value"
                    perform_visit(visitor, v, "value");
                }
            }
        }
        impl ValidateAuto for MyMap {}
        impl ValidateCustom for MyMap {}

        #[derive(Debug, Default)]
        struct Outer {
            rec: Inner,
            tup: Tup2,
            map: MyMap,
        }

        impl Visitable for Outer {
            fn drive(&self, visitor: &mut dyn Visitor) {
                perform_visit(visitor, &self.rec, "rec");
                perform_visit(visitor, &self.tup, "tup");
                perform_visit(visitor, &self.map, "map");
            }
        }
        impl ValidateAuto for Outer {}
        impl ValidateCustom for Outer {}

        let node = Outer {
            rec: Inner { leaf: Leaf(true) },
            tup: Tup2(Leaf(false), Leaf(true)),
            map: MyMap(vec![("k".to_string(), Leaf(true))]),
        };

        let flat = flatten_errs(&node);

        // Expect errors at specific dotted paths
        assert!(flat.iter().any(|(k, v)| k == "rec.leaf" && v == ERR_MSG));
        assert!(flat.iter().any(|(k, v)| k == "tup.1" && v == ERR_MSG));
        assert!(flat.iter().any(|(k, v)| k == "map.value" && v == ERR_MSG));
    }
}
