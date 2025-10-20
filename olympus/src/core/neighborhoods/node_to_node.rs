use crate::{Domain, Graph, NodeDomain, Window};

/// This window returns the adjacent nodes of a node from a node domain.
pub struct NodeToNode<'a> {
    domain: &'a NodeDomain,
}

impl<'a> NodeToNode<'a> {
    pub fn new(domain: &'a NodeDomain) -> NodeToNode<'a> {
        NodeToNode { domain }
    }
}

impl<'a> Window for NodeToNode<'a> {
    type Point = i32;

    fn apply(&self, p: &Self::Point) -> impl Iterator<Item = Self::Point> {
        debug_assert!(self.domain.has(p));
        self.domain.inner().adjacent_nodes(*p).unwrap()
    }
}
