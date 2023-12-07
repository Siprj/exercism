
pub mod graph {
    use std::collections::HashMap;
    use graph_items::edge::*;
    use graph_items::node::*;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: maplit::hashmap! {},
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs.extend(attrs.iter().map(|(s1, s2)| (s1.to_string(), s2.to_string())));
                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|v| v.as_ref())
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Node {
                    Self {
                        name: name.to_string(),
                        attrs: maplit::hashmap! {},
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs.extend(attrs.iter().map(|(s1, s2)| (s1.to_string(), s2.to_string())));
                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|v| v.as_ref())
                }
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Graph {
        pub edges: Vec<Edge>,
        pub nodes: Vec<Node>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs.extend(attrs.iter().map(|(s1, s2)| (s1.to_string(), s2.to_string())));
            self
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == name)
        }

        pub fn attr(&self, name: &str) -> Option<&str> {
            self.attrs.get(name).map(|v| v.as_ref())
        }
    }
}
