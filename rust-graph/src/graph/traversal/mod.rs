use crate::graph::traits::{Edge, Graph, Node};
use std::collections::HashSet;
struct BreadthFirstTraversal<'a, G, N, E>
where
    E: Edge<N>,
    N: Node,
    G: Graph<N, E>,
{
    graph: &'a G,
    /// visit is called on all nodes on their first visit
    visit: fn(node: &N),

    /// traverse is called on all nodes on their first visit
    /// during the walk. This includes edges that would hop
    /// to an already visited node.
    ///
    /// the value returned by traverse determines whether an edge
    /// can  be traversed  during the walk
    traverse: fn(edge: &E) -> bool,
    visited: HashSet<usize>,
}

impl<'a, G, N, E> BreadthFirstTraversal<'a, G, N, E>
where
    E: Edge<N>,
    N: Node,
    G: Graph<N, E>,
{
    fn new(
        graph: &'a G,
        visit: fn(node: &N),
        traverse: fn(edge: &E) -> bool,
    ) -> BreadthFirstTraversal<'a, G, N, E> {
        BreadthFirstTraversal {
            graph,
            visit,
            traverse,
            visited: HashSet::new(),
        }
    }

    /// Walks the graph starting from the given node id.
    ///
    /// If the node id is not found in the graph, the walk
    /// is aborted.
    fn walk(
        &mut self,
        start: usize,
        until: Option<fn(node: &N, depth: i64) -> bool>,
    ) -> Option<&N> {
        let mut queue = Vec::new();

        let start_node = self.graph.node(start);
        if start_node.is_none() {
            return None;
        }

        queue.push(start_node.unwrap());
        self.visited.insert(start_node.unwrap().id());

        let mut depth = 0;
        let mut children = 0;
        let mut untilNext = 1;

        while queue.len() > 0 {
            let node = queue.remove(0);
            (self.visit)(node);
            if until.is_some() {
                if (until.unwrap())(node, depth) {
                    return Some(node);
                }
            }

            let neighbors = self.graph.from(node.id());
            for neighbor in neighbors {
                if self.visited.get(&neighbor.id()).is_none() {
                    continue;
                }
                let edge = self.graph.edge(node.id(), neighbor.id());
                if edge.is_none() {
                    continue;
                }
                let edge = edge.unwrap();
                if !(self.traverse)(&edge) {
                    continue;
                }
                self.visited.insert(neighbor.id());
                children += 1;
                queue.push(neighbor);
            }
            untilNext -= 1;
            if untilNext == 0 {
                depth += 1;
                untilNext = children;
                children = 0;
            }
        }
        None
    }

    fn walk_all(
        &mut self,
        after: Option<fn()>,
        before: Option<fn()>,
        during: Option<fn(node: &N, depth: i64) -> bool>,
    ) {
        self.reset();
        let nodes = self.graph.nodes();
        if nodes.is_none() {
            return;
        }
        for node in nodes.unwrap() {
            let from = node;
            if self.visited.get(&from.id()).is_none() {
                continue;
            }
            if before.is_some() {
                (before.unwrap())();
            }
            self.walk(node.id(), during);

            if after.is_some() {
                (after.unwrap())();
            }
        }
    }

    fn reset(&mut self) {
        self.visited.clear();
    }
}

struct DepthFirstTraversal<'a, G, N, E>
where
    E: Edge<N>,
    N: Node,
    G: Graph<N, E>,
{
    graph: &'a G,
    /// visit is called on all nodes on their first visit
    visit: fn(node: &N),

    /// traverse is called on all nodes on their first visit
    /// during the walk. This includes edges that would hop
    /// to an already visited node.
    ///
    /// the value returned by traverse determines whether an edge
    /// can  be traversed  during the walk
    traverse: fn(edge: &E) -> bool,
    visited: HashSet<usize>,
    stack: Vec<usize>,
}

impl<'a, G, N, E> DepthFirstTraversal<'a, G, N, E>
where
    E: Edge<N>,
    N: Node,
    G: Graph<N, E>,
{
    fn new(
        graph: &'a G,
        visit: fn(node: &N),
        traverse: fn(edge: &E) -> bool,
    ) -> BreadthFirstTraversal<'a, G, N, E> {
        BreadthFirstTraversal {
            graph,
            visit,
            traverse,
            visited: HashSet::new(),
        }
    }

    fn reset(&mut self) {
        self.visited.clear();
        self.stack.clear();
    }

    /// Walks the graph starting from the given node id.
    ///
    /// If the until function returns true, the walk is aborted and
    /// a ref to the node that evalueated to true is returned
    /// If no nodes are found in the graph, the walk returns None
    fn walk(&mut self, start: usize, until: Option<fn(node: &N) -> bool>) -> Option<&N> {
        self.stack.push(start);
        while self.stack.len() != 0 {
            let uid = self.stack.pop().unwrap();
            if self.visited.get(&uid).is_some() {
                continue;
            }
            let n = self.graph.node(uid);
            if until.is_some() && (until.unwrap())(n.unwrap()) {
                return n;
            }

            self.visited.insert(uid);
            // self.visit(self.graph.node(uid).unwrap());
            let to = self.graph.from(uid);
            for node in to {
                let edge = self.graph.edge(uid, node.id());
                if edge.is_none() {
                    continue;
                }
                let edge = edge.unwrap();
                if !(self.traverse)(&edge) {
                    continue;
                }
                self.stack.push(node.id());
            }
        }
        None
    }

    fn walk_all(
        &mut self,
        after: Option<fn()>,
        before: Option<fn()>,
        during: Option<fn(node: &N) -> bool>,
    ) {
        self.reset();
        let nodes = self.graph.nodes();
        if nodes.is_none() {
            return;
        }
        for node in nodes.unwrap() {
            let from = node;
            if self.visited.get(&from.id()).is_none() {
                continue;
            }
            if before.is_some() {
                (before.unwrap())();
            }
            self.walk(node.id(), during);
            if after.is_some() {
                (after.unwrap())();
            }
        }
    }
}
