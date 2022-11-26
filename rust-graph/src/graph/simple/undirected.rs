use crate::graph::{
    simple::{SimpleEdge, SimpleNode},
    traits::{Builder, Edge, EdgeAdder, Graph, Node, NodeAdder, Undirected, UndirectedBuilder},
};
use std::collections::BTreeMap;
use std::iter::Iterator;

#[derive(Debug)]
struct SimpleUndirectedGraph {
    nodes: BTreeMap<usize, SimpleNode>,
    edges: BTreeMap<usize, BTreeMap<usize, SimpleEdge>>,
}

impl Graph<SimpleNode, SimpleEdge> for SimpleUndirectedGraph {
    fn node(&self, id: usize) -> Option<&SimpleNode> {
        self.nodes.get(&id)
    }
    fn nodes(&self) -> Option<Vec<&SimpleNode>> {
        if self.nodes.len() == 0 {
            return None;
        }
        let v = self.nodes.iter().map(|(_, n)| n).collect();
        Some(v)
    }

    fn from(&self, id: usize) -> Vec<&SimpleNode> {
        let mut v = Vec::new();
        if let Some(edges) = self.edges.get(&id) {
            for (idx, _) in edges.iter() {
                match self.node(*idx) {
                    Some(n) => v.push(n),
                    None => (),
                }
            }
        }
        v
    }

    fn has_edge_between(&self, xid: usize, yid: usize) -> bool {
        self.edges.get(&xid).map_or(false, |m| m.contains_key(&yid))
    }

    fn edge(&self, uid: usize, vid: usize) -> Option<&SimpleEdge> {
        self.edges.get(&uid).map_or(None, |m| m.get(&vid))
    }
}

impl NodeAdder<SimpleNode> for SimpleUndirectedGraph {
    fn new_node() -> SimpleNode {
        SimpleNode { value: 0 }
    }

    fn add_node(&mut self, node: SimpleNode) {
        if self.nodes.get(&node.id()).is_some() {
            panic!("node id collision : {}", node.id());
        }
        self.nodes.insert(node.id(), node);
    }
}

impl EdgeAdder<SimpleNode, SimpleEdge> for SimpleUndirectedGraph {
    fn new_edge(from: SimpleNode, to: SimpleNode) -> SimpleEdge {
        SimpleEdge { f: from, t: to }
    }

    fn add_edge(&mut self, edge: SimpleEdge) {
        let from = edge.from();
        let to = edge.to();
        self.edges.insert(from.id(), BTreeMap::new());
        self.edges.insert(to.id(), BTreeMap::new());
        self.edges
            .get_mut(&from.id())
            .unwrap()
            .insert(to.id(), edge.clone());
        self.edges
            .get_mut(&to.id())
            .unwrap()
            .insert(from.id(), edge);
    }
}

impl Undirected<SimpleNode, SimpleEdge> for SimpleUndirectedGraph {
    fn edge_between(&self, xid: usize, yid: usize) -> Option<&SimpleEdge> {
        if let Some(edges) = self.edges.get(&xid) {
            edges.get(&yid)
        } else {
            None
        }
    }
}

impl Builder<SimpleNode, SimpleEdge> for SimpleUndirectedGraph {}

impl UndirectedBuilder<SimpleNode, SimpleEdge> for SimpleUndirectedGraph {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn empty_graph() -> SimpleUndirectedGraph {
        SimpleUndirectedGraph {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
        }
    }

    #[fixture]
    fn single_node_graph() -> SimpleUndirectedGraph {
        let mut g = SimpleUndirectedGraph {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
        };
        g.add_node(SimpleUndirectedGraph::new_node());
        g
    }

    #[fixture]
    fn fully_connected_graph() -> SimpleUndirectedGraph {
        let mut g = SimpleUndirectedGraph {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
        };
        let mut n1 = SimpleUndirectedGraph::new_node();
        let mut n2 = SimpleUndirectedGraph::new_node();
        let mut n3 = SimpleUndirectedGraph::new_node();
        n1.value = 1;
        n2.value = 2;
        n3.value = 3;
        g.add_node(n1.clone());
        g.add_node(n2.clone());
        g.add_node(n3.clone());
        g.add_edge(SimpleUndirectedGraph::new_edge(n1.clone(), n2.clone()));
        g.add_edge(SimpleUndirectedGraph::new_edge(n1.clone(), n3.clone()));
        g.add_edge(SimpleUndirectedGraph::new_edge(n2.clone(), n3.clone()));
        g
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn add_node_test(#[case] id: usize) {
        let mut g = SimpleUndirectedGraph {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
        };
        let mut n = SimpleUndirectedGraph::new_node();
        n.value = id;
        g.add_node(n);
        assert_eq!(g.node(id).unwrap().value, id);
    }
}
