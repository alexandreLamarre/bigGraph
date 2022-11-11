use std::ops::Mul;

use crate::graph::traits::Node;

pub trait Line<N>
where
    N: Node,
{
    fn from(&self) -> &N;
    fn to(&self) -> &N;

    /// returns the edge reversal of this line if it is valid for that
    /// data type, otherwise return unchanged line
    fn reversed_line(&self) -> Self;

    fn id() -> usize;
}

pub trait WeightedLine<N>: Line<N>
where
    N: Node,
{
    fn weight(&self) -> f64;
}

pub trait Multigraph<N, L>
where
    N: Node,
    L: Line<N>,
{
    fn node(&self, id: usize) -> &N;
    fn nodes(&self) -> Vec<&N>;

    fn from(&self, id: usize) -> Vec<&N>;
    fn has_edge_between(&self, xid: usize, yid: usize) -> bool;
    // lines returns the lines from u to v,
    // if no such lines exist return None
    fn lines(&self, uid: usize, vid: usize) -> Option<Vec<&L>>;
}

pub trait WeightedMultiGraph<N, L>: Multigraph<N, L>
where
    N: Node,
    L: WeightedLine<N>,
{
    fn weighted_lines(&self, uid: usize, vid: usize) -> Option<Vec<&L>>;
}

pub trait UndirectedMultiGraph<N, L>: Multigraph<N, L>
where
    N: Node,
    L: Line<N>,
{
    fn lines_between(&self, xid: usize, yid: usize) -> Option<Vec<&L>>;
}

pub trait WeightedUndirectedMultiGraph<N, L>: WeightedMultiGraph<N, L>
where
    N: Node,
    L: WeightedLine<N>,
{
    fn weighted_lines_between(&self, xid: usize, yid: usize) -> Option<Vec<&L>>;
}

pub trait DirectedMultiGraph<N, L>: Multigraph<N, L>
where
    N: Node,
    L: Line<N>,
{
    fn has_edge_from_to(&self, uid: usize, vid: usize) -> bool;
    fn to(&self, id: usize) -> Vec<&N>;
}

pub trait WeightedUndirectedMultiGraph<N, L> : WeightedMultiGraph<N, L>
where
    N: Node,
    L: WeightedLine<N>,
{
    fn has_edge_from_to(&self, uid: usize, vid: usize) -> bool;
    fn to(&self, id: usize) -> Vec<&N>;
}