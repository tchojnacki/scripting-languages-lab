use super::Aggregation;

#[derive(Default)]
pub struct MinAggr {
    current: Option<f64>,
}

impl Aggregation for MinAggr {
    fn consume(&mut self, element: f64) {
        if self.current.is_none() || &element < self.current.as_ref().unwrap() {
            self.current = Some(element);
        }
    }

    fn results(&self) -> Option<f64> {
        self.current
    }
}
