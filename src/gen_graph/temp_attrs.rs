use super::{Graph, Node};

// So far only for nodes, but should add attrs for edges as well
pub struct TempAttrs<'a, T: 'a>{
    graph: &'a Graph,
    attrs: Vec<T>
}

impl<'a, T: Default> TempAttrs<'a, T> {
    pub fn borrow_mut(&mut self, node: &Node) -> &mut T {
        &mut self.attrs[*node]
    }

    pub fn borrow(&self, node: &Node) -> &T {
        &self.attrs[*node]
    }

    pub fn graph(&self) -> &'a Graph {
        self.graph
    }

    pub fn new(graph: &'a Graph) -> TempAttrs<'a, T> {
            let nbr_nodes = graph.get_nbr_nodes();
            let mut attrs = Vec::with_capacity(nbr_nodes);
            for _ in 0..nbr_nodes {
                attrs.push(Default::default());
            }

            TempAttrs {
                graph: graph,
                attrs: attrs
            }
    }
}


