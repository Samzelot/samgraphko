use super::{Graph, Node, Edge};

// TODO how should lifetimes be? On E and N, or on vec?
pub struct Attrs<N, E> {
    node_attrs: VecAttrs<N>,
    edge_attrs: VecAttrs<E>
}

pub struct VecAttrs<N> {
    attrs: Vec<N>
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
                node_attrs: VecAttrs::new(graph.nbr_nodes()),
                edge_attrs: VecAttrs::new(graph.nbr_edges()),
            }
    }
}


impl<'a, N: Default> VecAttrs<N> {
    pub fn get_mut(&mut self, ind: &usize) -> &mut N {
        &mut self.attrs[*ind]
    }

    pub fn get(&self, ind: &usize) -> &N {
        &self.attrs[*ind]
    }

    pub fn new(nbr_attrs: usize) -> VecAttrs<N> {
        let attrs = (0..nbr_attrs)
            .map(|_| Default::default())
            .collect();

        Self {
            attrs
        }
    }
}

/*
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
*/
