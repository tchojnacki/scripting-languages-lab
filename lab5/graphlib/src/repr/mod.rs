use crate::{Edge, Node};
pub mod adjacency_list;

pub trait GraphRepr {
    fn add_node(&mut self, node: Node);
    fn add_edge(&mut self, from: &str, to: &str, edge: Edge);
    fn node_list(&self) -> Vec<&Node>;
    fn edge_list(&self) -> Vec<&Edge>;
}
