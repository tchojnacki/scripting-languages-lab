use super::Aggregation;

#[derive(Default)]
pub struct MaxAggr {
    current: Option<f64>,
}

impl Aggregation for MaxAggr {
    fn consume(&mut self, element: f64) {
        if self.current.is_none() || &element > self.current.as_ref().unwrap() {
            self.current = Some(element);
        }
    }

    fn result(&self) -> Option<f64> {
        self.current
    }
}
