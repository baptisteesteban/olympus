use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    n1: i32,
    n2: i32,
}

impl Edge {
    pub fn new(n1: i32, n2: i32) -> Self {
        Edge { n1, n2 }
    }

    pub fn n1(&self) -> i32 {
        self.n1
    }

    pub fn n2(&self) -> i32 {
        self.n2
    }
}

impl Default for Edge {
    fn default() -> Self {
        Self { n1: -1, n2: -1 }
    }
}

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
