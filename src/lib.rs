use pyo3::prelude::*;

use gather::Gather;
use crate::gather::Precision;

pub mod gather;

/// A Python Seismic processing module implemented in Rust.
#[pymodule]
fn seis(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Gather>()?;
    m.add_class::<Precision>()?;
    Ok(())
}

