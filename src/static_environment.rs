use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

/// Stack of local variables in nested scopes
///
/// Each scope represents a Ruby scope:
///
/// ```test
/// # 1
/// class A
///   # 1, 2
///   def m
///     # 1, 2, 3
///   end
///   # 1, 2
/// end
/// # 1
/// ```
///
/// In the example above comments show what's in the stack.
/// Basically, it's pushed when you enter a new scope
/// and it's popped when exit it.
#[derive(Debug, Clone, Default)]
pub struct StaticEnvironment {
    variables: Rc<RefCell<BTreeSet<String>>>,
    stack: Rc<RefCell<Vec<BTreeSet<String>>>>,
}

const FORWARD_ARGS: &str = "FORWARD_ARGS";

impl StaticEnvironment {
    /// Constructor
    pub fn new() -> Self {
        Self {
            variables: Rc::new(RefCell::new(BTreeSet::new())),
            stack: Rc::new(RefCell::new(vec![])),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.stack.borrow().is_empty()
    }

    #[allow(dead_code)]
    pub(crate) fn reset(&self) {
        self.variables.borrow_mut().clear();
        self.stack.borrow_mut().clear();
    }

    /// Performs a push, doesn't inherit previously declared variables in the new scope
    ///
    /// Handles class/module scopes
    pub fn extend_static(&self) {
        let variables = std::mem::take(&mut *self.variables.borrow_mut());
        self.stack.borrow_mut().push(variables);
    }

    /// Performs a puch, inherits previously declared variables in the new scope
    ///
    /// Handles block/lambda scopes
    pub fn extend_dynamic(&self) {
        self.stack
            .borrow_mut()
            .push(self.variables.borrow().clone());
    }

    /// Performs pop
    pub fn unextend(&self) {
        *self.variables.borrow_mut() = self
            .stack
            .borrow_mut()
            .pop()
            .expect("expected static_env to have at least one frame");
    }

    /// Declares a new variable in the current scope
    pub fn declare(&self, name: &str) {
        self.variables.borrow_mut().insert(name.to_owned());
    }

    /// Returns `true` if variable with a given `name` is declared in the current scope
    pub fn is_declared(&self, name: &str) -> bool {
        self.variables.borrow().get(name).is_some()
    }

    pub(crate) fn declare_forward_args(&self) {
        self.declare(FORWARD_ARGS);
    }

    pub(crate) fn is_forward_args_declared(&self) -> bool {
        self.is_declared(FORWARD_ARGS)
    }
}
