use std::fmt;

use algo::{multiply_single, Matrix};
use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyResult};

#[pyclass(name = "Matrix")]
pub struct PyMatrix {
    inner: Matrix<f64>,
}

#[pymethods]
impl PyMatrix {
    #[new]
    pub fn try_new(data: Vec<Vec<f64>>) -> PyResult<Self> {
        if data.is_empty() || data[0].is_empty() {
            return Err(PyValueError::new_err(
                "Matrix must have at least one row and one column",
            ));
        }

        let row = data.len();
        let col = data[0].len();
        let data: Vec<_> = data.into_iter().flatten().collect();

        Ok(Self {
            inner: Matrix::new(data, row, col),
        })
    }

    pub fn mul(&self, other: &PyMatrix) -> PyResult<Self> {
        let result = multiply_single(&self.inner, &other.inner).unwrap();
        Ok(Self { inner: result })
    }

    pub fn multiply(&self, other: Vec<Vec<f64>>) -> PyResult<Self> {
        let other = PyMatrix::try_new(other)?;
        self.mul(&other)
    }

    pub fn __repr__(&self) -> String {
        self.to_string()
    }

    pub fn __str__(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for PyMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Matrix: {}>", self.inner)
    }
}
