use std::ops::Deref;
use std::collections::HashSet;


#[derive(Debug,Default,PartialEq,Eq,Clone)]
pub struct Graph(Vec<Node>);

#[derive(Debug,Default,PartialEq,Eq,Clone)]
struct Node {
    edge_in: HashSet<NodeRef>,
    edge_out: HashSet<NodeRef>,
}

#[derive(Debug,Default,PartialEq,Eq,Copy,Clone,Hash)]
pub struct NodeRef(usize);

impl Deref for NodeRef {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Graph {
    pub fn new() -> Graph {
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

    pub fn scc(&self) -> Vec<SCC> {
        Tarjan::new(self).scc()
    }
}

#[derive(Debug,Clone)]
pub enum SCC {
    Single(NodeRef),
    Group(HashSet<NodeRef>),
}

struct Tarjan<'a> {
    meta: Vec<(bool, usize)>,
    dfs: Vec<NodeRef>,
    backtrack: Vec<NodeRef>,
    index: usize,
    label: usize,
    graph: &'a Graph,
    output: Vec<SCC>,
}

impl<'a> Tarjan<'a> {
    fn new(graph: &'a Graph) -> Tarjan<'a> {
        Tarjan{
            graph: graph,
            meta: vec![(false, 0); graph.size()],
            output: Default::default(),
            index: 1,
            label: graph.size(),
            dfs: Default::default(),
            backtrack: Default::default(),
        }
    }

    fn scc(self) -> Vec<SCC> {
        let mut s = self;
        for n in 0..s.graph.size() {
            let nr = NodeRef(n);
            if ! s.visited(nr) {
                s.dfs.push(nr);
                s.search();
            }
        }
        s.cleanup();
        s.output
    }

    fn search(&mut self) {
        loop {
            if self.dfs.len() == 0 {
                break
            }

            let n = *self.dfs.last().unwrap();
            let link = self.label(n);

            if link == 0 {
                self.visit(n);
            } else {
                self.dfs.pop();
                self.backtrack(n, link);
            }
        }
    }

    fn visit(&mut self, n: NodeRef) {
        self.set_label_index(n);
        self.index += 1;
        for nr in self.graph.parents(n).iter() {
            if ! self.visited(*nr) && *nr != n {
                self.dfs.push(*nr);
            }
        }
        self.set_visited(n);
    }

    fn backtrack(&mut self, n: NodeRef, link: usize) {
        let mut min = self.graph.size();
        for nr in self.graph.parents(n).iter() {
            let label = self.label(*nr);
            if label != 0 && label < min {
                min = label;
            }
        }

        if min < link {
            self.backtrack.push(n);
            self.set_label_to(n, min);
        } else {
            let mut group: HashSet<NodeRef> = HashSet::new();
            group.insert(n);
            while self.backtrack.len() != 0 {
                let top: NodeRef = *self.backtrack.last().unwrap();
                if self.label(top) == link {
                    group.insert(self.backtrack.pop().unwrap());
                } else {
                    break
                }
            }

            for nr in group.iter() {
                self.set_label(*nr);
            }

            self.label -= 1;
            self.index -= group.len();

            if group.len() == 1 {
                self.output.push(SCC::Single(n));
            } else {
                self.output.push(SCC::Group(group));
            }
        }
    }

    fn cleanup(&mut self) {
        while self.backtrack.len() != 0 {
            let top = self.backtrack.pop().unwrap();
            self.set_label(top);
            self.output.push(SCC::Single(top));
            self.label -= 1;
            self.index -= 1;
        }
    }

    fn label(&self, nr: NodeRef) -> usize {
        self.meta[*nr].1
    }

    fn set_label(&mut self, nr: NodeRef) {
        self.meta[*nr].1 = self.label;
    }

    fn set_label_index(&mut self, nr: NodeRef) {
        self.meta[*nr].1 = self.index;
    }

    fn set_label_to(&mut self, nr: NodeRef, label: usize) {
        self.meta[*nr].1 = label;
    }

    fn visited(&self, nr: NodeRef) -> bool {
        self.meta[*nr].0
    }

    fn set_visited(&mut self, nr: NodeRef) {
        self.meta[*nr].0 = true;
    }
}

#[test]
fn do_stuff() {
    let mut a: Graph = Default::default();

    for _ in 0..6 {
        a.create_node();
        // a.add_edge(np, n);
        // np = n;
    }
    a.create_node();
    a.create_node();

    a.add_edge(NodeRef(4), NodeRef(5));
    a.add_edge(NodeRef(0), NodeRef(1));
    a.add_edge(NodeRef(3), NodeRef(1));
    a.add_edge(NodeRef(4), NodeRef(3));
    a.add_edge(NodeRef(2), NodeRef(4));
    a.add_edge(NodeRef(1), NodeRef(2));
    a.add_edge(NodeRef(0), NodeRef(0));
    a.add_edge(NodeRef(3), NodeRef(3));

    // a.add_edge(n2, n1);
    println!("{:?}", a.scc());
    panic!();
}
