use std::option::Iter;

pub trait Node {
    fn id(&self) -> usize;
}

pub trait Edge {
    fn from(&self) -> dyn Node;
    fn to(&self) -> dyn Node;
}

pub trait WeightedEdge: Edge {
    fn weight(&self) -> usize;
}

pub trait Graph {
    fn node(&self, id: usize) -> Option<Box<dyn Node>>;
    fn nodes(&self) -> Iter<Box<dyn Node>>;
    // returns all nodes that can be reached directly from the node
    // with the given id
    fn from(&self, id: usize) -> Vec<Box<dyn Node>>;
    fn has_edge_between(&self, xid: usize, yid: usize) -> bool;
    fn edge(&self, uid: usize, vid: usize) -> Option<Box<dyn Edge>>;
}

pub trait Weighted: Graph {}

pub trait Undirected: Graph {}

pub trait WeightedUndirected: Graph {}

pub trait Directed: Graph {}

pub trait WeightedDirected: Graph {}

pub trait NodeAdder {
    fn new_node() -> dyn Node;
    fn add_node(node: dyn Node);
}

pub trait NodeRemover {
    fn remove_node(id: usize);
}

pub trait EdgeAdder {
    fn new_edge(from: dyn Node, to: dyn Node) -> dyn Edge;
    fn add_edge(edge: dyn Edge);
}

pub trait WeightedEdgeAdder {
    fn new_weighted_edge(from: dyn Node, to: dyn Node, weight: usize) -> dyn WeightedEdge;
    fn add_weighted_edge(edge: dyn WeightedEdge);
}

pub trait EdgeRemover {
    fn remove_edge(uid: usize, vid: usize);
}

pub trait Builder: NodeAdder + NodeRemover {}

pub trait WeightedBuilder: NodeAdder + WeightedEdgeAdder {}

pub trait UndirectedBuilder: Undirected + Builder {}

pub trait DirectedWeightedBuilder: Directed + WeightedBuilder {}
