use pyo3::{prelude::*, types::{PyDict, IntoPyDict}};
use revm::primitives::{ResultAndState, ExecutionResult, State, Account};

#[pyclass]
#[derive(Debug, Clone)]
pub struct RSS(pub ResultAndState);

#[pyclass]
#[derive(Debug, Clone)]
pub struct RExecutionResult(ExecutionResult);

#[pyclass]
#[derive(Debug, Clone)]
pub struct RState(State);

fn account_to_object(account: &Account, py: Python<'_>) -> PyObject {
    // todo account.storage
    vec![
        ("is_destroyed", account.is_destroyed.to_object(py)),
        ("is_touched", account.is_touched.to_object(py)),
        ("is_not_existing", account.is_not_existing.to_object(py)),
        ("storage_cleared", account.storage_cleared.to_object(py)),
    ].to_object(py)
}

fn state_to_object(state: &State, py: Python<'_>) -> PyObject {
    state.iter().map(|(k, v)| (k.into_py(py), account_to_object(v, py))).collect::<Vec<_>>().to_object(py)
}

#[pymethods]
impl RSS {
    #[getter]
    fn result(&self) -> PyResult<RExecutionResult> {
        Ok(RExecutionResult(self.0.result.clone()))
    }

    #[getter]
    fn state(&self, py: Python<'_>) -> PyResult<PyObject> {
        Ok(state_to_object(&self.0.state, py))
    }
}