mod hasher;
mod matrix;

use hasher::PyAlgo;
use matrix::PyMatrix;
use pyo3::prelude::*;

/// Prints a message.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from python algo!".into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_class::<PyAlgo>()?;
    m.add_class::<PyMatrix>()?;
    Ok(())
}
