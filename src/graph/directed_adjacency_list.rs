use crate::traits::{DirectedGraph, Graph};

use super::AdjacencyList;

pub struct DirectedAdjacencyList(AdjacencyList);

impl DirectedAdjacencyList {
    pub fn new() -> DirectedAdjacencyList {
        DirectedAdjacencyList(AdjacencyList::new())
    }
}

impl Graph for DirectedAdjacencyList {
    fn num_vertices(&self) -> i32 {
        self.0.num_vertices()
    }

    fn add_vertex(&mut self) -> i32 {
        self.0.add_vertex()
    }

    fn add_edge(&mut self, edge: (i32, i32)) -> Result<(), String> {
        self.0.add_edge(edge)
    }

    fn has_vertex(&self, v: i32) -> bool {
        self.0.has_vertex(v)
    }

    fn has_edge(&self, edge: (i32, i32)) -> bool {
        self.0.has_edge(edge)
    }
}

impl DirectedGraph for DirectedAdjacencyList {}
