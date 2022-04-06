use super::{max::MaxAggr, min::MinAggr, Aggregation};

#[derive(Default)]
pub struct RangeAggr {
    min: MinAggr,
    max: MaxAggr,
}

impl Aggregation for RangeAggr {
    fn consume(&mut self, element: f64) {
        self.min.consume(element);
        self.max.consume(element);
    }

    fn result(&self) -> Option<f64> {
        match (self.min.result(), self.max.result()) {
            (Some(min), Some(max)) => Some(max - min),
            _ => None,
        }
    }
}
