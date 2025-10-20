use crate::graph::Edge;

/// Trait implementation to represent a graph (from now on undirected).
pub trait Graph {
    /// Type alias for the iterator iterating over the graph nodes.
    type NodeIterator<'a>: Iterator<Item = i32>
    where
        Self: 'a;
    /// Type alias for the iterator iterating over the graph edges.
    type EdgeIterator<'a>: Iterator<Item = Edge>
    where
        Self: 'a;
    /// Type alias for the iterator iterating over the adjacent nodes of a node.
    type AdjacentNodeIterator<'a>: Iterator<Item = i32>
    where
        Self: 'a;

    /// Returns an iterator over all the nodes of a graph.
    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a>;
    /// Returns the number of nodes of a graph.
    fn num_nodes(&self) -> usize;

    /// Returns an iterator over all the edges of a graph.
    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a>;
    /// Returns the number of edges of a graph.
    fn num_edges(&self) -> usize;

    /// Returns the degree of a node of a graph.
    fn degree(&self, n: i32) -> Result<usize, String>;
    /// Returns an iterator iterating over the adjacent nodes of a node of a graph.
    fn adjacent_nodes<'a>(&'a self, n: i32) -> Result<Self::AdjacentNodeIterator<'a>, String>;
}

/// Traits to build a graph from a fixed number of nodes.
pub trait GraphBuildFromNumberOfNodes: Graph {
    /// Build a graph containing `n` nodes.
    fn new_from_number_of_nodes(n: usize) -> Self;
}

/// Trait for a graph whose number of nodes may be modified.
pub trait MutableNodeGraph: Graph {
    /// Add a new node to the graph and returns its index.
    fn add_node(&mut self) -> i32;
}

/// Trait to obtain a `MutableNodeGraph` from a potentially non `MutableNodeGraph`.
pub trait ToMutableNodeGraph: Graph {
    /// The resulting `MutableNodeGraph`.
    type Result: MutableNodeGraph;
}

/// Trait for a graph whose number of edges may be modified.
pub trait MutableEdgeGraph: Graph {
    /// Add an edge between two nodes `n1` and `n2`
    ///
    /// # Errors
    ///
    /// An error may be implemented in case of one of the nodes does not belong
    /// to the graph node set.
    fn add_edge(&mut self, n1: i32, n2: i32) -> Result<(), String>;
}

/// Trait to obtain a `MutableEdgeGraph` from a potentially non `MutableEdgeGraph`.
pub trait ToMutableEdgeGraph: Graph {
    /// The resulting `MutableEdgeGraph`.
    type Result: MutableEdgeGraph;
}

/// Trait to represent an edge weighted graph.
pub trait EdgeWeightedGraph: Graph {
    /// Weight type of the edges
    type Weight;
    /// Return the weight of an edge.
    ///
    /// # Errors
    ///
    /// May returns an error if the edge does not belong to the graph edge.
    fn weight(&self, e: usize) -> Result<&Self::Weight, String>;
    /// Returns an iterator which iterates over the edges and contains its
    /// weight and value.
    fn weighted_edges(&self) -> impl Iterator<Item = (&Self::Weight, Edge)>;
}

/// Trait to represent a mutable edge weighted graph.
pub trait MutableEdgeWeightedGraph: EdgeWeightedGraph {
    /// Add an edge between the nodes `n1` and `n2` with a weight `w`.
    fn add_edge(&mut self, n1: i32, n2: i32, w: Self::Weight) -> Result<(), String>;
}

/// Trait to returns the type of a `MutableEdgeWeightedGraph` from a graph that is potentially not mutable.
pub trait ToMutableEdgeWeightedGraph: EdgeWeightedGraph {
    /// The type alias to the resulting `MutableEdgeWeightedGraph`.
    type Result: MutableEdgeWeightedGraph;
}

/// Trait that represents a node weighted graph.
pub trait NodeWeightedGraph: Graph {
    /// Type of the node weights.
    type Weight;

    /// Returns the weight of a node `n`.
    ///
    /// # Errors
    ///
    /// It may returns an error if the node `n` does not belong to the set of graph nodes.
    fn weight(&self, n: i32) -> Result<&Self::Weight, String>;
    /// Returns an iterator which iterates over the nodes of the graph and their weight.
    fn weighted_nodes(&self) -> impl Iterator<Item = (&Self::Weight, i32)>;
}

/// Trait that represents a node weighted graph.
pub trait MutableNodeWeightedGraph: NodeWeightedGraph {
    /// Adds a node with its weight and returns the new node index.
    fn add_node(&mut self, w: Self::Weight) -> i32;
}

/// Trait to convert a (potentially not mutable) node weighted graph to a `MutableNodeWeightedGraph`.
pub trait ToMutableNodeWeightedGraph {
    /// Resulting type of conversion
    type Result: MutableNodeWeightedGraph;
}
