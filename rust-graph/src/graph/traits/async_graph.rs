use async_trait::async_trait;
use std::fmt::Debug;
use std::option::IntoIter;

#[async_trait]
pub trait AsyncNode {
    async fn id(&self) -> usize;
}
//pub trait AsyncNode : Debug + Clone + Size + PartialEq {
//    async fn id(&self) -> usize;
//}

#[async_trait]
pub trait AsyncEdge<T>: Debug + Clone + Sized + PartialEq
where
    T: AsyncNode,
{
    async fn from(&self) -> &T;
    async fn to(&self) -> &T;
}

#[async_trait]
pub trait AsyncWeightedEdge<T>: AsyncEdge<T>
where
    T: AsyncNode,
{
    async fn weight(&self) -> f64;
}

#[async_trait]
pub trait AsyncGraph<N, E>
where
    N: AsyncNode,
    E: AsyncEdge<N>,
{
    async fn node(&self, id: usize) -> Option<&N>;
    async fn nodes(&self) -> Option<IntoIter<&N>>;

    async fn from(&self, id: usize) -> Vec<&N>;
    async fn has_edge_between(&self, xid: usize, yid: usize) -> bool;
    async fn edge(&self, uid: usize, vid: usize) -> Option<&E>;
}

#[async_trait]
pub trait AsyncDirected<N, E>: AsyncGraph<N, E>
where
    N: AsyncNode,
    E: AsyncEdge<N>,
{
    async fn has_edge_from_to(&self, uid: usize, vid: usize) -> bool;
    async fn to(&self, id: usize) -> Vec<&N>;
}

#[async_trait]
pub trait AsyncWeighted<N, E>: AsyncGraph<N, E>
where
    N: AsyncNode,
    E: AsyncWeightedEdge<N>,
{
    async fn weighted_edge(&self, uid: usize, vid: usize) -> Option<&E>;
    async fn weight(&self, uid: usize, vid: usize) -> Option<f64>;
}

#[async_trait]
pub trait AsyncUndirected<N, E>: AsyncGraph<N, E>
where
    N: AsyncNode,
    E: AsyncEdge<N>,
{
    async fn edge_between(&self, xid: usize, yid: usize) -> Option<&E>;
}

#[async_trait]
pub trait AsyncWeightedUndirected<N, E>: AsyncWeighted<N, E>
where
    N: AsyncNode,
    E: AsyncWeightedEdge<N>,
{
    async fn weighted_edge_between(&self, xid: usize, yid: usize) -> Option<&E>;
}

#[async_trait]
pub trait AsyncWeightedDirected<N, E>: AsyncWeighted<N, E> + AsyncDirected<N, E>
where
    N: AsyncNode,
    E: AsyncWeightedEdge<N>,
{
    async fn has_edge_from_to(&self, uid: usize, vid: usize) -> bool;
    async fn to(&self, id: usize) -> Vec<&N>;
}

#[async_trait]
pub trait AsyncNodeAdder<N>
where
    N: AsyncNode,
{
    async fn new_node() -> N;
    async fn add_node(&mut self, node: N);
}

#[async_trait]
pub trait AsyncNodeRemover {
    async fn remove_node(&mut self, id: usize);
}

#[async_trait]
pub trait AsyncEdgeAdder<N, E>
where
    N: AsyncNode,
    E: AsyncEdge<N>,
{
    async fn new_edge(&self, from: &N, to: &N) -> E;
    async fn add_edge(&mut self, edge: E);
}

#[async_trait]
pub trait AsyncWeightedEdgeAdder<N, E>
where
    N: AsyncNode,
    E: AsyncWeightedEdge<N>,
{
    async fn new_weighted_edge(&self, from: &N, to: &N, weight: f64) -> E;
    async fn add_weighted_edge(&mut self, edge: E);
}

#[async_trait]
pub trait AsyncEdgeRemover {
    async fn remove_edge(&mut self, uid: usize, vid: usize);
}

#[async_trait]
pub trait AsyncBuilder<N, E>: AsyncNodeAdder<N> + AsyncEdgeAdder<N, E>
where
    N: AsyncNode,
    E: AsyncEdge<N>,
{
}

#[async_trait]
pub trait AsyncWeightedBuilder<N, E>: AsyncBuilder<N, E> + AsyncWeightedEdgeAdder<N, E>
where
    N: AsyncNode,
    E: AsyncWeightedEdge<N>,
{
}

#[async_trait]
pub trait AsyncUndirectedBuilder<N, E>: AsyncUndirected<N, E> + AsyncBuilder<N, E>
where
    N: AsyncNode,
    E: AsyncEdge<N>,
{
}

#[async_trait]
pub trait AsyncUndirectedWeightedBuilder<N, E>:
    AsyncUndirected<N, E> + AsyncWeightedBuilder<N, E>
where
    N: AsyncNode,
    E: AsyncWeightedEdge<N>,
{
}

#[async_trait]
pub trait AsyncDirectedBuilder<N, E>: AsyncDirected<N, E> + AsyncBuilder<N, E>
where
    N: AsyncNode,
    E: AsyncEdge<N>,
{
}

#[async_trait]
pub trait AsyncDirectedWeightedBuilder<N, E>:
    AsyncDirected<N, E> + AsyncWeightedBuilder<N, E>
where
    N: AsyncNode,
    E: AsyncWeightedEdge<N>,
{
}
