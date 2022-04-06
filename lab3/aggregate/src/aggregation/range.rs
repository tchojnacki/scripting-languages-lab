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
        Some(self.max.result()? - self.min.result()?)
    }
}
