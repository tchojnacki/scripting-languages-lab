use crate::{
    repr::{adjacency_list::AdjacencyList, GraphRepr},
    Edge, Node,
};
use bimap::BiMap;
use delegate::delegate;
use std::collections::HashMap;

pub struct Graph {
    repr: Box<dyn GraphRepr + Send>,
    labels: BiMap<String, usize>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            repr: Box::new(AdjacencyList::default()),
            labels: BiMap::default(),
        }
    }
}

impl Graph {
    pub fn add_node(&mut self, label: &str) -> Node {
        let idx = self.repr.add_node();
        self.labels.insert(label.to_string(), idx);
        Node::from(label)
    }

    pub fn add_edge(&mut self, from: &str, to: &str, weight: f64) -> Option<Edge> {
        if from == to {
            return None;
        }

        self.repr.add_edge(
            *self.labels.get_by_left(from)?,
            *self.labels.get_by_left(to)?,
            weight,
        );
        Some(Edge::new(weight))
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.repr
            .nodes()
            .iter()
            .map(|idx| Node::from(self.labels.get_by_right(idx).unwrap()))
            .collect()
    }

    pub fn edges(&self) -> Vec<(&str, &str, Edge)> {
        self.repr
            .edges()
            .iter()
            .map(|&(from, to, weight)| {
                (
                    self.labels.get_by_right(&from).unwrap().as_str(),
                    self.labels.get_by_right(&to).unwrap().as_str(),
                    Edge::new(weight),
                )
            })
            .collect()
    }

    pub fn neighbours(&self, node: &str) -> Option<Vec<Node>> {
        Some(
            self.repr
                .neighbours(*self.labels.get_by_left(node)?)
                .iter()
                .map(|idx| Node::from(self.labels.get_by_right(idx).unwrap()))
                .collect(),
        )
    }

    pub fn get_edge(&self, from: &str, to: &str) -> Option<Edge> {
        self.repr
            .get_edge(
                *self.labels.get_by_left(from)?,
                *self.labels.get_by_left(to)?,
            )
            .map(Edge::new)
    }

    pub fn dijkstra(&self, source: &str) -> Option<HashMap<String, (f64, Vec<Node>)>> {
        Some(HashMap::from_iter(
            self.repr
                .dijkstra(*self.labels.get_by_left(source)?)
                .iter()
                .enumerate()
                .map(|(to, (dist, path))| {
                    (
                        self.labels.get_by_right(&to).unwrap().to_owned(),
                        (
                            *dist,
                            path.iter()
                                .map(|idx| Node::from(self.labels.get_by_right(idx).unwrap()))
                                .collect(),
                        ),
                    )
                }),
        ))
    }

    delegate! {
        to self.repr {
            pub fn order(&self) -> usize;
            pub fn size(&self) -> usize;
            pub fn is_empty(&self) -> bool;
        }
    }
}
