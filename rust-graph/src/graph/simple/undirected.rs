use crate::graph::{
    simple::{SimpleEdge, SimpleNode},
    traits::{Edge, EdgeAdder, Graph, Node, NodeAdder, Undirected},
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
    fn nodes(&self) -> Iter<&SimpleNode> {
        todo!("not  implemented yet")
    }

    fn from(&self, id: usize) -> Vec<&SimpleNode> {
        todo!("not  implemented yet")
    }

    fn has_edge_between(&self, xid: usize, yid: usize) -> bool {
        todo!("not  implemented yet")
    }

    fn edge(&self, uid: usize, vid: usize) -> Option<&SimpleEdge> {
        todo!("not  implemented yet")
    }
}

impl NodeAdder<SimpleNode> for SimpleUndirectedGraph {
    fn new_node() -> SimpleNode {
        SimpleNode { value: 0 }
    }

    fn add_node(&mut self, node: SimpleNode) {
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

impl Undirected<SimpleNode, SimpleEdge> for SimpleUndirectedGraph {}
