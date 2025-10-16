use crate::graph::{AdjacencyList, EdgeWeightedGraphDecorator};

pub type AdjacencyListEdgeWeigthedGraph<W> = EdgeWeightedGraphDecorator<W, AdjacencyList>;
