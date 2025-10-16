use olympus::graph::{dot_format, kruskal, AdjacencyListEdgeWeigthedGraph};
use olympus::{MutableEdgeWeightedGraph, MutableNodeGraph};

fn main() {
    let mut g = AdjacencyListEdgeWeigthedGraph::default();
    let n1 = g.add_node();
    let n2 = g.add_node();
    let n3 = g.add_node();
    g.add_edge(n1, n2, 3).unwrap();
    g.add_edge(n2, n3, 5).unwrap();
    g.add_edge(n3, n1, 2).unwrap();

    let mst = kruskal(&g);

    println!("{}", dot_format(&mst));
}
