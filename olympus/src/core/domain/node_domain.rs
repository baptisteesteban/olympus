use crate::{graph::AdjacencyList, Domain, Graph, SizedDomain};

#[derive(Clone, Debug)]
pub struct NodeDomain {
    g: AdjacencyList,
}

impl NodeDomain {
    /// Create a `NodeDomain` from an adjacency list
    pub fn new(g: AdjacencyList) -> NodeDomain {
        NodeDomain { g }
    }

    pub fn inner(&self) -> &AdjacencyList {
        &self.g
    }
}

impl Domain for NodeDomain {
    type Point = i32;

    fn has(&self, p: &Self::Point) -> bool {
        *p >= 0 && *p < self.g.num_nodes() as i32
    }
}

impl SizedDomain for NodeDomain {
    fn size(&self) -> usize {
        self.g.num_nodes()
    }
}

impl IntoIterator for NodeDomain {
    type Item = i32;
    type IntoIter = <AdjacencyList as Graph>::NodeIterator<'static>;

    fn into_iter(self) -> Self::IntoIter {
        self.g.nodes()
    }
}
