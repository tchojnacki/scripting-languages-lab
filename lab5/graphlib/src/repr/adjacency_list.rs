use super::GraphRepr;
use crate::{Edge, Node};
use std::collections::HashMap;

#[derive(Default)]
pub struct AdjacencyList {
    index_lookup: HashMap<String, usize>,
    nodes: Vec<(Node, Vec<(usize, Edge)>)>,
}

impl GraphRepr for AdjacencyList {
    fn add_node(&mut self, node: Node) {
        if !self.index_lookup.contains_key(node.label()) {
            self.index_lookup
                .insert(node.label().to_owned(), self.nodes.len());
            self.nodes.push((node, vec![]));
        }
    }

    fn add_edge(&mut self, from: &str, to: &str, edge: Edge) {
        if self.index_lookup.contains_key(from) && self.index_lookup.contains_key(to) {
            self.nodes[self.index_lookup[from]]
                .1
                .push((self.index_lookup[to], edge));
        }
    }

    fn node_list(&self) -> Vec<&Node> {
        self.nodes.iter().map(|(n, _)| n).collect()
    }

    fn edge_list(&self) -> Vec<&Edge> {
        self.nodes
            .iter()
            .flat_map(|(_, l)| l)
            .map(|(_, e)| e)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::AdjacencyList;
    use crate::{repr::GraphRepr, Edge, Node};

    #[test]
    fn it_adds_node() {
        let mut a = AdjacencyList::default();

        a.add_node(Node::new("a".to_string()));
        a.add_node(Node::new("b".to_string()));

        assert_eq!(a.node_list().len(), 2);
    }

    #[test]
    fn it_adds_edges() {
        let mut a = AdjacencyList::default();

        a.add_node(Node::new("a".to_string()));
        a.add_node(Node::new("b".to_string()));
        a.add_edge("a", "b", Edge::default());

        assert_eq!(a.edge_list().len(), 2);
    }
}
