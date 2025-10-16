use olympus::{
    graph::{dot_format, AdjacencyListNodeWeightedGraph},
    MutableEdgeGraph, MutableNodeWeightedGraph, Rgb8,
};

fn main() {
    let mut g = AdjacencyListNodeWeightedGraph::<u8>::default();
    let n1 = g.add_node(0);
    let n2 = g.add_node(255);
    let n3 = g.add_node(50);
    g.add_edge(n1, n2).unwrap();
    g.add_edge(n2, n3).unwrap();
    g.add_edge(n3, n1).unwrap();

    let mut gc = AdjacencyListNodeWeightedGraph::<Rgb8>::default();
    let nc1 = gc.add_node(Rgb8::new(255, 0, 0));
    let nc2 = gc.add_node(Rgb8::new(0, 0, 255));
    let nc3 = gc.add_node(Rgb8::default());
    gc.add_edge(nc1, nc2).unwrap();
    gc.add_edge(nc2, nc3).unwrap();
    gc.add_edge(nc3, nc1).unwrap();

    println!("{}", dot_format(&gc));
}
