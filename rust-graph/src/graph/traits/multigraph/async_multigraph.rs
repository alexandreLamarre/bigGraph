use async_trait::async_trait;
use std::fmt::Debug;
use std::option::IntoIter;

use crate::graph::traits::async_graph::{AsyncNode, AsyncNodeAdder};

#[async_trait]
pub trait AsyncLine<N>
where
    N: AsyncNode,
{
    async fn from(&self) -> &N;
    async fn to(&self) -> &N;

    // returns the edge reversal of this line if it is valid for that
    // data type, otherwise return unchanged line
    async fn reversed_line(&self) -> Self;
    async fn id() -> usize;
}

#[async_trait]
pub trait AsyncWeightedLine<N>: AsyncLine<N>
where
    N: AsyncNode,
{
    async fn weight(&self) -> f64;
}

#[async_trait]
pub trait AsyncMultigraph<N, L>
where
    N: AsyncNode,
    L: AsyncLine<N>,
{
    async fn node(&self, id: usize) -> Option<&N>;
    async fn nodes(&self) -> Vec<&N>;

    async fn from(&self, id: usize) -> Vec<&N>;
    async fn has_edge_between(&self, xid: usize, yid: usize) -> bool;
    // lines returns the lines from u to v,
    // if no such lines exist return None
    async fn lines(&self, uid: usize, vid: usize) -> Option<Vec<&L>>;
}

#[async_trait]
pub trait AsyncWeightedMultiGraph<N, L>: AsyncMultigraph<N, L>
where
    N: AsyncNode,
    L: AsyncWeightedLine<N>,
{
    async fn weighted_lines(&self, uid: usize, vid: usize) -> Option<Vec<&L>>;
}

#[async_trait]
pub trait AsyncUndirectedMultiGraph<N, L>
where
    N: AsyncNode,
    L: AsyncLine<N>,
{
    async fn lines_between(&self, xid: usize, yid: usize) -> Option<Vec<&L>>;
}

#[async_trait]
pub trait AsyncWeightedUndirectedMultiGraph<N, L>: AsyncWeightedMultiGraph<N, L>
where
    N: AsyncNode,
    L: AsyncWeightedLine<N>,
{
    async fn weighted_lines_between(&self, xid: usize, y: usize) -> Option<Vec<&L>>;
}

#[async_trait]
pub trait AsyncDirectedMultiGraph<N, L>: AsyncMultigraph<N, L>
where
    N: AsyncNode,
    L: AsyncLine<N>,
{
    async fn has_edge_from_to(&self, uid: usize, vid: usize) -> bool;
    async fn to(&self, id: usize) -> Vec<&N>;
}

#[async_trait]
pub trait AsyncWeightedDirectedMultiGraph<N, L>: AsyncWeightedMultiGraph<N, L>
where
    N: AsyncNode,
    L: AsyncWeightedLine<N>,
{
    async fn has_edge_from_to(&self, uid: usize, vid: usize) -> bool;
    async fn to(&self, id: usize) -> Vec<&N>;
}

#[async_trait]
pub trait AsyncLineAdder<N, L>
where
    N: AsyncNode,
    L: AsyncLine<N>,
{
    async fn new_line(from: N, to: N) -> L;
    async fn add_line(&mut self, from: &N, to: &N) -> L;
}

#[async_trait]
pub trait AsyncWeightedLineAdder<N, L>: AsyncLineAdder<N, L>
where
    N: AsyncNode,
    L: AsyncWeightedLine<N>,
{
    async fn new_weighted_line(from: N, to: N, weight: f64) -> L;
    async fn add_weighted_line(&mut self, from: &N, to: &N, weight: f64) -> L;
}

#[async_trait]
pub trait AsyncLineRemover<N, L>
where
    N: AsyncNode,
    L: AsyncLine<N>,
{
    async fn remove_line(&mut self, from: &N, to: &N);
}

#[async_trait]
pub trait AsyncMultiGraphBuilder<N, L>: AsyncNodeAdder<N> + AsyncLineAdder<N, L>
where
    N: AsyncNode,
    L: AsyncLine<N>,
{
}

#[async_trait]
pub trait AsyncWeightedMultiGraphBuilder<N, L>:
    AsyncNodeAdder<N> + AsyncWeightedLineAdder<N, L>
where
    N: AsyncNode,
    L: AsyncWeightedLine<N>,
{
}

#[async_trait]
pub trait AsyncUndirectedMultiGraphBuilder<N, L>: AsyncMultiGraphBuilder<N, L>
where
    N: AsyncNode,
    L: AsyncLine<N>,
{
}

#[async_trait]
pub trait AsyncUndirectedWeightedMultiGraphBuilder<N, L>:
    AsyncWeightedMultiGraphBuilder<N, L>
where
    N: AsyncNode,
    L: AsyncWeightedLine<N>,
{
}

#[async_trait]
pub trait AsyncDirectedMultiGraphBuilder<N, L>:
    AsyncDirectedMultiGraph<N, L> + AsyncMultiGraphBuilder<N, L>
where
    N: AsyncNode,
    L: AsyncLine<N>,
{
}

#[async_trait]
pub trait AsyncDirectedWeightedMultiGraphBuilder<N, L>:
    AsyncDirectedMultiGraph<N, L> + AsyncWeightedMultiGraphBuilder<N, L>
where
    N: AsyncNode,
    L: AsyncWeightedLine<N>,
{
}
