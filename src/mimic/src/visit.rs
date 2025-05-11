use crate::{
    traits::{EntityDyn, ValidateAuto, ValidateCustom, Visitable},
    types::ErrorTree,
};

///
/// Event
///

#[derive(Debug)]
pub enum Event {
    Enter,
    Exit,
}

///
/// Visitor
/// plus helper functions that allow navigation of the tree in an object-safe way
///

pub trait Visitor {
    // nodes
    fn visit(&mut self, item: &dyn Visitable, event: Event);

    // path
    fn push(&mut self, _: &str) {}
    fn pop(&mut self) {}
}

// perform_visit
pub fn perform_visit(visitor: &mut dyn Visitor, node: &dyn Visitable, key: &str) {
    visitor.push(key);
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
    pub path: Vec<String>,
}

impl ValidateVisitor {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl ValidateVisitor {
    fn current_route(&self) -> String {
        self.path
            .iter()
            .filter(|s| !s.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join(".")
    }
}

impl Visitor for ValidateVisitor {
    fn visit(&mut self, item: &dyn Visitable, event: Event) {
        match event {
            Event::Enter => {
                if let Err(errs) = item.validate() {
                    if !errs.is_empty() {
                        let route = self.current_route();

                        if route.is_empty() {
                            // At the current level, merge directly.
                            self.errors.merge(errs);
                        } else {
                            // Add to a child entry under the computed route.
                            self.errors.children.entry(route).or_default().merge(errs);
                        }
                    }
                }
            }
            Event::Exit => {}
        }
    }

    fn push(&mut self, s: &str) {
        self.path.push(s.to_string());
    }

    fn pop(&mut self) {
        self.path.pop();
    }
}

///
/// EntityAdapter
///
/// because no dynamic trait upscaling
/// `<https://github.com/rust-lang/rust/issues/65991>`
///

pub struct EntityAdapter<'a>(pub &'a dyn EntityDyn);

impl Visitable for EntityAdapter<'_> {
    fn drive(&self, visitor: &mut dyn Visitor) {
        self.0.drive(visitor);
    }
}

impl ValidateAuto for EntityAdapter<'_> {}
impl ValidateCustom for EntityAdapter<'_> {}
