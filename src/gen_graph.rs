
use std::collections::BTreeMap;

enum Attr {
    Num(i32),
    Str(String),
    Pos {x: i32, y: i32},
    Color(u8, u8, u8)
}

struct Connection {
    n: usize,
    e: usize,
}

struct Graph {
    nodes: Vec<BTreeMap<String, Attr>>,
    edges: Vec<BTreeMap<String, Attr>>,
    adj: Vec<Vec<Connection>>
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
    }

    fn add_edge(&mut self, from: usize, to: usize, attrs: BTreeMap<String, Attr>) {
        let edge_ind = self.edges.len();
        self.edges.push(attrs);
        self.adj[from].push(Connection { n: to, e: edge_ind})
    }
}

fn main() {
    let mut graph = Graph::new();

    //Nodes
    graph.add_node(BTreeMap::from([("hej".to_owned(), Attr::Num(5))]));
    graph.add_node(BTreeMap::from([("hejdå".to_owned(), Attr::Pos{x: 5, y:3})]));

    //Edges
    graph.add_edge(0, 1, BTreeMap::from([("kantinf".to_owned(), Attr::Color(2, 3, 4))]));
}

