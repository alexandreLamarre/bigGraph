use crate::graph::{
    simple::{SimpleEdge, SimpleNode},
    traits::{Builder, Edge, EdgeAdder, Graph, Node, NodeAdder, Undirected, UndirectedBuilder},
};
use std::collections::BTreeMap;
use std::option::Iter;

#[derive(Debug)]
struct SimpleUndirectedGraph {
    nodes: BTreeMap<usize, SimpleNode>,
    edges: BTreeMap<usize, BTreeMap<usize, SimpleEdge>>,
}

impl Graph<SimpleNode, SimpleEdge> for SimpleUndirectedGraph {
    fn node(&self, id: usize) -> Option<&SimpleNode> {
        self.nodes.get(&id)
    }
    fn nodes(&self) -> Option<Iter<&SimpleNode>> {
        if self.nodes.len() == 0 {
            return None;
        }
        //  >:(
        todo!("Implement nodes() for SimpleUndirectedGraph")
    }

    fn from(&self, id: usize) -> Vec<&SimpleNode> {
        todo!("Implement from() for SimpleUndirectedGraph")
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
        todo!("Implement edge_between() for SimpleUndirectedGraph")
    }
}

impl Builder<SimpleNode, SimpleEdge> for SimpleUndirectedGraph {}

impl UndirectedBuilder<SimpleNode, SimpleEdge> for SimpleUndirectedGraph {}
