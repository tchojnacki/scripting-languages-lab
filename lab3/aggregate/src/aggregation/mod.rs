mod avg;
mod count;
mod first;
mod last;
mod max;
mod min;
mod product;
mod range;
mod sum;

pub trait Aggregation {
    fn consume(&mut self, element: f64);
    fn result(&self) -> Option<f64>;
}

impl dyn Aggregation {
    pub fn from_string(s: &str) -> Box<dyn Aggregation> {
        match s {
            "min" => Box::from(min::MinAggr::default()),
            "max" => Box::from(max::MaxAggr::default()),
            "sum" => Box::from(sum::SumAggr::default()),
            "avg" => Box::from(avg::AvgAggr::default()),
            "count" => Box::from(count::CountAggr::default()),
            "first" => Box::from(first::FirstAggr::default()),
            "last" => Box::from(last::LastAggr::default()),
            "range" => Box::from(range::RangeAggr::default()),
            "product" => Box::from(product::ProductAggr::default()),
            _ => panic!("No aggregation {} found!", s),
        }
    }
}
