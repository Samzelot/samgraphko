
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
    fn add_node(&mut self, attrs: Attrs) {
        self.adj.push(Default::default());
        self.nodes.push(attrs);
        return
    }

    fn add_edge(&mut self, from: usize, to: usize, attrs: Attrs) {
        self.adj[from].push(to);
        self.edges.insert((from, to), attrs);
    }

    fn get_node_attrs(&self, ind: usize) -> Option<&Attrs> {
        return self.nodes.get(ind)
    }

    fn get_connections(&self, node_ind: usize) -> Option<&Vec<usize>> {
        return self.adj.get(node_ind)
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self { nodes: Default::default(), edges: Default::default(), adj: Default::default() }
    }
}

fn test_graph() -> Graph {
    let mut graph = Graph::default();

    //Nodes
    graph.add_node([("hej".to_owned(), Attr::Num(5))].into());
    graph.add_node([("hejdå".to_owned(), Attr::Pos{x: 5, y:3})].into());

    //Edges
    graph.add_edge(0, 1, [("kantinf".to_owned(), Attr::Color(2, 3, 4))].into());


    graph

}
