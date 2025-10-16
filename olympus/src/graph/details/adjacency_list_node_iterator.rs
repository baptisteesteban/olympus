pub struct AdjacencyListNodeIterator {
    n: i32,
    cur: i32,
}

impl AdjacencyListNodeIterator {
    pub fn new(n: i32) -> AdjacencyListNodeIterator {
        AdjacencyListNodeIterator { n, cur: 0 }
    }
}

impl Iterator for AdjacencyListNodeIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.n {
            let res = self.cur;
            self.cur += 1;
            Some(res)
        } else {
            None
        }
    }
}
