use crate::gen_graph::{Node, Graph, attrs::NodeAttrs};


//TODO: Set up unit tests
struct TestNodeAttrs {
    num: i32,
    pair: (i32, i32)
}

impl Default for TestNodeAttrs {
    fn default() -> Self {
        Self {
            num: 0,
            pair: (0, 0)
        }
    }
}

pub fn test_attrs() {
    let graph = Graph::example();
    let mut test_attrs: NodeAttrs<TestNodeAttrs> = graph.gen_node_attrs();

    let usize_var: usize = 5;
    let _node: Node = usize_var;

    let ind = 0;
    test_attrs.get_mut(&ind).num = 37;
    test_attrs.get_mut(&ind).pair = (-2, 42);

    println!("Cheap man's assert(37 = {} && (-2, 42) == {:?})", 
        test_attrs.get(&ind).num, test_attrs.get(&ind).pair);
    
}