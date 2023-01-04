use std::fmt::{Display, Formatter};

use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Precision {
    Single,
    Double,
}

#[pymethods]
impl Precision {
    #[new]
    fn new(precision: Precision) -> Self {
        match precision {
            Precision::Double => Precision::Double,
            Precision::Single => Precision::Single
        }
    }
}

#[pyclass]
#[derive(Copy, Clone)]
pub struct Gather {
    #[pyo3(get, set)]
    id: i32,
    #[pyo3(get, set)]
    nx: i32,
    #[pyo3(get, set)]
    nt: i32,
    #[pyo3(get, set)]
    dt: f32,
    #[pyo3(get, set)]
    precision: Option<Precision>,
}

impl Display for Gather {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Seismic Gather {}", self.id)
    }
}

impl Default for Gather {
    fn default() -> Self {
        Gather {
            id: 0,
            nx: 0,
            nt: 0,
            dt: 0.0,
            precision: Option::from(Precision::Double),
        }
    }
}

impl Gather {
    #[allow(dead_code)]
    fn get_stuff(&self) -> (f32, Precision) {
        (self.dt, self.precision.unwrap())
    }
}

#[pymethods]
impl Gather {
    #[new]
    fn new(id: i32, nx: i32, nt: i32, dt: f32, precision: Precision) -> Self {
        Gather { id, nx, nt, dt, precision: Some(precision) }
    }

    fn set_method(&mut self, value: i32) -> PyResult<()> {
        self.nt = value;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stuff_for_dead_code() {
        let my_gather: Gather = Gather { id: 1, nx: 2, nt: 3, dt: 0.01, precision: Some(Precision::Double) };
        let (dt, precision) = my_gather.get_stuff();
        assert_eq!(dt, 0.01);
        assert_eq!(precision, Precision::Double)
    }
}
