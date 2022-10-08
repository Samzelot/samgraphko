pub mod temp_attrs;
pub mod perm_attrs;

use std::default::Default;
use std::collections::HashMap;

use self::temp_attrs::TempAttrs;
use self::perm_attrs::{PermAttr, PermAttrs}; 

pub type Node = usize;


pub struct Graph {
    nodes: Vec<PermAttrs>,
    edges: HashMap<(usize, usize), PermAttrs>,
    adj: Vec<Vec<usize>>
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

    pub fn gen_attrs<'a, T: Default + 'a>(&'a self) -> TempAttrs<'a, T> {
        TempAttrs::new(self)
    }

    fn add_node(&mut self, attrs: PermAttrs) {
        self.adj.push(Default::default());
        self.nodes.push(attrs);
        return
    }

    fn add_edge(&mut self, from: usize, to: usize, attrs: PermAttrs) {
        self.adj[from].push(to);
        self.edges.insert((from, to), attrs);
    }

    pub fn get_node_attrs(&self, ind: usize) -> Option<&PermAttrs> {
        return self.nodes.get(ind)
    }

    pub fn get_connections(&self, node_ind: usize) -> Option<&Vec<usize>> {
        return self.adj.get(node_ind)
    }

    pub fn get_nbr_nodes(&self) -> usize {
        return self.nodes.len()
    }
}
