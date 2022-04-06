use super::Aggregation;

#[derive(Default)]
pub struct LastAggr {
    last: Option<f64>,
}

impl Aggregation for LastAggr {
    fn consume(&mut self, element: f64) {
        self.last = Some(element);
    }

    fn result(&self) -> Option<f64> {
        self.last
    }
}
