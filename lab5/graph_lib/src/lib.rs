use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn graph_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    /// Formats the sum of two numbers as string.
    #[pyfn(m)]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    Ok(())
}
