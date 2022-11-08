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
    fn walk(&mut self, start: usize) {
        let mut queue = Vec::new();

        let start_node = self.graph.node(start);
        if start_node.is_none() {
            return;
        }

        queue.push(start_node.unwrap());
        self.visited.insert(start_node.unwrap().id());

        while queue.len() > 0 {
            let node = queue.remove(0);
            (self.visit)(node);

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
                queue.push(neighbor);
                self.visited.insert(neighbor.id());
            }
        }
    }

    fn walk_all(&mut self, after: Option<fn()>, before: Option<fn()>) {
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
            self.walk(node.id());

            if after.is_some() {
                (after.unwrap())();
            }
        }
    }

    fn reset(&mut self) {
        self.visited.clear();
    }
}
