pub mod attrs;
pub mod perm_attrs;

use std::default::Default;
use std::collections::HashMap;

use self::attrs::{Attrs, NodeAttrs, EdgeAttrs};
use self::perm_attrs::{PermAttr, PermAttrs}; 

pub type Node = usize;
pub type Edge = (Node, Node);   // TODO ?


pub struct Graph {
    nodes: Vec<PermAttrs>,
    edges: HashMap<(Node, Node), PermAttrs>,
    adj: Vec<Vec<Node>>
}

impl Graph {
    fn empty() -> Graph {
        Graph {
            adj: Default::default(),
            edges: Default::default(),
            nodes: Default::default()
        }
    }

    pub fn example() -> Graph {
        let mut  graph = Self::empty();
        
        graph.add_node([
            ("Is".to_owned(), PermAttr::Str("Apartment".to_owned())),
            ("Rooms".to_owned(), PermAttr::Num(6))
        ].into());
        graph.add_node([
            ("Is".to_owned(), PermAttr::Str("Person".to_owned())),
            ("Name".to_owned(), PermAttr::Str("Samuel".to_owned()))
        ].into());
        graph.add_node([
            ("Is".to_owned(), PermAttr::Str("Person".to_owned())),
            ("Name".to_owned(), PermAttr::Str("Kåre".to_owned()))
        ].into());

        graph.add_edge(1, 0, [
            ("Relationship".to_owned(), PermAttr::Str("Lives in".to_owned())),
        ].into());
        graph.add_edge(2, 0, [
            ("Relationship".to_owned(), PermAttr::Str("Lives in".to_owned())),
        ].into());
        graph
    }

    pub fn gen_attrs<N: Default, E: Default>(&self) -> Attrs<N, E> {
        Attrs::new(self)
    }

    pub fn gen_node_attrs<N: Default>(&self) -> NodeAttrs<N> {
        NodeAttrs::new(self)
    }

    pub fn gen_edge_attrs<E: Default>(&self) -> EdgeAttrs<E> {
        EdgeAttrs::new(self)
    }

    fn add_node(&mut self, attrs: PermAttrs) {
        self.adj.push(Default::default());
        self.nodes.push(attrs);
        return
    }

    fn add_edge(&mut self, from: Node, to: Node, attrs: PermAttrs) {
        self.adj[from].push(to);
        self.edges.insert((from, to), attrs);
    }

    pub fn get_node_attrs(&self, node: Node) -> Option<&PermAttrs> {
        return self.nodes.get(node)
    }

    pub fn get_connections(&self, node: Node) -> Option<&Vec<Node>> {
        return self.adj.get(node)
    }

    pub fn get_nbr_nodes(&self) -> usize {
        return self.nodes.len()
    }

    pub fn get_nbr_undirected_edges(&self) -> usize {
        // TODO: fix for number directed egdes as well
        // Now can count duplicate edges
        return self.edges.len()
    }
}
