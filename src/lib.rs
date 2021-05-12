extern crate pyo3;

use pyo3::prelude::*;
use pyo3::Python;

#[pyclass]
#[repr(C)]
struct Wrapper {
    #[pyo3(get, set)]
    child: Child,
}

impl Clone for Wrapper {
    fn clone(&self) -> Self {
        Self {
            child: self.child.clone(),
        }
    }
}

#[pymethods]
impl Wrapper {
    #[new]
    fn new() -> Self {
        Self {
            child: Child { flag: true }
        }
    }
}

#[pyclass]
struct Child {
    #[pyo3(get, set)]
    flag: bool,
}

#[pymethods]
impl Child {
    #[new]
    fn new() -> Self {
        Self {
            flag: true
        }
    }
}

impl Clone for Child {
    fn clone(&self) -> Self {
        Self {
            flag: self.flag,
        }
    }
}

#[pymodule]
fn azul(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Wrapper>()?;
    m.add_class::<Child>()?;

    Ok(())
}
