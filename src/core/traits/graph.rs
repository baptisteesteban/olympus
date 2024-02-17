pub trait Graph {
    fn num_vertices(&self) -> i32;
    fn add_vertex(&mut self) -> i32;
    fn add_edge(&mut self, edge: (i32, i32)) -> Result<(), String>;
    fn has_vertex(&self, v: i32) -> bool;
    fn has_edge(&self, edge: (i32, i32)) -> bool;
}

pub trait DirectedGraph: Graph {}
pub trait UndirectedGraph: Graph {}
