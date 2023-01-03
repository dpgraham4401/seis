use pyo3::prelude::*;

#[pyclass]
struct Gather {
    id: i32,
    nx: i32,
    nt: i32,
}

#[pymethods]
impl Gather {
    #[new]
    fn new(id: i32, nx: i32, nt: i32) -> Self {
        Gather{id, nx, nt}
    }
    fn get_id(&self) -> PyResult<i32> {
        Ok(self.id)
    }

    fn get_nx(&self) -> PyResult<i32> {
        Ok(self.nx)
    }

    fn set_method(&mut self, value: i32) -> PyResult<()> {
        self.nt = value;
        Ok(())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn seis(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Gather>()?;
    Ok(())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
