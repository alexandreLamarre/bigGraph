mod async_multigraph;

use std::ops::Mul;

use crate::graph::traits::{Node, NodeAdder};

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
    fn node(&self, id: usize) -> Option<&N>;
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

pub trait WeightedDirectedMultiGraph<N, L>: WeightedMultiGraph<N, L>
where
    N: Node,
    L: WeightedLine<N>,
{
    fn has_edge_from_to(&self, uid: usize, vid: usize) -> bool;
    fn to(&self, id: usize) -> Vec<&N>;
}

pub trait LineAdder<N, L>
where
    N: Node,
    L: Line<N>,
{
    fn new_line(from: N, to: N) -> L;
    fn add_line(&mut self, from: &N, to: &N) -> L;
}

pub trait WeightedLineAdder<N, L>: LineAdder<N, L>
where
    N: Node,
    L: WeightedLine<N>,
{
    fn new_weighted_line(from: N, to: N, weight: f64) -> L;
    fn add_weighted_line(&mut self, from: &N, to: &N, weight: f64) -> L;
}

pub trait LineRemover<N, L>
where
    N: Node,
    L: Line<N>,
{
    fn remove_line(&mut self, from: &N, to: &N);
}

pub trait MultiGraphBuilder<N, L>: NodeAdder<N> + LineAdder<N, L>
where
    N: Node,
    L: Line<N>,
{
}

pub trait WeightedMultiGraphBuilder<N, L>: NodeAdder<N> + WeightedLineAdder<N, L>
where
    N: Node,
    L: WeightedLine<N>,
{
}

pub trait UndirectedMultiGraphBuilder<N, L>: MultiGraphBuilder<N, L>
where
    N: Node,
    L: Line<N>,
{
}

pub trait UndirectedWeightedMultiGraphBuilder<N, L>: WeightedMultiGraphBuilder<N, L>
where
    N: Node,
    L: WeightedLine<N>,
{
}

pub trait DirectedMultiGraphBuilder<N, L>:
    DirectedMultiGraph<N, L> + MultiGraphBuilder<N, L>
where
    N: Node,
    L: Line<N>,
{
}

pub trait DirectedWeightedMultiGraphBuilder<N, L>:
    DirectedMultiGraph<N, L> + WeightedMultiGraphBuilder<N, L>
where
    N: Node,
    L: WeightedLine<N>,
{
}
