use crate::graph::Edge;

pub trait Graph {
    type NodeIterator<'a>: Iterator<Item = i32>
    where
        Self: 'a;
    type EdgeIterator<'a>: Iterator<Item = Edge>
    where
        Self: 'a;
    type AdjacentNodeIterator<'a>: Iterator<Item = i32>
    where
        Self: 'a;

    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a>;
    fn num_nodes(&self) -> usize;

    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a>;
    fn num_edges(&self) -> usize;

    fn degree(&self, n: i32) -> Result<usize, String>;
    fn adjacent_nodes<'a>(&'a self, n: i32) -> Result<Self::AdjacentNodeIterator<'a>, String>;
}

pub trait GraphBuildFromNumberOfNodes: Graph {
    fn new_from_number_of_nodes(n: usize) -> Self;
}

pub trait MutableNodeGraph: Graph {
    fn add_node(&mut self) -> i32;
}

pub trait ToMutableNodeGraph: Graph {
    type Result: MutableNodeGraph;
}

pub trait MutableEdgeGraph: Graph {
    fn add_edge(&mut self, n1: i32, n2: i32) -> Result<(), String>;
}

pub trait ToMutableEdgeGraph: Graph {
    type Result: MutableEdgeGraph;
}

pub trait EdgeWeightedGraph: Graph {
    type Weight;
    fn weight(&self, e: usize) -> Result<&Self::Weight, String>;
    fn weighted_edges(&self) -> impl Iterator<Item = (&Self::Weight, Edge)>;
}

pub trait MutableEdgeWeightedGraph: EdgeWeightedGraph {
    fn add_edge(&mut self, n1: i32, n2: i32, w: Self::Weight) -> Result<(), String>;
}

pub trait ToMutableEdgeWeightedGraph: EdgeWeightedGraph {
    type Result: MutableEdgeWeightedGraph;
}

pub trait NodeWeightedGraph: Graph {
    type Weight;
    fn weight(&self, n: i32) -> Result<&Self::Weight, String>;
    fn weighted_nodes(&self) -> impl Iterator<Item = (&Self::Weight, i32)>;
}

pub trait MutableNodeWeightedGraph: NodeWeightedGraph {
    fn add_node(&mut self, w: Self::Weight) -> i32;
}

pub trait ToMutableNodeWeightedGraph {
    type Result: MutableNodeWeightedGraph;
}
