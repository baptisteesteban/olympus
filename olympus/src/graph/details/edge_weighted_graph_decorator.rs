use std::{fmt::Display, iter::zip, ops::Sub};

use crate::{
    drawing::{inferno, Normalizer},
    graph::DotFormat,
    BoundedValueSet, EdgeWeightedGraph, Graph, GraphBuildFromNumberOfNodes, MutableEdgeGraph,
    MutableEdgeWeightedGraph, MutableNodeGraph, ToMutableEdgeGraph, ToMutableEdgeWeightedGraph,
    ToMutableNodeGraph,
};

pub struct EdgeWeightedGraphDecorator<W, G: Graph> {
    g: G,
    w: Vec<W>,
}

impl<W, G: Graph> EdgeWeightedGraphDecorator<W, G> {
    pub fn new(g: G, w: Vec<W>) -> Result<EdgeWeightedGraphDecorator<W, G>, String> {
        if g.num_edges() != w.len() {
            return Err(format!(
                "Invalid number of edge weights (Got {}, expected {})",
                w.len(),
                g.num_edges()
            ));
        }
        Ok(EdgeWeightedGraphDecorator { g, w })
    }

    pub fn inner(&self) -> &G {
        &self.g
    }
}

impl<W, G> Default for EdgeWeightedGraphDecorator<W, G>
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

impl<W, G: Graph> Graph for EdgeWeightedGraphDecorator<W, G> {
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

impl<W, G> GraphBuildFromNumberOfNodes for EdgeWeightedGraphDecorator<W, G>
where
    G: Graph + GraphBuildFromNumberOfNodes,
{
    fn new_from_number_of_nodes(n: usize) -> Self {
        EdgeWeightedGraphDecorator {
            g: G::new_from_number_of_nodes(n),
            w: Default::default(),
        }
    }
}

impl<W, G> MutableNodeGraph for EdgeWeightedGraphDecorator<W, G>
where
    G: Graph + MutableNodeGraph,
{
    fn add_node(&mut self) -> i32 {
        self.g.add_node()
    }
}

impl<W, G> ToMutableNodeGraph for EdgeWeightedGraphDecorator<W, G>
where
    G: Graph + MutableNodeGraph,
{
    type Result = EdgeWeightedGraphDecorator<W, G>;
}

impl<W, G> MutableEdgeGraph for EdgeWeightedGraphDecorator<W, G>
where
    G: Graph + MutableEdgeGraph,
    W: Default,
{
    fn add_edge(&mut self, n1: i32, n2: i32) -> Result<(), String> {
        self.w.push(Default::default());
        self.g.add_edge(n1, n2)
    }
}

impl<W, G> ToMutableEdgeGraph for EdgeWeightedGraphDecorator<W, G>
where
    G: Graph + MutableEdgeGraph,
    W: Default,
{
    type Result = EdgeWeightedGraphDecorator<W, G>;
}

impl<W, G> EdgeWeightedGraph for EdgeWeightedGraphDecorator<W, G>
where
    G: Graph,
{
    type Weight = W;

    fn weight(&self, e: usize) -> Result<&Self::Weight, String> {
        if e >= self.g.num_edges() {
            Err(String::from("Invalid edge index"))
        } else {
            Ok(self.w.get(e).unwrap())
        }
    }

    fn weighted_edges(&self) -> impl Iterator<Item = (&Self::Weight, crate::graph::Edge)> {
        zip(self.w.iter(), self.edges())
    }
}

impl<W, G> MutableEdgeWeightedGraph for EdgeWeightedGraphDecorator<W, G>
where
    G: Graph + MutableEdgeGraph,
{
    fn add_edge(&mut self, n1: i32, n2: i32, w: Self::Weight) -> Result<(), String> {
        let e = self.g.add_edge(n1, n2);
        self.w.push(w);
        e
    }
}

impl<W, G> ToMutableEdgeWeightedGraph for EdgeWeightedGraphDecorator<W, G>
where
    G: Graph + MutableEdgeGraph,
{
    type Result = EdgeWeightedGraphDecorator<W, G>;
}

impl<W, G: Graph> DotFormat for EdgeWeightedGraphDecorator<W, G>
where
    W: Display + Ord + BoundedValueSet + Sub<Output = W> + Into<f64> + Clone,
{
    fn to_dot(&self) -> String {
        let normalize = Normalizer::new(self.w.clone().into_iter());
        let mut res = String::from("graph G {\n");
        for n in self.nodes() {
            res.push_str(format!("\t{} [label=\"{}\" shape=\"circle\"]\n", n, n).as_str());
        }
        for (w, e) in self.weighted_edges() {
            let color = inferno(normalize.apply(w));
            res.push_str(
                format!("\t{} -- {} [color=\"#{}\"]\n", e.n1(), e.n2(), color.hex()).as_str(),
            );
        }
        res.push_str("}\n");
        res
    }
}
