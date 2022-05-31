use std::collections::{HashMap, HashSet};

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

    fn neighbours(&self, node: &Node) -> Vec<&Node>;

    fn get_edge(&self, from: &Node, to: &Node) -> &Edge;

    fn dijkstra<'a>(&'a self, source: &'a Node) -> HashMap<&'a Node, f64> {
        let mut distances = HashMap::<&Node, f64>::new();
        let mut previous = HashMap::<&Node, &Node>::new();
        let mut queue = HashSet::<&Node>::new();

        for v in self.node_list() {
            distances.insert(v, f64::INFINITY);
            queue.insert(v);
        }

        distances.insert(source, 0.0);

        while let Some(u) = distances
            .iter()
            .filter(|(&n, _)| queue.contains(n))
            .min_by(|(_, &d1), (_, &d2)| d1.partial_cmp(&d2).unwrap())
            .map(|(&n, _)| n)
        {
            queue.remove(u);

            for v in self.neighbours(u).iter().filter(|&v| queue.contains(v)) {
                let alt = distances[u] + self.get_edge(u, v).weight();

                if alt < distances[v] {
                    distances.insert(v, alt);
                    previous.insert(v, u);
                }
            }
        }

        distances
    }
}

impl dyn Graph {
    pub fn default() -> Box<dyn Graph + Send> {
        Box::new(AdjacencyList::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Graph, Node};

    #[test]
    fn finds_shortest_path_with_dijkstra() {
        let mut g = <dyn Graph>::default();

        g.create_node("a");
        g.create_node("b");
        g.create_node("c");
        g.create_node("d");

        g.create_edge("a", "b", 15.0);
        g.create_edge("b", "c", 3.0);
        g.create_edge("c", "d", 6.0);
        g.create_edge("b", "d", 15.0);

        let source = &Node::new("a".to_string());
        let results = g.dijkstra(source);
        assert!(results[&Node::new("d".to_string())] == 24.0)
    }
}
