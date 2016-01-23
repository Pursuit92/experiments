use super::igraph::IGraph;
use super::NodeRef;

use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug,Clone)]
// A Strongly Connected Component is either a group of nodes in a cycle or a
// single node.
pub enum SCC<T> where T: Debug + Clone + Eq + Hash {
    Single(T),
    Group(HashSet<T>),
}

impl<T> SCC<T> where T: Debug + Clone + Eq + Hash {
    pub fn map<U, F>(self, f: F) -> SCC<U>
        where F: Fn(T) -> U, U: Debug + Clone + Eq + Hash {
        match self {
            SCC::Single(n) => SCC::Single(f(n)),
            SCC::Group(g) => SCC::Group(g.into_iter().map(|x| f(x)).collect()),
        }
    }
}

// State for Tarjan's SCC algorithm
pub struct Tarjan<'a> {
    meta: Vec<(bool, usize)>,
    dfs: Vec<NodeRef>,
    backtrack: Vec<NodeRef>,
    index: usize,
    label: usize,
    igraph: &'a IGraph,
    output: Vec<SCC<NodeRef>>,
}

impl<'a> Tarjan<'a> {
    pub fn new(igraph: &'a IGraph) -> Tarjan<'a> {
        Tarjan{
            igraph: igraph,
            meta: vec![(false, 0); igraph.size()],
            output: Default::default(),
            index: 1,
            label: igraph.size(),
            dfs: Default::default(),
            backtrack: Default::default(),
        }
    }

    // run the SCC algorithm
    pub fn scc(self) -> Vec<SCC<NodeRef>> {
        let mut s = self;
        // start the dfs from unvisited nodes. Nodes that have been
        // visited by previous searches will be skipped.
        for nr in (0..s.igraph.size()).map(|x| x.into()) {
            if ! s.visited(nr) {
                // add the node to the dfs stack and initiate the search.
                s.dfs.push(nr);
                s.search();
            }
        }

        // cleanup any nodes left on the backtrack stack.
        s.cleanup();

        // output gets populated by search and cleanup.
        s.output
    }

    fn search(&mut self) {
        loop {
            // the search is over when the search stack is empty.
            if self.dfs.len() == 0 {
                break
            }

            // peek at the top of the stack and get its label.
            let n = *self.dfs.last().unwrap();
            let link = self.label(n);

            // if link is 0, it's not yet labeled or visited.
            if link == 0 {
                // visit the node and continue the search.
                self.visit(n);
            } else {
                // we're seeing this node agian, so we must be backtracking.
                // remove it from the stack and run the backtracking logic.
                self.dfs.pop();
                self.backtrack(n, link);
            }
        }
    }

    fn visit(&mut self, n: NodeRef) {
        // set the node's label to the current index and increment it.
        self.set_label_index(n);
        self.index += 1;

        // add the children of the node to the dfs stack if they haven't been
        // visited and aren't the same node.
        for nr in self.igraph.children(n).iter() {
            if ! self.visited(*nr) && *nr != n {
                self.dfs.push(*nr);
            }
        }

        // set the visited flag on the node so we don't try to search from it
        // again.
        self.set_visited(n);
    }

    fn backtrack(&mut self, n: NodeRef, link: usize) {
        // find the minimum label on this node's children.
        let mut min = self.igraph.size();
        for nr in self.igraph.children(n).iter() {
            let label = self.label(*nr);
            if label != 0 && label < min {
                min = label;
            }
        }

        // if the minimum child label is less than the current label, it means
        // we saw it earlier in the igraph, so there must be a group that we
        // haven't finished. Otherwise, we just need to add the group to the
        // outputs.
        if min < link {
            // group isn't finished, so store this node for later and set its
            // label to the group's min. when the group is done, they'll all
            // have this min label.
            self.backtrack.push(n);
            self.set_label_to(n, min);
        } else {
            // group is done, so we can pop the nodes off of the stack that have
            // the same label and add them to the set.
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

            // give the entire group a new label
            for nr in group.iter() {
                self.set_label(*nr);
            }
            self.label -= 1;

            // since the group is finished, reset the index according to
            // the group size.
            self.index -= group.len();

            // add the group (or single node) to the output vec.
            if group.len() == 1 {
                self.output.push(SCC::Single(n));
            } else {
                self.output.push(SCC::Group(group));
            }
        }
    }

    // if there are orphan nodes, they could still be on the backtrack stack
    // after the rest are processed. This will clean them up.
    fn cleanup(&mut self) {
        while self.backtrack.len() != 0 {
            let top = self.backtrack.pop().unwrap();
            self.set_label(top);
            self.output.push(SCC::Single(top));
            self.label -= 1;
            self.index -= 1;
        }
    }

    // helper functions for manipulating labels and visited status.

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

pub fn scc(g: &IGraph) -> Vec<SCC<NodeRef>> {
    Tarjan::new(g).scc()
}
