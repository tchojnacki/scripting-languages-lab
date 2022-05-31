use std::collections::HashSet;

pub mod adjacency_list;

pub trait GraphRepr {
    fn add_node(&mut self) -> usize;
    fn add_edge(&mut self, from: usize, to: usize, weight: f64);
    fn nodes(&self) -> Vec<usize>;
    fn edges(&self) -> Vec<(usize, usize, f64)>;
    fn neighbours(&self, node: usize) -> Vec<usize>;
    fn get_edge(&self, from: usize, to: usize) -> Option<f64>;

    fn order(&self) -> usize {
        self.nodes().len()
    }

    fn size(&self) -> usize {
        self.edges().len()
    }

    fn is_empty(&self) -> bool {
        self.nodes().is_empty()
    }

    fn dijkstra(&self, source: usize) -> Vec<f64> {
        let mut distances = Vec::from_iter(self.nodes().iter().map(|_| f64::INFINITY));
        let mut previous = Vec::from_iter(self.nodes().iter().map(|_| None));
        let mut queue = HashSet::<_>::from_iter(self.nodes().into_iter());

        distances.insert(source, 0.0);

        while let Some(u) = distances
            .iter()
            .enumerate()
            .filter(|(n, _)| queue.contains(n))
            .min_by(|(_, &d1), (_, &d2)| d1.partial_cmp(&d2).unwrap())
            .map(|(n, _)| n)
        {
            queue.remove(&u);

            for &v in self.neighbours(u).iter().filter(|&v| queue.contains(v)) {
                let alt = distances[u] + self.get_edge(u, v).unwrap();

                if alt < distances[v] {
                    distances[v] = alt;
                    previous[v] = Some(u);
                }
            }
        }

        distances
    }
}
