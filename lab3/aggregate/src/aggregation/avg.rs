use super::{count::CountAggr, sum::SumAggr, Aggregation};

#[derive(Default)]
pub struct AvgAggr {
    sum: SumAggr,
    count: CountAggr,
}

impl Aggregation for AvgAggr {
    fn consume(&mut self, element: f64) {
        self.sum.consume(element);
        self.count.consume(element);
    }

    fn result(&self) -> Option<f64> {
        match (self.sum.result(), self.count.result()) {
            (Some(sum), Some(count)) => Some(sum / count),
            _ => None,
        }
    }
}
