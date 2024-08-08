pub mod graph {
    use std::collections::HashMap;

    #[derive(PartialEq, Debug)]
    pub struct Graph<'a> {
        pub nodes: Vec<graph_items::node::Node<'a>>,
        pub edges: Vec<graph_items::edge::Edge<'a>>,
        pub attrs: HashMap<String, String>
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            return Graph {nodes: Vec::new(), edges: Vec::new(), attrs: HashMap::new() }
        }
        pub fn with_nodes(mut self, nodes: &Vec<graph_items::node::Node<'a>>) -> Self {
            self.nodes.extend(nodes.clone());
            self
        }
        pub fn with_edges(mut self, edges: &Vec<graph_items::edge::Edge<'a>>) -> Self {
            self.edges.extend(edges.clone());
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }
        pub fn node(&self, name: &'a str) -> Option<& graph_items::node::Node> {
            for node in &self.nodes {
                if node.name == name {
                    return Some(node)
                }
            }
            return None
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node<'a> {
                pub name: &'a str,
                attrs: HashMap<String, String>
            }
            impl<'a> Node<'a> {
                pub fn new(name: &'a str) -> Self {
                    return Node {name, attrs: HashMap::new()}
                }
                pub fn with_attrs(mut self,attrs: &[(&'a str, &'a str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }
                pub fn attr(&self, k: &str) -> Option<&str> {
                    return self.attrs.get(k).map(|v|v.as_str())
                }

            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge<'a> {
                from_to: (&'a str, &'a str),
                attrs: HashMap<String, String>
            }

            impl<'a> Edge<'a> {
                pub fn new(from: &'a str, to: &'a str) -> Self {
                    return Edge {from_to: (from, to), attrs: HashMap::new()}
                }
                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self{
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }
                pub fn attr(&self, k: &str) -> Option<&str> {
                    return self.attrs.get(k).map(|v|v.as_str())
                }
            }
        }
    }

}
