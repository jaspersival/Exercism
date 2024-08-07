pub mod graph {
    #[derive(PartialEq, Debug)]
    pub struct Graph<'a> {
        pub nodes: &'a Vec<graph_items::node::Node<'a>>,
        pub edges: &'a Vec<graph_items::edge::Edge<'a>>,
        pub attrs: Vec<(&'a str, &'a str)>
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            return Graph {nodes: &vec![], edges: &vec![], attrs: vec![]}
        }
        pub fn with_nodes(mut self, nodes: &Vec<graph_items::node::Node<'a>>) -> Self {
            self.nodes.extend(nodes);
            self
        }
        pub fn with_edges(mut self, edges: &Vec<graph_items::edge::Edge<'a>>) -> Self {
            self.edges.extend(edges);
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            self.attrs.extend_from_slice(attrs);
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
                attrs: Vec<(&'a str, &'a str)>
            }

            impl<'a> Edge<'a> {
                pub fn new(from: &'a str, to: &'a str) -> Self {
                    return Edge {from_to: (from, to), attrs: vec![]}
                }
                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self{
                    self.attrs.extend(attrs);
                    self
                }
            }
        }
    }

}
