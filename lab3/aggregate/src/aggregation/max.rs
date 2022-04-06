use super::Aggregation;

#[derive(Default)]
pub struct MaxAggr {
    max: Option<f64>,
}

impl Aggregation for MaxAggr {
    fn consume(&mut self, element: f64) {
        if self.max.is_none() || &element > self.max.as_ref().unwrap() {
            self.max = Some(element);
        }
    }

    fn result(&self) -> Option<f64> {
        self.max
    }
}
