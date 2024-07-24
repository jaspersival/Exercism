pub mod graph {
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: Vec<(String)>
    }

    impl Graph {
        pub fn new() -> Self {
            return Graph {nodes: vec![], edges: vec![], attrs: vec![]}
        }
    }

    pub mod graph_items {
        pub mod node {
            pub struct Node {
                name: String,
                attr: Vec<(String)>
            }
            impl Node {
                pub fn new(name: String) -> Self {
                    return Node {name, attr: vec![]}
                }
            }
        }

        pub mod edge {
            pub struct Edge {
                from_to: (String, String),
                attr: Vec<(String)>
            }

            impl Edge {
                pub fn new(from: String, to: String) -> Self {
                    return Edge {from_to: (from, to), attr: vec![]}
                }
            }
        }
    }

}
