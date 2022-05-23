use crate::{Edge, Node};

use self::adjacency_list::AdjacencyList;
pub mod adjacency_list;

pub trait Graph {
    fn add_node(&mut self, node: Node) -> Option<&Node>;

    fn create_node(&mut self, label: &str) -> Option<&Node> {
        self.add_node(Node::new(label.to_string()))
    }

    fn add_edge(&mut self, from: &str, to: &str, edge: Edge) -> Option<&Edge>;

    fn create_edge(&mut self, from: &str, to: &str, weight: f64) -> Option<&Edge> {
        self.add_edge(from, to, Edge::new(weight))
    }

    fn node_list(&self) -> Vec<&Node>;

    fn edge_list(&self) -> Vec<&Edge>;

    fn order(&self) -> usize {
        self.node_list().len()
    }

    fn size(&self) -> usize {
        self.edge_list().len()
    }

    fn is_empty(&self) -> bool {
        self.order() == 0
    }
}

impl dyn Graph {
    pub fn default() -> Box<dyn Graph + Send> {
        Box::new(AdjacencyList::default())
    }
}
