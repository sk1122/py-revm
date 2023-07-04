use pyo3::prelude::*;
use revm::primitives::ResultAndState;

#[pyclass]
#[derive(Debug, Clone)]
pub struct RSS(pub ResultAndState);