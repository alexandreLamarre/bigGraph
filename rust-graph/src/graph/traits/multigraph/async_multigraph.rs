use async_trait::async_trait;
use std::fmt::Debug;
use std::option::IntoIter;

use crate::graph::traits::async_graph::AsyncNode;

use super::{Multigraph, WeightedLine};

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
