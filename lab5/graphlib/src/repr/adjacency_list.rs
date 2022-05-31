use super::GraphRepr;

#[derive(Default)]
pub struct AdjacencyList {
    adj_list: Vec<Vec<(usize, f64)>>,
}

impl GraphRepr for AdjacencyList {
    fn add_node(&mut self) -> usize {
        self.adj_list.push(Vec::new());
        self.adj_list.len() - 1
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        if let Some(neighbours) = self.adj_list.get_mut(from) {
            if !neighbours.iter().any(|&(n, _)| n == to) && from != to {
                neighbours.push((to, weight));
            }
        }
    }

    fn nodes(&self) -> Vec<usize> {
        self.adj_list
            .iter()
            .enumerate()
            .map(|(index, _)| index)
            .collect()
    }

    fn edges(&self) -> Vec<(usize, usize, f64)> {
        self.adj_list
            .iter()
            .enumerate()
            .flat_map(|(from, neighbours)| {
                neighbours
                    .iter()
                    .map(move |&(to, weight)| (from, to, weight))
            })
            .collect()
    }

    fn neighbours(&self, node: usize) -> Vec<usize> {
        self.adj_list
            .get(node)
            .map(|neighbours| neighbours.iter().map(|&(to, _)| to).collect())
            .unwrap_or_default()
    }

    fn get_edge(&self, from: usize, to: usize) -> Option<f64> {
        self.adj_list.get(from).and_then(|neighbours| {
            neighbours
                .iter()
                .find(|(other, _)| *other == to)
                .map(|&(_, weight)| weight)
        })
    }
}
