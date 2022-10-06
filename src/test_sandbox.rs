use crate::gen_graph::{Graph, TempAttrs};

// TODO: Set up unit tests
struct TestAttrs {
    num: i32,
    pair: (i32, i32)
}

impl Default for TestAttrs {
    fn default() -> Self {
        Self {
            num: 0,
            pair: (0, 0)
        }
    }
}

pub fn test_attrs() {
    let mut graph = Graph::example();
    let mut test_attrs: TempAttrs<TestAttrs> = graph.gen_attrs();

    let ind = 0;
    test_attrs.borrow_mut(&ind).num = 37;
    test_attrs.borrow_mut(&ind).pair = (-2, 42);

    println!("Cheap man's assert(37 = {} && (-2, 42) == {:?})", 
        test_attrs.borrow(&ind).num, test_attrs.borrow(&ind).pair);
    
}