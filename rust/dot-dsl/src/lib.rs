pub mod graph {
    use std::collections::HashMap;
    use maplit::hashmap;
    use crate::graph::graph_items::node::Node;

    #[derive(PartialEq,Debug)]
    pub struct Graph<'a> {
        pub nodes: Vec<graph_items::node::Node<'a>>,
        pub edges: Vec<graph_items::edge::Edge<'a>>,
        pub attrs: HashMap<String, String>
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            return Graph {nodes: vec![], edges: vec![], attrs: hashmap!{}}
        }
        pub fn with_nodes(mut self, nodes: Vec<graph_items::node::Node<'a>>) -> Self {
            self.nodes = nodes;
            self
        }
        pub fn with_edges(mut self, edges: Vec<graph_items::edge::Edge<'a>>) -> Self {
            self.edges = edges;
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            for (key, value) in attrs {
                self.attrs.insert(key.to_string(), value.to_string());

            }
            self
        }
        pub fn node(self, name: &str) -> Node {
            for node in self.nodes {
                if node.name == name {
                    return node
                }
            }
            panic!("node {} must be stored", name);
        }
    }

    pub mod graph_items {
        pub mod node {
            #[derive(Debug, PartialEq)]
            pub struct Node<'a> {
                pub name: &'a str,
                pub attrs: Vec<(&'a str,&'a str)>
            }
            impl<'a> Node<'a> {
                pub fn new(name: &'a str) -> Self {
                    return Node {name, attrs: vec![]}
                }
                pub fn with_attrs(mut self,attrs: &[(&'a str, &'a str)]) -> Self {
                    self.attrs.extend_from_slice(attrs);
                    self
                }

            }
        }

        pub mod edge {
            #[derive(Debug, PartialEq)]
            pub struct Edge<'a> {
                from_to: (&'a str, &'a str),
                pub attr: Vec<(&'a str, &'a str)>
            }

            impl<'a> Edge<'a> {
                pub fn new(from: &'a str, to: &'a str) -> Self {
                    return Edge {from_to: (from, to), attr: vec![]}
                }
                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self{
                    self.attr.extend(attrs);
                    self
                }
            }
        }
    }

}
