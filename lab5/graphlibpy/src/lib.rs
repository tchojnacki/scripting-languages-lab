#![allow(clippy::type_complexity)]

use graphlib::{Edge, Graph, Node};
use pyo3::prelude::*;
use std::collections::HashMap;

/// A class representing a graph.
#[pyclass(name = "Graph")]
pub struct PyGraph(Graph);

#[pymethods]
impl PyGraph {
    #[new]
    fn new() -> Self {
        PyGraph(Graph::default())
    }

    fn add_node(&mut self, label: &str) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add_node(label)))
    }

    fn add_edge(&mut self, from: &str, to: &str, weight: f64) -> PyResult<Option<PyEdge>> {
        Ok(self.0.add_edge(from, to, weight).map(PyEdge))
    }

    #[getter]
    fn nodes(&self) -> PyResult<Vec<PyNode>> {
        Ok(self.0.nodes().iter().map(|n| PyNode(n.clone())).collect())
    }

    #[getter]
    fn edges(&self) -> PyResult<Vec<(&str, &str, PyEdge)>> {
        Ok(self
            .0
            .edges()
            .iter()
            .map(|&(from, to, e)| (from, to, PyEdge(e)))
            .collect())
    }

    fn neighbours(&self, node: &str) -> PyResult<Option<Vec<PyNode>>> {
        Ok(self
            .0
            .neighbours(node)
            .map(|v| v.iter().map(|n| PyNode(n.clone())).collect()))
    }

    fn get_edge(&self, from: &str, to: &str) -> PyResult<Option<PyEdge>> {
        Ok(self.0.get_edge(from, to).map(PyEdge))
    }

    fn dijkstra(&self, source: &str) -> PyResult<Option<HashMap<String, (f64, Vec<PyNode>)>>> {
        Ok(self.0.dijkstra(source).map(|m| {
            m.iter()
                .map(|(s, (f, v))| {
                    (
                        s.to_owned(),
                        (*f, v.iter().map(|n| PyNode(n.clone())).collect()),
                    )
                })
                .collect()
        }))
    }

    fn bfs(&self, source: &str) -> PyResult<Option<Vec<PyNode>>> {
        Ok(self
            .0
            .bfs(source)
            .map(|v| v.iter().map(|n| PyNode(n.clone())).collect()))
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
    fn label(&self) -> PyResult<&str> {
        Ok(self.0.label())
    }

    fn __repr__(&self) -> String {
        format!("Node('{}')", self.0.label())
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
    fn weight(&self) -> PyResult<f64> {
        Ok(self.0.weight())
    }

    fn __repr__(&self) -> String {
        format!("Edge({})", self.0.weight())
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
