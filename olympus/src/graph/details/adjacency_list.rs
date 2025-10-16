use crate::{
    graph::{
        details::adjacency_list_edge_iterator::AdjacencyListEdgeIterator,
        AdjacencyListAdjacencyNodeIterator, AdjacencyListNodeIterator, DotFormat, Edge,
    },
    Graph, GraphBuildFromNumberOfNodes, MutableEdgeGraph, MutableNodeGraph, ToMutableEdgeGraph,
    ToMutableNodeGraph,
};

use std::cmp::{max, min};

#[derive(Default)]
pub struct AdjacencyList {
    edges: Vec<Edge>,
    adjacency: Vec<Vec<i32>>,
}

impl Graph for AdjacencyList {
    type NodeIterator<'a> = AdjacencyListNodeIterator;
    type EdgeIterator<'a> = AdjacencyListEdgeIterator<'a>;
    type AdjacentNodeIterator<'a> = AdjacencyListAdjacencyNodeIterator<'a>;

    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a> {
        AdjacencyListNodeIterator::new(self.num_nodes() as i32)
    }

    fn num_nodes(&self) -> usize {
        self.adjacency.len()
    }

    fn edges(&self) -> Self::EdgeIterator<'_> {
        AdjacencyListEdgeIterator::new(&self.edges)
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn degree(&self, n: i32) -> Result<usize, String> {
        if n < self.num_nodes() as i32 {
            Ok(self.adjacency.get(n as usize).unwrap().len())
        } else {
            Err(format!("Node {} not in the graph", n))
        }
    }

    fn adjacent_nodes<'a>(&'a self, n: i32) -> Result<Self::AdjacentNodeIterator<'a>, String> {
        if n < self.num_nodes() as i32 {
            Ok(AdjacencyListAdjacencyNodeIterator::new(
                &self.edges,
                &self.adjacency[n as usize],
                n,
            ))
        } else {
            Err(format!("Node {} not in the graph", n))
        }
    }
}

impl GraphBuildFromNumberOfNodes for AdjacencyList {
    fn new_from_number_of_nodes(n: usize) -> Self {
        AdjacencyList {
            edges: Default::default(),
            adjacency: vec![Vec::<i32>::new(); n],
        }
    }
}

impl MutableNodeGraph for AdjacencyList {
    fn add_node(&mut self) -> i32 {
        let n = self.adjacency.len();
        self.adjacency.push(Default::default());
        n as i32
    }
}

impl ToMutableNodeGraph for AdjacencyList {
    type Result = AdjacencyList;
}

impl MutableEdgeGraph for AdjacencyList {
    fn add_edge(&mut self, n1: i32, n2: i32) -> Result<(), String> {
        if n1 >= self.adjacency.len() as i32 {
            return Err(format!("Node {} not in the graph", n1));
        }
        if n2 >= self.adjacency.len() as i32 {
            return Err(format!("Node {} not in the graph", n2));
        }
        let e = Edge::new(min(n1, n2), max(n1, n2));
        let ei = self.edges.len() as i32;
        self.edges.push(e);
        self.adjacency.get_mut(n1 as usize).unwrap().push(ei);
        self.adjacency.get_mut(n2 as usize).unwrap().push(ei);
        Ok(())
    }
}

impl ToMutableEdgeGraph for AdjacencyList {
    type Result = AdjacencyList;
}

impl DotFormat for AdjacencyList {
    fn to_dot(&self) -> String {
        let mut res = String::from("graph G {\n");
        for n in self.nodes() {
            res.push_str(format!("\t{} [label=\"{}\" shape=\"circle\"]\n", n, n).as_str());
        }
        for e in self.edges() {
            res.push_str(format!("\t{} -- {}\n", e.n1(), e.n2()).as_str());
        }
        res.push_str("}\n");
        res
    }
}
