use crate::{
    repr::{adjacency_list::AdjacencyList, GraphRepr},
    Edge, Node,
};
use delegate::delegate;

pub struct Graph {
    repr: Box<dyn GraphRepr + Send>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            repr: Box::new(AdjacencyList::default()),
        }
    }
}

impl Graph {
    delegate! {
        to self.repr {
            pub fn add_node(&mut self, node: Node);
            pub fn add_edge(&mut self, from: &str, to: &str, edge: Edge);
            pub fn node_list(&self) -> Vec<&Node>;
            pub fn edge_list(&self) -> Vec<&Edge>;
        }
    }

    pub fn order(&self) -> usize {
        self.node_list().len()
    }

    pub fn size(&self) -> usize {
        self.edge_list().len()
    }

    pub fn is_empty(&self) -> bool {
        self.order() == 0
    }
}
