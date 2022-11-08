use std::fmt::Debug;
use std::option::Iter;

pub trait Node: Debug + Clone + Sized {
    fn id(&self) -> usize;
}

pub trait Edge<T>: Debug + Clone + Sized
where
    T: Node,
{
    fn from(&self) -> &T;
    fn to(&self) -> &T;
}

pub trait WeightedEdge<T>: Edge<T>
where
    T: Node,
{
    fn weight(&self) -> f64;
}

pub trait Graph<N, E>
where
    N: Node,
    E: Edge<N>,
{
    fn node(&self, id: usize) -> Option<&N>;
    fn nodes(&self) -> Iter<&N>;
    // returns all nodes that can be reached directly from the node
    // with the given id
    fn from(&self, id: usize) -> Vec<&N>;
    fn has_edge_between(&self, xid: usize, yid: usize) -> bool;
    fn edge(&self, uid: usize, vid: usize) -> Option<&E>;
}

pub trait Weighted<N, E>: Graph<N, E>
where
    N: Node,
    E: Edge<N>,
{
}

pub trait Undirected<N, E>: Graph<N, E>
where
    N: Node,
    E: Edge<N>,
{
}

pub trait WeightedUndirected<N, E>: Graph<N, E>
where
    N: Node,
    E: Edge<N>,
{
}

pub trait Directed<N, E>: Graph<N, E>
where
    N: Node,
    E: Edge<N>,
{
}

pub trait WeightedDirected<N, E>: Graph<N, E>
where
    N: Node,
    E: Edge<N>,
{
}

pub trait NodeAdder<N>
where
    N: Node,
{
    fn new_node() -> N;
    fn add_node(&mut self, node: N);
}

pub trait NodeRemover {
    fn remove_node(id: usize);
}

pub trait EdgeAdder<N, E>
where
    N: Node,
    E: Edge<N>,
{
    fn new_edge(from: N, to: N) -> E;
    fn add_edge(&mut self, edge: E);
}

pub trait WeightedEdgeAdder<N, E>: EdgeAdder<N, E>
where
    N: Node,
    E: WeightedEdge<N>,
{
    fn new_weighted_edge(from: N, to: N, weight: usize) -> E;
    fn add_weighted_edge(edge: E);
}

pub trait EdgeRemover {
    fn remove_edge(uid: usize, vid: usize);
}

pub trait Builder<N, E>: NodeAdder<N> + NodeRemover + EdgeAdder<N, E>
where
    N: Node,
    E: Edge<N>,
{
}

pub trait WeightedBuilder<N, E>: NodeAdder<N> + WeightedEdgeAdder<N, E>
where
    N: Node,
    E: WeightedEdge<N>,
{
}

pub trait UndirectedBuilder<N, E>: Undirected<N, E> + Builder<N, E>
where
    N: Node,
    E: Edge<N>,
{
}

pub trait DirectedWeightedBuilder<N, E>: Directed<N, E> + WeightedBuilder<N, E>
where
    N: Node,
    E: WeightedEdge<N>,
{
}
