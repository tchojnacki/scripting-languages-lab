use graphlib::{Edge, Graph, Node};
use pyo3::prelude::*;

/// A class representing a graph.
#[pyclass(name = "Graph")]
pub struct PyGraph(Box<dyn Graph + Send>);

#[pymethods]
impl PyGraph {
    #[new]
    fn new() -> Self {
        PyGraph(<dyn Graph>::default())
    }

    pub fn add_node(&mut self, py: Python, node: PyObject) -> PyResult<Option<PyObject>> {
        let PyNode(node) = node.extract(py)?;
        Ok(self.0.add_node(node).map(|n| PyNode(n.clone()).into_py(py)))
    }

    pub fn create_node(&mut self, py: Python, label: &str) -> PyResult<Option<PyObject>> {
        Ok(self
            .0
            .create_node(label)
            .map(|n| PyNode(n.clone()).into_py(py)))
    }

    pub fn add_edge(
        &mut self,
        py: Python,
        from: &str,
        to: &str,
        edge: PyObject,
    ) -> PyResult<Option<PyObject>> {
        let PyEdge(edge) = edge.extract(py)?;
        Ok(self
            .0
            .add_edge(from, to, edge)
            .map(|e| PyEdge(e.clone()).into_py(py)))
    }

    pub fn create_edge(
        &mut self,
        py: Python,
        from: &str,
        to: &str,
        weight: f64,
    ) -> PyResult<Option<PyObject>> {
        Ok(self
            .0
            .create_edge(from, to, weight)
            .map(|e| PyEdge(e.clone()).into_py(py)))
    }

    #[getter]
    fn node_list(&self, py: Python) -> PyResult<Vec<PyObject>> {
        Ok(self
            .0
            .node_list()
            .iter()
            .map(|&n| PyNode(n.clone()).into_py(py))
            .collect())
    }

    #[getter]
    fn edge_list(&self, py: Python) -> PyResult<Vec<PyObject>> {
        Ok(self
            .0
            .edge_list()
            .iter()
            .map(|&e| PyEdge(e.clone()).into_py(py))
            .collect())
    }

    #[getter]
    fn order(&self) -> PyResult<usize> {
        Ok(self.0.order())
    }

    #[getter]
    fn size(&self) -> PyResult<usize> {
        Ok(self.0.size())
    }

    #[getter]
    fn is_empty(&self) -> PyResult<bool> {
        Ok(self.0.is_empty())
    }
}

/// A class representing a node.
#[pyclass(name = "Node")]
#[derive(Clone)]
pub struct PyNode(Node);

#[pymethods]
impl PyNode {
    #[new]
    fn new(label: String) -> Self {
        Self(Node::new(label))
    }

    #[getter]
    pub fn label(&self) -> PyResult<&str> {
        Ok(self.0.label())
    }
}

/// A class representing an edge.
#[pyclass(name = "Edge")]
#[derive(Clone)]
pub struct PyEdge(Edge);

#[pymethods]
impl PyEdge {
    #[new]
    fn new(weight: f64) -> Self {
        Self(Edge::new(weight))
    }

    #[getter]
    pub fn weight(&self) -> PyResult<f64> {
        Ok(self.0.weight())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn graphlibpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGraph>()?;
    m.add_class::<PyNode>()?;
    m.add_class::<PyEdge>()?;
    Ok(())
}
