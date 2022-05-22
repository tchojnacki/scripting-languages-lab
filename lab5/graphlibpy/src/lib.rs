use graphlib::{Graph, Node};
use pyo3::prelude::*;

/// A class representing a graph.
#[pyclass(name = "Graph")]
pub struct PyGraph(Graph);

#[pymethods]
impl PyGraph {
    #[new]
    fn new() -> Self {
        PyGraph(Graph::default())
    }

    pub fn add_node(&mut self, node: PyRef<PyNode>) -> PyResult<()> {
        self.0.add_node(node.0.clone());
        Ok(())
    }
}

/// A class representing a node.
#[pyclass(name = "Node")]
pub struct PyNode(Node);

#[pymethods]
impl PyNode {
    #[new]
    fn new(label: String) -> Self {
        PyNode(Node::new(label))
    }

    #[getter]
    pub fn label(&self) -> PyResult<&str> {
        Ok(self.0.label())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn graphlibpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGraph>()?;
    m.add_class::<PyNode>()?;
    Ok(())
}
