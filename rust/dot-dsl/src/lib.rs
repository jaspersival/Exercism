pub mod graph {
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: Vec<(str)>
    }

    impl Graph {
        pub fn new() -> Self {
            todo!("Construct a new Graph struct.");
        }
    }

    pub mod graph_items {
        pub mod node {
            pub struct Node {
                name: str,
                attr: Vec<(str)>
            }
        }

        pub mod edge {
            pub struct Edge {
                name: str,
                attr: Vec<(str)>
            }
        }
    }

}
