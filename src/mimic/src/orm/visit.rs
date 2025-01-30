use crate::orm::{
    traits::{EntityDyn, ValidateAuto, ValidateManual, Visitable},
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
///
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

impl Visitor for ValidateVisitor {
    fn visit(&mut self, item: &dyn Visitable, event: Event) {
        match event {
            Event::Enter => match item.validate() {
                Ok(()) => {}
                Err(errs) => {
                    if !errs.is_empty() {
                        let key = self
                            .path
                            .iter()
                            .filter(|s| !s.is_empty())
                            .cloned()
                            .collect::<Vec<String>>()
                            .join(".");

                        self.errors.set_list(&key, &errs);
                    }
                }
            },
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

impl ValidateManual for EntityAdapter<'_> {}
impl ValidateAuto for EntityAdapter<'_> {}
