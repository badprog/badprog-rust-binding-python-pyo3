pub mod computation;

use crate::computation::Computation;
use pyo3::prelude::*;

#[pyclass]
pub struct PyComputation {
    pub inner: computation::Computation,
}

#[pymethods]
impl PyComputation {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Computation::new(),
        }
    }

    // ========================================================================
    // add()
    pub fn add(&mut self, left: f64, right: f64) -> f64 {
        self.inner.add(left, right)
    }
    pub fn get_val_add(&self) -> f64 {
        self.inner.get_val_add()
    }

    // ========================================================================
    // sub()
    pub fn sub(&mut self, left: f64, right: f64) -> f64 {
        self.inner.sub(left, right)
    }
    pub fn get_val_sub(&self) -> f64 {
        self.inner.get_val_sub()
    }

    // ========================================================================
    // div()
    pub fn div(&mut self, left: f64, right: f64) -> f64 {
        self.inner.div(left, right)
    }
    pub fn get_val_div(&self) -> f64 {
        self.inner.get_val_div()
    }

    // ========================================================================
    // mul()
    pub fn mul(&mut self, left: f64, right: f64) -> f64 {
        self.inner.mul(left, right)
    }
    pub fn get_val_mul(&self) -> f64 {
        self.inner.get_val_mul()
    }
}

#[pymodule]
fn rust_classic_operations(
    _py: Python,
    m: &pyo3::Bound<'_, pyo3::types::PyModule>,
) -> pyo3::PyResult<()> {
    m.add_class::<PyComputation>()?;
    Ok(())
}
