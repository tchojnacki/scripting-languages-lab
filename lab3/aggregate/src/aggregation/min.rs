use super::Aggregation;

#[derive(Default)]
pub struct MinAggr {
    min: Option<f64>,
}

impl Aggregation for MinAggr {
    fn consume(&mut self, element: f64) {
        if self.min.is_none() || &element < self.min.as_ref().unwrap() {
            self.min = Some(element);
        }
    }

    fn result(&self) -> Option<f64> {
        self.min
    }
}
