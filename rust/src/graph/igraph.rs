use super::NodeRef;

use std::collections::HashSet;

#[derive(Debug,Default,PartialEq,Eq,Clone)]
pub struct IGraph(Vec<Node>);

#[derive(Debug,Default,PartialEq,Eq,Clone)]
struct Node {
    edge_in: HashSet<NodeRef>,
    edge_out: HashSet<NodeRef>,
}

impl IGraph {
    pub fn new() -> IGraph {
        Default::default()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn create_node(&mut self) -> NodeRef {
        self.0.push(Default::default());
        NodeRef(self.size() - 1)
    }

    pub fn add_edge(&mut self, from: NodeRef, to: NodeRef) {
        self.0[*from].edge_out.insert(to);
        self.0[*to].edge_in.insert(from);
    }

    pub fn remove_edge(&mut self, from: NodeRef, to: NodeRef) {
        self.0[*from].edge_out.remove(&to);
        self.0[*to].edge_in.remove(&from);
    }

    pub fn children(&self, node: NodeRef) -> &HashSet<NodeRef> {
        &self.0[*node].edge_out
    }

    pub fn parents(&self, node: NodeRef) -> &HashSet<NodeRef> {
        &self.0[*node].edge_in
    }
}
