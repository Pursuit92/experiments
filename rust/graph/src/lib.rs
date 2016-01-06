use std::ops::Deref;

mod igraph;
mod scc;
mod vgraph;

pub use igraph::*;
pub use scc::scc;

#[derive(Debug,Default,PartialEq,Eq,Copy,Clone,Hash)]
pub struct NodeRef(usize);

impl Deref for NodeRef {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for NodeRef {
    fn from(other: usize) -> Self {
        NodeRef(other)
    }
}


#[cfg(test)]
mod test {
    use scc;
    use IGraph;
    use scc::SCC;
    use vgraph::VGraph;

    #[test]
    fn do_stuff() {
        let mut a: VGraph<String> = Default::default();

        for i in 0..6 {
            a.create_node(format!("{}", i));
            // a.add_edge(np, n);
            // np = n;
        }
        a.create_node("a".into());
        a.create_node("b".into());

        (*a).add_edge(4.into(), 5.into());
        (*a).add_edge(0.into(), 1.into());
        (*a).add_edge(3.into(), 1.into());
        (*a).add_edge(4.into(), 3.into());
        (*a).add_edge(2.into(), 4.into());
        (*a).add_edge(1.into(), 2.into());
        (*a).add_edge(0.into(), 0.into());
        (*a).add_edge(3.into(), 3.into());

        // a.add_edge(n2, n1);
        println!("{:?}", scc(&a).into_iter().map(|c| c.map(|x| a.value(x))).collect::<Vec<SCC<String>>>());
        panic!();
    }
}
