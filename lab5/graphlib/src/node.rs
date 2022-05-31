#[derive(Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node {
    label: String,
}

impl Node {
    pub fn new(label: String) -> Self {
        Self { label }
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}
