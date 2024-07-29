pub mod graph {
    pub struct Graph<'a> {
        pub nodes: Vec<graph_items::node::Node<'a>>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: Vec<(String)>
    }

    impl Graph {
        pub fn new() -> Self {
            return Graph {nodes: vec![], edges: vec![], attrs: vec![]}
        }
        pub fn with_nodes(mut self, nodes: &Vec<graph_items::node::Node>) -> Self {
            self.nodes.extend(nodes);
            self
        }
    }

    pub mod graph_items {
        pub mod node {
            #[derive(Debug, PartialEq)]
            pub struct Node<'a> {
                name: &'a str,
                attrs: Vec<(&'a str,&'a str)>
            }
            impl<'a> Node<'a> {
                pub fn new(name: &str) -> Self {
                    return Node {name, attrs: vec![]}
                }
                pub fn with_attrs(mut self,attrs: &[(&'a str, &'a str)]) -> Self {
                    self.attrs.extend_from_slice(attrs);
                    self
                }

            }
        }

        pub mod edge {
            pub struct Edge {
                from_to: (String, String),
                attrs: Vec<(String, String)>
            }

            impl Edge {
                pub fn new(from: String, to: String) -> Self {
                    return Edge {from_to: (from, to), attrs: vec![]}
                }
            }
        }
    }

}
