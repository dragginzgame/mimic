use crate::orm::{
    traits::{EntityDyn, SanitizeAuto, SanitizeManual, ValidateAuto, ValidateManual, Visitable},
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
    fn visit_mut(&mut self, item: &mut dyn Visitable, event: Event);

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

// perform_visit_mut
pub fn perform_visit_mut(visitor: &mut dyn Visitor, node: &mut dyn Visitable, key: &str) {
    visitor.push(key);
    visitor.visit_mut(node, Event::Enter);
    node.drive_mut(visitor);
    visitor.visit_mut(node, Event::Exit);
    visitor.pop();
}

///
/// SanitizeVisitor
///

#[derive(Debug, Default)]
pub struct SanitizeVisitor {
    pub errors: ErrorTree,
    pub path: Vec<String>,
}

impl SanitizeVisitor {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Visitor for SanitizeVisitor {
    fn visit(&mut self, _: &dyn Visitable, _: Event) {
        panic!("sanitize requires visit_mut");
    }

    fn visit_mut(&mut self, item: &mut dyn Visitable, event: Event) {
        match event {
            Event::Enter => match item.sanitize() {
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

    fn visit_mut(&mut self, _: &mut dyn Visitable, _: Event) {
        panic!("validate requires visit (not visit_mut)");
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

impl SanitizeManual for EntityAdapter<'_> {}
impl SanitizeAuto for EntityAdapter<'_> {}
impl ValidateManual for EntityAdapter<'_> {}
impl ValidateAuto for EntityAdapter<'_> {}

///
/// EntityAdapterMut
///

pub struct EntityAdapterMut<'a>(pub &'a mut dyn EntityDyn);

impl Visitable for EntityAdapterMut<'_> {
    fn drive_mut(&mut self, visitor: &mut dyn Visitor) {
        self.0.drive_mut(visitor);
    }
}

impl SanitizeManual for EntityAdapterMut<'_> {}
impl SanitizeAuto for EntityAdapterMut<'_> {}
impl ValidateManual for EntityAdapterMut<'_> {}
impl ValidateAuto for EntityAdapterMut<'_> {}
