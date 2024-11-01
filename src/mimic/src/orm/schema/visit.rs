use super::node::VisitableNode;
use crate::orm::types::ErrorTree;

///
/// Event
/// fine to redeclare this every time we use the visitor pattern
/// makes no sense to rely on such a simple dependency
///

#[derive(Debug)]
pub enum Event {
    Enter,
    Exit,
}

///
/// Visitor
///

pub trait Visitor {
    // name
    fn name(&self) -> &'static str;

    // visit
    fn visit<V: VisitableNode + ?Sized>(&mut self, _: &V, _: Event) {}

    // key
    fn push(&mut self, _: &str) {}
    fn pop(&mut self) {}
}

///
/// Validator
/// (Visitor)
///

#[derive(Debug, Default)]
pub struct Validator {
    errors: ErrorTree,
    route: Vec<String>,
    node_count: usize,
}

impl Validator {
    #[must_use]
    pub fn new() -> Self {
        Self {
            errors: ErrorTree::new(),
            ..Default::default()
        }
    }

    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }

    #[must_use]
    pub fn errors(&self) -> ErrorTree {
        self.errors.clone()
    }
}

impl Visitor for Validator {
    fn name(&self) -> &'static str {
        "Validator"
    }

    fn visit<T: VisitableNode + ?Sized>(&mut self, node: &T, event: Event) {
        match event {
            Event::Enter => {
                self.node_count += 1;

                match node.validate() {
                    Ok(()) => {}
                    Err(errs) => {
                        if !errs.is_empty() {
                            let route = &self
                                .route
                                .iter()
                                .filter(|s| !s.is_empty())
                                .cloned()
                                .collect::<Vec<String>>()
                                .join(" -> ");

                            self.errors.set_list(route, &errs);
                        }
                    }
                }
            }
            Event::Exit => {}
        }
    }

    fn push(&mut self, s: &str) {
        self.route.push(s.to_string());
    }

    fn pop(&mut self) {
        self.route.pop();
    }
}
