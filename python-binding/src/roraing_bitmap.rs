use core::fmt;

use pyo3::{pyclass, pymethods};
use roaring::RoaringBitmap;

#[pyclass(name = "RoaringBitmap")]
pub struct PyRoaingBitmap {
    inner: RoaringBitmap,
}

#[pymethods]
impl PyRoaingBitmap {
    #[new]
    #[pyo3(signature = (data = vec![]))]
    pub fn new(data: Vec<u32>) -> Self {
        let mut inner = RoaringBitmap::new();
        inner.extend(data);
        Self { inner }
    }

    pub fn insert(&mut self, value: u32) -> bool {
        self.inner.insert(value)
    }

    pub fn remove(&mut self, value: u32) -> bool {
        self.inner.remove(value)
    }

    pub fn contains(&self, value: u32) -> bool {
        self.inner.contains(value)
    }

    pub fn push(&mut self, value: u32) -> bool {
        self.inner.push(value)
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.inner.is_full()
    }
    pub fn len(&self) -> u64 {
        self.inner.len()
    }

    pub fn min(&self) -> Option<u32> {
        self.inner.min()
    }

    pub fn max(&self) -> Option<u32> {
        self.inner.max()
    }

    pub fn rank(&self, value: u32) -> u64 {
        self.inner.rank(value)
    }

    pub fn is_disjoint(&self, other: &PyRoaingBitmap) -> bool {
        self.inner.is_disjoint(&other.inner)
    }

    pub fn is_subset(&self, other: &PyRoaingBitmap) -> bool {
        self.inner.is_subset(&other.inner)
    }

    pub fn is_superset(&self, other: &PyRoaingBitmap) -> bool {
        self.inner.is_superset(&other.inner)
    }

    pub fn __repr__(&self) -> String {
        self.to_string()
    }

    pub fn __str__(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for PyRoaingBitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<RoaringBitMap: {:?}>", self.inner)
    }
}
