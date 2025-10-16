use crate::graph::Edge;

pub struct AdjacencyListAdjacencyNodeIterator<'a> {
    adjacent_edges: &'a Vec<i32>,
    edges: &'a Vec<Edge>,
    cur: usize,
    n: i32,
}

impl<'a> AdjacencyListAdjacencyNodeIterator<'a> {
    pub fn new(
        edges: &'a Vec<Edge>,
        adjacenct_edges: &'a Vec<i32>,
        n: i32,
    ) -> AdjacencyListAdjacencyNodeIterator<'a> {
        AdjacencyListAdjacencyNodeIterator {
            edges,
            adjacent_edges: adjacenct_edges,
            cur: 0,
            n,
        }
    }
}

impl<'a> Iterator for AdjacencyListAdjacencyNodeIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.adjacent_edges.len() {
            let e = self
                .edges
                .get(*self.adjacent_edges.get(self.cur).unwrap() as usize)
                .unwrap();
            self.cur += 1;
            Some(if e.n1() == self.n { e.n2() } else { e.n1() })
        } else {
            None
        }
    }
}
