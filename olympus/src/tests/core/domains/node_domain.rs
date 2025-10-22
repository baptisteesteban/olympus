use crate::{
    graph::AdjacencyList, Domain, MutableEdgeGraph, MutableNodeGraph, NodeDomain, SizedDomain,
};

#[test]
fn test_node_domain() {
    // Build an adjacency list graph
    let mut g = AdjacencyList::default();
    let n1 = g.add_node();
    let n2 = g.add_node();
    let n3 = g.add_node();
    g.add_edge(n1, n2).unwrap();
    g.add_edge(n2, n3).unwrap();
    g.add_edge(n1, n3).unwrap();

    // Build the node domain
    let domain = NodeDomain::new(g);
    assert_eq!(domain.size(), 3);
    assert!(domain.has(&n1));
    assert!(domain.has(&n2));
    assert!(domain.has(&n3));
    assert!(!domain.has(&4));

    for (i, n) in domain.clone().into_iter().enumerate() {
        assert_eq!(i as i32, n);
    }
    assert!(domain.has(&n1));
}
