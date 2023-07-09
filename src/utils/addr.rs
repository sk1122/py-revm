use revm::primitives::B160;
use pyo3::{prelude::*, exceptions::PyTypeError};

pub fn addr(addr: &str) -> Result<B160, PyErr> {
    addr.parse::<B160>()
        .map_err(|err| PyTypeError::new_err(err.to_string()))
}
