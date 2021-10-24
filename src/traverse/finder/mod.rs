mod pattern;
pub use pattern::{Item as PatternItem, Pattern, PatternError};

use crate::traverse::visitor::{Item as VisitorItem, Observer, Visitor};
use crate::Node;

/// A struct to find sub-nodes in AST by by a given `Pattern`
#[derive(Debug)]
pub struct Finder<'a> {
    looking_for: Pattern,
    current_path: Pattern,
    result: Option<&'a Node<'a>>,
}

impl<'a> Finder<'a> {
    /// Performs a search of a given pattern on a given AST.
    ///
    /// `looking_for` is a string slice that is used to construct a `Pattern`.
    pub fn run(
        looking_for: &str,
        root: &'a Node<'a>,
    ) -> Result<Option<&'a Node<'a>>, PatternError> {
        let looking_for = Pattern::new(looking_for)?;
        let mut visitor = Visitor {
            observer: Self {
                looking_for,
                current_path: Pattern::empty(),
                result: None,
            },
        };
        visitor.visit_root(root);
        Ok(visitor.observer.result)
    }
}

impl<'a> Observer<'a> for Finder<'a> {
    fn on_node(&mut self, node: &'a Node<'a>) {
        if self.current_path == self.looking_for {
            self.result = Some(node);
        }
    }

    fn on_subitem(&mut self, subitem: VisitorItem) {
        self.current_path.push(PatternItem::VisitorItem(subitem))
    }

    fn on_subitem_moving_up(&mut self, _: VisitorItem) {
        self.current_path.pop().unwrap();
    }
}

// #[cfg(test)]
// mod tests;
