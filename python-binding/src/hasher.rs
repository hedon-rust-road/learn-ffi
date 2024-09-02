use core::fmt;

use algo::Algo;
use pyo3::prelude::*;

#[pyclass(name = "Algo")]
pub struct PyAlgo {
    inner: Algo,
}

#[pymethods]
impl PyAlgo {
    #[new]
    #[pyo3(signature = (name=""))]
    pub fn new(name: &str) -> Self {
        let algo = match name {
            "blake3" => Algo::new(algo::AlgoType::Blake3),
            _ => Algo::new(algo::AlgoType::Default),
        };
        Self { inner: algo }
    }

    pub fn hash(&self, v: &str) -> String {
        self.inner.hash(v).to_string()
    }

    pub fn get_name(&self) -> String {
        self.inner.get_name().to_string()
    }

    // For `__repr__` we want to return a string that Python code could use to recreate.
    fn __repr__(&self) -> String {
        self.to_string()
    }

    // `__str__` is generally used to create an "informal" representation.
    fn __str__(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for PyAlgo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Algo:{}>", self.inner.get_name())
    }
}
