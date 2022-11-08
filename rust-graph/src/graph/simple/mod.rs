pub mod undirected;
/// Simple Implementation of the graph traits for each
/// type of graph
use crate::graph::traits::Edge;
use crate::graph::traits::Node;
use crate::graph::traits::WeightedEdge;

#[derive(Debug, PartialEq, Clone)]
struct SimpleNode {
    value: usize,
}

impl Node for SimpleNode {
    fn id(&self) -> usize {
        self.value
    }
}

impl Eq for SimpleNode {}

impl std::hash::Hash for SimpleNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.value);
        state.finish();
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SimpleEdge {
    f: SimpleNode,
    t: SimpleNode,
}

impl Edge<SimpleNode> for SimpleEdge {
    fn from(&self) -> &SimpleNode {
        &self.f
    }

    fn to(&self) -> &SimpleNode {
        &self.t
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SimpleWeightedEdge {
    f: SimpleNode,
    t: SimpleNode,
    w: f64,
}

impl Edge<SimpleNode> for SimpleWeightedEdge {
    fn from(&self) -> &SimpleNode {
        &self.f
    }

    fn to(&self) -> &SimpleNode {
        &self.t
    }
}

impl WeightedEdge<SimpleNode> for SimpleWeightedEdge {
    fn weight(&self) -> f64 {
        self.w
    }
}

pub struct SimpleGraph {}
