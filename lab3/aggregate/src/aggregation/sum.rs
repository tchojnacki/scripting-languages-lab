use super::Aggregation;

#[derive(Default)]
pub struct SumAggr {
    sum: f64,
}

impl Aggregation for SumAggr {
    fn consume(&mut self, element: f64) {
        self.sum += element;
    }

    fn results(&self) -> Option<f64> {
        Some(self.sum)
    }
}
