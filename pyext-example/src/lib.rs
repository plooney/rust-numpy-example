#[macro_use]
extern crate numpy;
extern crate ndarray;
extern crate pyo3;

use numpy::*;
use ndarray::*;
use pyo3::prelude::{pymodinit, Py, PyModule, PyResult, Python};

/* Pure rust-ndarray functions */


#[pymodinit]
fn rust_binding(_py: Python, m: &PyModule) -> PyResult<()> {

    // immutable example
    fn sum(x: ArrayViewMutD<f64>) -> f64 {
        x.scalar_sum()
    }

    // wrapper of `sum`
    #[pyfn(m, "sum")]
    fn sum_py(py: Python, x: &PyArrayDyn<f64>) -> PyResult<f64> {
        let x = x.as_array_mut();
        let result = sum(x);
        Ok(result) // Python function must returns
    }

    Ok(())
}
