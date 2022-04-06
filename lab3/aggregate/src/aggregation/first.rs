use super::Aggregation;

#[derive(Default)]
pub struct FirstAggr {
    first: Option<f64>,
}

impl Aggregation for FirstAggr {
    fn consume(&mut self, element: f64) {
        if self.first.is_none() {
            self.first = Some(element);
        }
    }

    fn result(&self) -> Option<f64> {
        self.first
    }
}
