use crate::{
    graph::AdjacencyList, MutableEdgeGraph, MutableNodeGraph, NodeDomain, NodeToNode, Window,
};

#[test]
fn test_node_to_node() {
    let mut g = AdjacencyList::default();
    let n1 = g.add_node();
    let n2 = g.add_node();
    let n3 = g.add_node();
    g.add_edge(n1, n2).unwrap();
    g.add_edge(n2, n3).unwrap();
    let domain = NodeDomain::new(g);
    let nbh = NodeToNode::new(&domain);

    for n in nbh.apply(&n1) {
        assert_eq!(n, n2);
    }
    for n in nbh.apply(&n3) {
        assert_eq!(n, n2);
    }
    let ref_nbh: [i32; 2] = [n1, n3];
    for (i, n) in nbh.apply(&n2).enumerate() {
        assert_eq!(n, ref_nbh[i])
    }
}
