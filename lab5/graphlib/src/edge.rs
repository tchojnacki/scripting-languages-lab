#[derive(Default)]
pub struct Edge {
    label: Option<String>,
    weight: f64,
}

impl Edge {
    pub fn new(weight: f64) -> Self {
        Self {
            label: None,
            weight,
        }
    }

    pub fn with_label(label: String, weight: f64) -> Self {
        Self {
            label: Some(label),
            weight,
        }
    }

    pub fn label(&self) -> &Option<String> {
        &self.label
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }
}
