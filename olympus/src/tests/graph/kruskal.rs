use crate::graph::{kruskal, AdjacencyListEdgeWeigthedGraph, Edge};
use crate::{EdgeWeightedGraph, Graph, MutableEdgeWeightedGraph, MutableNodeGraph};

#[test]
fn test_kruskal_adjacency_list() {
    let mut g = AdjacencyListEdgeWeigthedGraph::default();
    let n1 = g.add_node();
    let n2 = g.add_node();
    let n3 = g.add_node();
    g.add_edge(n1, n2, 3).unwrap();
    g.add_edge(n2, n3, 5).unwrap();
    g.add_edge(n3, n1, 2).unwrap();

    let mst = kruskal(&g);

    assert_eq!(mst.num_nodes(), 3);
    assert_eq!(mst.num_edges(), 2);
    let ref_edges = [(2u8, Edge::new(0, 2)), (3, Edge::new(0, 1))];
    for ((w, e), (w_ref, e_ref)) in std::iter::zip(mst.weighted_edges(), ref_edges) {
        assert_eq!(*w, w_ref);
        assert_eq!(e, e_ref);
    }
}
