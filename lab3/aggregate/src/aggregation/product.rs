use super::Aggregation;

pub struct ProductAggr {
    product: f64,
}

impl Default for ProductAggr {
    fn default() -> Self {
        ProductAggr { product: 1. }
    }
}

impl Aggregation for ProductAggr {
    fn consume(&mut self, element: f64) {
        self.product *= element;
    }

    fn result(&self) -> Option<f64> {
        Some(self.product)
    }
}
