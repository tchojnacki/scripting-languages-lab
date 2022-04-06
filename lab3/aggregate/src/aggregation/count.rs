use super::Aggregation;

#[derive(Default)]
pub struct CountAggr {
    count: u64,
}

impl Aggregation for CountAggr {
    fn consume(&mut self, _element: f64) {
        self.count += 1;
    }

    fn result(&self) -> Option<f64> {
        Some(self.count as f64)
    }
}
