use igraph::IGraph;
use NodeRef;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct VGraph<T> where T: Eq + Hash + Clone {
    igraph: IGraph,
    vs: HashMap<T, NodeRef>,
    ns: Vec<T>,
}

impl<T> Deref for VGraph<T> where T: Eq + Hash + Clone {
    type Target = IGraph;

    fn deref(&self) -> &Self::Target {
        &self.igraph
    }
}

impl<T> DerefMut for VGraph<T> where T: Eq + Hash + Clone {
    fn deref_mut(&mut self) -> &mut IGraph {
        &mut self.igraph
    }
}

impl<T> VGraph<T> where T: Eq + Hash + Clone {
    pub fn new() -> VGraph<T> {
        VGraph {
            igraph: IGraph::new(),
            vs: HashMap::new(),
            ns: vec![],
        }
    }

    pub fn create_node(&mut self, val: T) -> NodeRef {
        let nr = self.igraph.create_node();
        self.vs.insert(val.clone(), nr);
        self.ns.push(val);
        nr
    }

    pub fn node(&self, val: &T) -> Option<NodeRef> {
        self.vs.get(val).map(|x| *x)
    }

    pub fn value(&self, nr: NodeRef) -> T {
        self.ns[*nr].clone()
    }

    pub fn add_edge(&mut self, from: &T, to: &T) -> Option<()> {
        let from_r = match self.node(from) {
            Some(nr) => nr,
            None => { return None },
        };
        let to_r = match self.node(from) {
            Some(nr) => nr,
            None => { return None },
        };

        self.deref_mut().add_edge(from_r, to_r);

        Some(())
    }
}
