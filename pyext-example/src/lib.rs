#[macro_use]
extern crate cpython;
extern crate numpy;
extern crate ndarray;

use numpy::*;
use ndarray::*;
use cpython::{PyResult, Python, PyObject};

/* Pure rust-ndarray functions */

// immutable example
fn sum(x: ArrayViewD<f64>) -> f64 {
    x.scalar_sum()
}

// wrapper of `sum`
fn sum_py(py: Python, x: PyArray) -> PyResult<f64> {
    let x = x.as_array().into_pyresult(py, "x must be f64 array")?;
    let result = sum(x);
    Ok(result) // Python function must returns
}

/* Define module "rust_binding" */
py_module_initializer!(rust_binding, init_rust_binding, PyInit_rust_binding, |py, m| {
    m.add(py, "__doc__", "Rust extension for NumPy")?;
    m.add(py, "sum", py_fn!(py, sum_py(x: PyArray)))?;
    Ok(())
});

