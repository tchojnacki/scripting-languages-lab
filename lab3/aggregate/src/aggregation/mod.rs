mod min;
mod sum;

pub trait Aggregation {
    fn consume(&mut self, element: f64);
    fn results(&self) -> Option<f64>;
}

impl dyn Aggregation {
    pub fn from_string(s: &str) -> Box<dyn Aggregation> {
        match s {
            "min" => Box::from(min::MinAggr::default()),
            "sum" => Box::from(sum::SumAggr::default()),
            _ => unreachable!(),
        }
    }
}
