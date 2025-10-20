use std::fmt::Display;

/// Represent an edge between two nodes `n1` and `n2`.
#[derive(Clone, Copy, Debug)]
pub struct Edge {
    n1: i32,
    n2: i32,
}

impl Edge {
    /// Create a new edge between two nodes `n1` and `n2`.
    pub fn new(n1: i32, n2: i32) -> Self {
        Edge { n1, n2 }
    }

    /// Returns the first edge node `n1`.
    pub fn n1(&self) -> i32 {
        self.n1
    }

    /// Returns the second edge node `n2`.
    pub fn n2(&self) -> i32 {
        self.n2
    }
}

impl Default for Edge {
    /// Default constructor for an edge. It is linked between two nodes that are not in a graph.
    fn default() -> Self {
        Self { n1: -1, n2: -1 }
    }
}

/// Implement the equality between two edges.
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.n1 == other.n1 && self.n2 == other.n2
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -- {})", self.n1, self.n2)
    }
}
