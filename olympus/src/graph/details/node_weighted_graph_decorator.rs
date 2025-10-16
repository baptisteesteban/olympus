use std::iter::zip;

use crate::{
    graph::DotFormat, Graph, GraphBuildFromNumberOfNodes, MutableEdgeGraph, MutableNodeGraph,
    MutableNodeWeightedGraph, NodeWeightedGraph, Rgb8, ToMutableEdgeGraph, ToMutableNodeGraph,
    ToMutableNodeWeightedGraph,
};

pub struct NodeWeightedGraphDecorator<W, G: Graph> {
    g: G,
    w: Vec<W>,
}

impl<W, G: Graph> NodeWeightedGraphDecorator<W, G> {
    pub fn new(g: G, w: Vec<W>) -> Result<NodeWeightedGraphDecorator<W, G>, String> {
        if g.num_nodes() != w.len() {
            return Err(format!(
                "Invalid number of edge weights (Got {}, expected {})",
                w.len(),
                g.num_edges()
            ));
        }
        Ok(NodeWeightedGraphDecorator { g, w })
    }

    pub fn inner(&self) -> &G {
        &self.g
    }
}

impl<W, G> Default for NodeWeightedGraphDecorator<W, G>
where
    G: Graph + Default,
{
    fn default() -> Self {
        Self {
            g: Default::default(),
            w: Default::default(),
        }
    }
}

impl<W, G: Graph> Graph for NodeWeightedGraphDecorator<W, G> {
    type NodeIterator<'a>
        = G::NodeIterator<'a>
    where
        Self: 'a;

    type EdgeIterator<'a>
        = G::EdgeIterator<'a>
    where
        Self: 'a;

    type AdjacentNodeIterator<'a>
        = G::AdjacentNodeIterator<'a>
    where
        Self: 'a;

    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a> {
        self.g.nodes()
    }

    fn num_nodes(&self) -> usize {
        self.g.num_nodes()
    }

    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a> {
        self.g.edges()
    }

    fn num_edges(&self) -> usize {
        self.g.num_edges()
    }

    fn degree(&self, n: i32) -> Result<usize, String> {
        self.g.degree(n)
    }

    fn adjacent_nodes<'a>(&'a self, n: i32) -> Result<Self::AdjacentNodeIterator<'a>, String> {
        self.g.adjacent_nodes(n)
    }
}

impl<W, G> GraphBuildFromNumberOfNodes for NodeWeightedGraphDecorator<W, G>
where
    G: Graph + GraphBuildFromNumberOfNodes,
    W: Default + Clone,
{
    fn new_from_number_of_nodes(n: usize) -> Self {
        NodeWeightedGraphDecorator {
            g: G::new_from_number_of_nodes(n),
            w: vec![Default::default(); n],
        }
    }
}

impl<W, G> MutableNodeGraph for NodeWeightedGraphDecorator<W, G>
where
    G: Graph + MutableNodeGraph,
    W: Default,
{
    fn add_node(&mut self) -> i32 {
        self.w.push(Default::default());
        self.g.add_node()
    }
}

impl<W, G> ToMutableNodeGraph for NodeWeightedGraphDecorator<W, G>
where
    G: Graph + MutableNodeGraph,
    W: Default,
{
    type Result = NodeWeightedGraphDecorator<W, G>;
}

impl<W, G> MutableEdgeGraph for NodeWeightedGraphDecorator<W, G>
where
    G: Graph + MutableEdgeGraph,
{
    fn add_edge(&mut self, n1: i32, n2: i32) -> Result<(), String> {
        self.g.add_edge(n1, n2)
    }
}

impl<W, G> ToMutableEdgeGraph for NodeWeightedGraphDecorator<W, G>
where
    G: Graph + MutableEdgeGraph,
{
    type Result = NodeWeightedGraphDecorator<W, G>;
}

impl<W, G: Graph> NodeWeightedGraph for NodeWeightedGraphDecorator<W, G> {
    type Weight = W;

    fn weight(&self, n: i32) -> Result<&Self::Weight, String> {
        if n >= self.num_nodes() as i32 {
            return Err(format!("Node {} not in the graph", n));
        }
        Ok(self.w.get(n as usize).unwrap())
    }

    fn weighted_nodes(&self) -> impl Iterator<Item = (&Self::Weight, i32)> {
        zip(self.w.iter(), self.nodes())
    }
}

impl<W, G> MutableNodeWeightedGraph for NodeWeightedGraphDecorator<W, G>
where
    G: Graph + MutableNodeGraph,
{
    fn add_node(&mut self, w: Self::Weight) -> i32 {
        self.w.push(w);
        self.g.add_node()
    }
}

impl<W, G> ToMutableNodeWeightedGraph for NodeWeightedGraphDecorator<W, G>
where
    G: Graph + MutableNodeGraph,
{
    type Result = NodeWeightedGraphDecorator<W, G>;
}

impl<G: Graph> DotFormat for NodeWeightedGraphDecorator<u8, G> {
    fn to_dot(&self) -> String {
        let mut res = String::from("graph G {\n");

        for (w, n) in self.weighted_nodes() {
            res.push_str(
                format!(
                    "\t{} [style=\"filled\" shape=\"circle\" fillcolor=\"#{}\"",
                    n,
                    Rgb8::new(*w, *w, *w).hex()
                )
                .as_str(),
            );
            if *w < 70 {
                res.push_str(" fontcolor=\"#ffffff\"");
            }
            res.push_str("]\n");
        }

        for e in self.edges() {
            res.push_str(format!("\t{} -- {}\n", e.n1(), e.n2()).as_str());
        }

        res.push_str("}\n");
        res
    }
}

impl<G: Graph> DotFormat for NodeWeightedGraphDecorator<Rgb8, G> {
    fn to_dot(&self) -> String {
        let mut res = String::from("graph G {\n");

        for (w, n) in self.weighted_nodes() {
            res.push_str(
                format!(
                    "\t{} [style=\"filled\" shape=\"circle\" fillcolor=\"#{}\"]\n",
                    n,
                    w.hex()
                )
                .as_str(),
            );
        }

        for e in self.edges() {
            res.push_str(format!("\t{} -- {}\n", e.n1(), e.n2()).as_str());
        }

        res.push_str("}\n");
        res
    }
}
