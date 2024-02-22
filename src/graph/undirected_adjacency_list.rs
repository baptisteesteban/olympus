use crate::traits::{Graph, UndirectedGraph};

use super::AdjacencyList;

pub struct UndirectedAdjacencyList(AdjacencyList);

impl UndirectedAdjacencyList {
    pub fn new() -> UndirectedAdjacencyList {
        UndirectedAdjacencyList(AdjacencyList::new())
    }
}

impl Graph for UndirectedAdjacencyList {
    fn num_vertices(&self) -> i32 {
        self.0.num_vertices()
    }

    fn add_vertex(&mut self) -> i32 {
        self.0.add_vertex()
    }

    fn add_edge(&mut self, edge: (i32, i32)) -> Result<(), String> {
        let (mut u, mut v) = edge;
        if u > v {
            (u, v) = (v, u);
        }
        self.0.add_edge((u, v))
    }

    fn has_vertex(&self, v: i32) -> bool {
        self.0.has_vertex(v)
    }

    fn has_edge(&self, edge: (i32, i32)) -> bool {
        let (mut u, mut v) = edge;
        if u > v {
            (u, v) = (v, u);
        }
        self.0.has_edge((u, v))
    }
}

impl UndirectedGraph for UndirectedAdjacencyList {}
