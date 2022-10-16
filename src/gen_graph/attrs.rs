use std::collections::HashMap;

use super::{Graph, Node, Edge};

// TODO how should lifetimes be? On E and N, or on vec?
pub struct Attrs<N, E> {
    node_attrs: NodeAttrs<N>,
    edge_attrs: EdgeAttrs<E>
}

pub struct NodeAttrs<N> {
    attrs: Vec<N>
}

pub struct EdgeAttrs<E> {
    attrs: Vec<E>,
    map: HashMap<(Node, Node), usize>   // TODO: lifetime
}


impl<'a, N: Default, E: Default> Attrs<N, E> {

    pub fn get_mut_node(&mut self, node: &Node) -> &mut N {
        self.node_attrs.get_mut(node)
    }

    pub fn get_node(&self, node: &Node) -> &N {
        self.node_attrs.get(node)
    }

    pub fn get_mut_edge(&mut self, edge: &Edge) -> &mut E {
        self.edge_attrs.get_mut(edge)
    }

    pub fn get_edge(&mut self, edge: &Edge) -> &E {
        self.edge_attrs.get(edge)
    }

    pub fn new(graph: &'a Graph) -> Attrs<N, E> {
            Attrs {
                node_attrs: NodeAttrs::new(graph),
                edge_attrs: EdgeAttrs::new(graph)
            }
    }
}


impl<'a, N: Default> NodeAttrs<N> {
    pub fn get_mut(&mut self, node: &Node) -> &mut N {
        &mut self.attrs[*node]
    }

    pub fn get(&self, node: &Node) -> &N {
        &self.attrs[*node]
    }

    pub fn new(graph: &'a Graph) -> NodeAttrs<N> {
        let nbr_nodes = graph.get_nbr_nodes();
        let mut attrs = Vec::with_capacity(nbr_nodes);
        for _ in 0..nbr_nodes {
            attrs.push(Default::default());
        }

        NodeAttrs {
            attrs
        }
    }
}


impl<'a, E: Default> EdgeAttrs<E> {
    pub fn get_mut(&mut self, edge: &Edge) -> &mut E {
        // TODO This does not work for undirected edges
        let ind = 
            match self.map.get(edge) {
                // Sort of like a safe get_ind. Can add ind and default elem
                Some(ind)   => *ind,
                None        => {
                    let ind = self.map.len();
                    self.map.insert(*edge, ind);
                    
                    self.attrs.push(Default::default());
                    ind
                },
            };
        &mut self.attrs[ind]
    }

    pub fn get(&mut self,  edge: &Edge) -> &E {
        // TODO since not initialized hashmap we need to have mut self.
        // So might as well re-use the other method
        self.get_mut(edge)
    }

    pub fn new(_graph: &Graph) -> EdgeAttrs<E> {
        // TODO discuss moving back to indexes for edges. Could then re-use
        // code for node and edge attrs, as well as much easier handling of 
        // undirected edges as well as quicker access times.

        EdgeAttrs {
            attrs: Vec::new(),
            map: HashMap::new()
        }
    }
}

