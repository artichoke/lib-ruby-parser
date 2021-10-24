mod item;
pub use item::Item;

mod visit_gen;
pub use visit_gen::Observer;

use crate::Node;

crate::use_native_or_external!(Ptr);
crate::use_native_or_external!(Maybe);
crate::use_native_or_external!(Vec);

/// Generic visitor of `Node`.
///
/// It doesn't do anything on its own,
/// but it notifies given `Observer`.
///
/// ```text
/// struct MyObserver {
///     pub numbers: Vec<nodes::Int>,
/// }
///
/// impl Observer for MyObserver {
///     fn on_int(&mut self, node: &nodes::Int) {
///         self.numbers.push(node.clone())
///     }
/// }
///
/// let source = "2 + 3";
/// let mut parser = Parser::new(source.as_bytes(), ParserOptions::default());
/// let ast = parser.do_parse().ast.unwrap();
///
/// let observer = MyObserver { numbers: vec![] };
/// let visitor = Visitor { observer };
///
/// visitor.visit_root(&ast);
///
/// println!("{:?}", visitor.observer.numbers);
/// // => [Int { value: "2" }, Int { value: "3" }]
/// ```
#[derive(Debug)]
pub struct Visitor<T> {
    /// Observer of the visitor, receives calls like `on_int(&mut self, node: nodes::Int)`
    pub observer: T,
}

pub(crate) trait Visit<TItem> {
    fn visit(&mut self, item: TItem, visit_as: Item);
}

impl<'a, TObserver: Observer<'a>> Visit<&'a Vec<'a, &'a Node<'a>>> for Visitor<TObserver>
where
    Self: 'a,
{
    fn visit(&mut self, nodes: &'a Vec<'a, &'a Node<'a>>, visit_as: Item) {
        self.visit(nodes.as_slice(), visit_as);
    }
}

impl<'a, TObserver: Observer<'a>> Visit<&'a [&'a Node<'a>]> for Visitor<TObserver>
where
    Self: 'a,
{
    fn visit(&mut self, nodes: &'a [&'a Node<'a>], visit_as: Item) {
        self.observer.on_subitem(visit_as);
        self.observer.on_node_list(nodes);

        for (idx, node) in nodes.iter().enumerate() {
            let node = &**node;
            self.visit(node, Item::Idx(idx));
        }

        self.observer.on_subitem_moving_up(visit_as);
    }
}

// impl<'a, TObserver: Observer<'a>> Visit<&'a Vec<'a, &'a Node<'a>>> for Visitor<TObserver>
// where
//     Self: 'a,
// {
//     fn visit(&mut self, nodes: &'a Vec<'a, &'a Node<'a>>, visit_as: Item) {
//         let nodes: &'a [&'a Node<'a>] = nodes.as_slice();
//         self.visit(nodes, visit_as);
//     }
// }

impl<'a, TObserver: Observer<'a>> Visit<Option<&'a &'a Node<'a>>> for Visitor<TObserver>
where
    Self: 'a,
{
    fn visit(&mut self, node: Option<&'a &'a Node<'a>>, visit_as: Item) {
        if let Some(node) = node {
            self.visit(&**node, visit_as);
        }
    }
}

impl<'a, T> Visitor<T>
where
    T: Observer<'a>,
    Self: 'a,
{
    /// Starts traversing on a given `node`
    pub fn visit_root(&mut self, node: &'a Node<'a>) {
        self.visit(node, Item::Root);
    }
}
