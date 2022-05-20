use graphlib::Graph;
use pyo3::prelude::*;

/// A class representing a graph.
#[pyclass(name = "Graph")]
struct PyGraph(Graph);

#[pymethods]
impl PyGraph {
    #[new]
    fn new() -> Self {
        PyGraph(Graph::new())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn graphlibpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGraph>()?;
    Ok(())
}
