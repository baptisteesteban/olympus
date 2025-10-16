use crate::graph::{AdjacencyList, NodeWeightedGraphDecorator};

pub type AdjacencyListNodeWeightedGraph<W> = NodeWeightedGraphDecorator<W, AdjacencyList>;
