use crate::graph::{AdjacencyList, Edge};
use crate::{Graph, MutableEdgeGraph, MutableNodeGraph};

#[test]
fn test_adjacency_list() {
    let mut g = AdjacencyList::default();
    let n1 = g.add_node();
    let n2 = g.add_node();
    let n3 = g.add_node();

    g.add_edge(n1, n2).unwrap();
    g.add_edge(n2, n3).unwrap();
    g.add_edge(n3, n1).unwrap();

    let err = g.add_edge(n1, 5);
    assert!(err.is_err());
    assert_eq!(g.num_nodes(), 3);
    assert_eq!(g.num_edges(), 3);

    let mut i = 0;
    for n in g.nodes() {
        assert_eq!(n, i);
        assert_eq!(g.degree(n).unwrap(), 2);
        i += 1;
    }
    assert_eq!(i, 3);

    let ref_edges = [Edge::new(n1, n2), Edge::new(n2, n3), Edge::new(n1, n3)];
    for (e, e_ref) in std::iter::zip(ref_edges, g.edges()) {
        assert_eq!(e, e_ref);
    }

    let ref_adjacent_nodes = [n1, n3];
    for (n, n_ref) in std::iter::zip(g.adjacent_nodes(n2).unwrap(), ref_adjacent_nodes) {
        assert_eq!(n, n_ref);
    }
}
