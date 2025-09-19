use crate::core::{
    traits::Visitable,
    visit::{Event, VisitorMut},
};

///
/// SanitizeVisitor
/// Walks a tree and applies `sanitize_self` + `sanitize_children` + `sanitize_custom`
/// on every node.
///

#[derive(Debug, Default)]
pub struct SanitizeVisitor;

impl SanitizeVisitor {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

///
/// perform_visit_mut
/// like `perform_visit` but takes &mut for in-place mutation
///

impl VisitorMut for SanitizeVisitor {
    fn visit(&mut self, node: &mut dyn Visitable, event: Event) {
        if matches!(event, Event::Enter) {
            node.sanitize();
        }
    }
}
