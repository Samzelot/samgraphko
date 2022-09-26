
use std::collections::BTreeMap;
use std::default::Default;
use std::collections::HashMap;

enum Attr {
    Num(i32),
    Str(String),
    Pos {x: i32, y: i32},
    Color(u8, u8, u8)
}

pub struct AttrSet {
    persistent: BTreeMap<String, Attr>,
    temp: BTreeMap<String, Attr>
}

impl Default for AttrSet {
    fn default() -> Self {
        Self { persistent: Default::default(), temp: Default::default() }
    }
}

pub struct Graph {
    nodes: Vec<AttrSet>,
    edges: HashMap<(usize, usize), AttrSet>,
    adj: Vec<Vec<usize>>
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            adj: vec![]
        }
    }

    fn add_node(&mut self, attrs: BTreeMap<String, Attr>) {
        self.adj.push(vec![]);
        self.nodes.push(attrs);
        return
    }

    fn add_edge(&mut self, from: usize, to: usize, attrs: BTreeMap<String, Attr>) {
        let edge_ind = self.edges.len();
        self.edges.push(attrs);
        self.adj[from].push(Connection { n: to, e: edge_ind})
    }

    fn get_node_attrs(&self, ind: usize) -> Option<&BTreeMap<String, Attr>> {
        return self.nodes.get(ind)
    }

    fn get_connections(&self, node_ind: usize) -> Option<&Vec<Connection>> {
        return self.adj.get(node_ind)
    }
}

impl Default for Graph {
    fn default() -> Self {
        let mut graph = Graph::new();

        //Nodes
        graph.add_node(BTreeMap::from([("hej".to_owned(), Attr::Num(5))]));
        graph.add_node(BTreeMap::from([("hejdå".to_owned(), Attr::Pos{x: 5, y:3})]));

        //Edges
        graph.add_edge(0, 1, BTreeMap::from([("kantinf".to_owned(), Attr::Color(2, 3, 4))]));


        graph

    }
}
