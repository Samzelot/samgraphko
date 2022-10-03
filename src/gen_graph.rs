
use std::collections::BTreeMap;
use std::default::Default;
use std::collections::HashMap;

enum Attr {
    Num(i32),
    Str(String),
    Pos {x: i32, y: i32},
    Color(u8, u8, u8)
}

pub struct Attrs {
    persistent: BTreeMap<String, Attr>,
    temp: BTreeMap<String, Attr>
}

impl<const T: usize> From<[(String, Attr); T]> for Attrs {
    fn from(v: [(String, Attr); T]) -> Self {
        Attrs {
            persistent: v.into(),
            temp: Default::default()
        }
    }
}

impl Default for Attrs {
    fn default() -> Self {
        Self { persistent: Default::default(), temp: Default::default() }
    }
}

pub struct Graph {
    nodes: Vec<Attrs>,
    edges: HashMap<(usize, usize), Attrs>,
    adj: Vec<Vec<usize>>
}

impl Graph {
    pub fn example() -> Graph {
        let mut  graph = Graph::default();
        
        graph.add_node([
            ("Is".to_owned(), Attr::Str("Apartment".to_owned())),
            ("Rooms".to_owned(), Attr::Num(6))
        ].into());
        graph.add_node([
            ("Is".to_owned(), Attr::Str("Person".to_owned())),
            ("Name".to_owned(), Attr::Str("Samuel".to_owned()))
        ].into());
        graph.add_node([
            ("Is".to_owned(), Attr::Str("Person".to_owned())),
            ("Name".to_owned(), Attr::Str("Kåre".to_owned()))
        ].into());

        graph.add_edge(1, 0, [
            ("Relationship".to_owned(), Attr::Str("Lives in".to_owned())),
        ].into());
        graph.add_edge(2, 0, [
            ("Relationship".to_owned(), Attr::Str("Lives in".to_owned())),
        ].into());
        graph
    }

    fn add_node(&mut self, attrs: Attrs) {
        self.adj.push(Default::default());
        self.nodes.push(attrs);
        return
    }

    fn add_edge(&mut self, from: usize, to: usize, attrs: Attrs) {
        self.adj[from].push(to);
        self.edges.insert((from, to), attrs);
    }

    pub fn get_node_attrs(&self, ind: usize) -> Option<&Attrs> {
        return self.nodes.get(ind)
    }

    pub fn get_connections(&self, node_ind: usize) -> Option<&Vec<usize>> {
        return self.adj.get(node_ind)
    }

    pub fn get_nbr_nodes(&self) -> usize {
        return self.nodes.len()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self { nodes: Default::default(), edges: Default::default(), adj: Default::default() }
    }
}
