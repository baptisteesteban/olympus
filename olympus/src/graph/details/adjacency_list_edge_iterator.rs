use crate::graph::Edge;

pub struct AdjacencyListEdgeIterator<'a> {
    cur: usize,
    edges: &'a Vec<Edge>,
}

impl<'a> AdjacencyListEdgeIterator<'a> {
    pub fn new(edges: &'a Vec<Edge>) -> AdjacencyListEdgeIterator<'a> {
        AdjacencyListEdgeIterator { cur: 0, edges }
    }
}

impl<'a> Iterator for AdjacencyListEdgeIterator<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.edges.len() {
            let e = self.edges.get(self.cur).unwrap();
            self.cur += 1;
            Some(*e)
        } else {
            None
        }
    }
}
