use super::GraphRepr;

#[derive(Default)]
pub struct EdgeList {
    node_count: usize,
    edge_list: Vec<(usize, usize, f64)>,
}

impl GraphRepr for EdgeList {
    fn add_node(&mut self) -> usize {
        self.node_count += 1;
        self.node_count - 1
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        if from < self.node_count && to < self.node_count {
            self.edge_list.push((from, to, weight));
        }
    }

    fn nodes(&self) -> Vec<usize> {
        (0..self.node_count).collect()
    }

    fn edges(&self) -> Vec<(usize, usize, f64)> {
        self.edge_list.clone()
    }

    fn neighbours(&self, node: usize) -> Vec<usize> {
        self.edge_list
            .iter()
            .filter_map(|&(from, to, _)| if from == node { Some(to) } else { None })
            .collect()
    }

    fn get_edge(&self, from: usize, to: usize) -> Option<f64> {
        self.edge_list.iter().find_map(|&(f, t, weight)| {
            if f == from && t == to {
                Some(weight)
            } else {
                None
            }
        })
    }
}
