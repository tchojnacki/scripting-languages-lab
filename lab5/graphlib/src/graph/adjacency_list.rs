use crate::{Edge, Graph, Node};
use std::collections::HashMap;

#[derive(Default)]
pub struct AdjacencyList {
    index_lookup: HashMap<String, usize>,
    nodes: Vec<(Node, Vec<(usize, Edge)>)>,
}

impl Graph for AdjacencyList {
    fn add_node(&mut self, node: Node) -> Option<&Node> {
        if !self.index_lookup.contains_key(node.label()) {
            let node_idx = self.nodes.len();
            self.index_lookup.insert(node.label().to_owned(), node_idx);
            self.nodes.push((node, vec![]));
            Some(&self.nodes[node_idx].0)
        } else {
            None
        }
    }

    fn add_edge(&mut self, from: &str, to: &str, edge: Edge) -> Option<&Edge> {
        if self.index_lookup.contains_key(from) && self.index_lookup.contains_key(to) {
            let neighbours = &mut self.nodes[self.index_lookup[from]].1;
            let neighbour_idx = neighbours.len();
            neighbours.push((self.index_lookup[to], edge));
            Some(&neighbours[neighbour_idx].1)
        } else {
            None
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
    use crate::{Edge, Graph, Node};

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
