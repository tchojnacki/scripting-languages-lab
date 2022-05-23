#[derive(Default, Clone)]
pub struct Edge {
    weight: f64,
}

impl Edge {
    pub fn new(weight: f64) -> Self {
        Self { weight }
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }
}
