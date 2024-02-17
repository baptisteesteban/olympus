use crate::traits::Graph;

pub struct AdjacencyList {
    adjacency: Vec<Vec<i32>>,
}

impl AdjacencyList {
    pub fn new() -> AdjacencyList {
        AdjacencyList {
            adjacency: Vec::<Vec<i32>>::new(),
        }
    }
}

impl Graph for AdjacencyList {
    fn num_vertices(&self) -> i32 {
        self.adjacency.len() as i32
    }

    fn add_vertex(&mut self) -> i32 {
        let n = self.adjacency.len() as i32;
        self.adjacency.push(Vec::<i32>::new());
        n
    }

    fn add_edge(&mut self, (u, v): (i32, i32)) -> Result<(), String> {
        let n = self.adjacency.len() as i32;
        if u >= n {
            Err(format!("Vertex {} not in the graph", u))
        } else if v >= n {
            Err(format!("Vertex {} not in the graph", v))
        } else {
            self.adjacency.get_mut(u as usize).unwrap().push(v);
            Ok(())
        }
    }

    fn has_edge(&self, (u, v): (i32, i32)) -> bool {
        if !self.has_vertex(u) || !self.has_vertex(v) {
            false
        } else {
            let adj_u: &Vec<i32> = self.adjacency.get(u as usize).unwrap();
            match adj_u.iter().find(|obj| v == **obj) {
                Some(_) => true,
                None => false,
            }
        }
    }

    fn has_vertex(&self, v: i32) -> bool {
        v < self.adjacency.len() as i32
    }
}
