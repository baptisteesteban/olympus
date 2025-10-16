use crate::{
    graph::Edge, EdgeWeightedGraph, GraphBuildFromNumberOfNodes, MutableEdgeWeightedGraph,
    MutableNodeGraph, ToMutableEdgeWeightedGraph, UnionFind,
};

pub trait KruskalVisitor {
    fn on_make_set(&mut self, n: i32);
    fn on_different_set(&mut self, a: i32, b: i32);
    fn on_same_set(&mut self, a: i32, b: i32);
}

pub fn kruskal_impl<
    G: ToMutableEdgeWeightedGraph<Result = impl MutableEdgeWeightedGraph<Weight = G::Weight>>
        + EdgeWeightedGraph,
>(
    g: &G,
    viz: &mut impl KruskalVisitor,
) -> <G as ToMutableEdgeWeightedGraph>::Result
where
    <G as ToMutableEdgeWeightedGraph>::Result:
        GraphBuildFromNumberOfNodes + MutableNodeGraph + MutableEdgeWeightedGraph,
    <G as EdgeWeightedGraph>::Weight: Copy + Ord,
{
    let mut res = <<G as ToMutableEdgeWeightedGraph>::Result as GraphBuildFromNumberOfNodes>::new_from_number_of_nodes(g.num_nodes());
    let mut uf = UnionFind::<Vec<usize>>::new(vec![0; g.num_nodes()]);
    for n in g.nodes() {
        uf.make_set(&(n as usize));
        viz.on_make_set(n);
    }

    let mut sorted_nodes: Vec<(&<G as EdgeWeightedGraph>::Weight, Edge)> =
        g.weighted_edges().collect();
    sorted_nodes.sort_by_key(|(w, _)| **w);
    for (w, e) in sorted_nodes {
        let ra = uf.find(&(e.n1() as usize));
        let rb = uf.find(&(e.n2() as usize));
        if ra != rb {
            res.add_edge(e.n1(), e.n2(), *w).unwrap();
            uf.union(&ra, &rb);
            viz.on_same_set(ra as i32, rb as i32);
        } else {
            viz.on_different_set(ra as i32, rb as i32);
        }
    }

    res
}

/*
* Default implementation of the Kruskal algorithm
*/

struct DefaultKruskalVisitor {}
impl KruskalVisitor for DefaultKruskalVisitor {
    fn on_make_set(&mut self, _n: i32) {}
    fn on_different_set(&mut self, _a: i32, _b: i32) {}
    fn on_same_set(&mut self, _a: i32, _b: i32) {}
}

pub fn kruskal<
    G: ToMutableEdgeWeightedGraph<Result = impl MutableEdgeWeightedGraph<Weight = G::Weight>>
        + EdgeWeightedGraph,
>(
    g: &G,
) -> <G as ToMutableEdgeWeightedGraph>::Result
where
    <G as ToMutableEdgeWeightedGraph>::Result:
        GraphBuildFromNumberOfNodes + MutableNodeGraph + MutableEdgeWeightedGraph,
    <G as EdgeWeightedGraph>::Weight: Copy + Ord,
{
    kruskal_impl(g, &mut DefaultKruskalVisitor {})
}
