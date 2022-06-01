use std::collections::{HashSet, VecDeque};

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

    fn dijkstra(&self, source: usize) -> Vec<(f64, Vec<usize>)> {
        let mut distances = vec![f64::INFINITY; self.order()];
        let mut previous = vec![None; self.order()];
        let mut queue = HashSet::<_>::from_iter(self.nodes().into_iter());

        distances[source] = 0.0;

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

        let get_path = |to: usize| {
            if source == to || previous[to].is_none() {
                Vec::new()
            } else {
                let mut path = Vec::<usize>::new();

                let mut u = Some(to);
                while let Some(v) = u {
                    path.insert(0, v);
                    u = previous[v];
                }

                path
            }
        };

        distances
            .iter()
            .enumerate()
            .map(|(to, dist)| (*dist, get_path(to)))
            .collect()
    }

    fn bfs(&self, source: usize) -> Vec<usize> {
        let mut visited = vec![false; self.order()];
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back(source);
        visited[source] = true;

        while let Some(s) = queue.pop_front() {
            result.push(s);
            for node in self.neighbours(s) {
                if !visited[node] {
                    queue.push_back(node);
                    visited[node] = true;
                }
            }
        }

        result
    }

    fn dfs(&self, source: usize) -> Vec<usize> {
        let mut visited = vec![false; self.order()];
        let mut stack = VecDeque::new();
        let mut result = Vec::new();

        stack.push_front(source);

        while let Some(s) = stack.pop_front() {
            if !visited[s] {
                result.push(s);
                visited[s] = true;
            }

            for node in self.neighbours(s) {
                if !visited[node] {
                    stack.push_front(node);
                }
            }
        }

        result
    }
}
