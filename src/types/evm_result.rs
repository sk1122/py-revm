use pyo3::{prelude::*, types::PyDict};
use revm::primitives::{ResultAndState, ExecutionResult, State, Account, Eval, Log, Output};

#[pyclass]
#[derive(Debug, Clone)]
pub struct RSS(pub ResultAndState);

#[pyclass]
#[derive(Debug, Clone)]
pub struct RExecutionResult(ExecutionResult);

fn convert_eval_to_string(reason: &Eval) -> String {
    match reason {
        Eval::Return => "Return".to_string(),
        Eval::SelfDestruct => "SelfDestruct".to_string(),
        Eval::Stop => "Stop".to_string(),
    }
}

fn convert_log_to_dict(log: Log, py: Python<'_>) -> &PyDict {
    let dict = PyDict::new(py);

    dict.set_item("address", log.address.to_string()).unwrap();
    dict.set_item("data", log.data.to_vec()).unwrap();
    dict.set_item("topics", log.topics.iter().map(|val| val.to_string()).collect::<Vec<String>>()).unwrap();

    dict
}

fn convert_output_to_dict(output: &Output) -> Vec<String> {
    match output {
        Output::Call(x) => x.to_vec().iter().map(|val| val.to_string()).collect::<Vec<String>>(),
        Output::Create(x, _) => x.to_vec().iter().map(|val| val.to_string()).collect::<Vec<String>>()
    }
}

#[pymethods]
impl RExecutionResult {
    #[getter]
    fn get<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyDict> {
        println!("why is this reverting? {:?}", self.0);
        match &self.0 {
            ExecutionResult::Success { reason, gas_used, gas_refunded, logs, output } => {
                let dict = PyDict::new(py);
                dict.set_item("reason", convert_eval_to_string(reason))?;
                dict.set_item("gas_used", gas_used)?;
                dict.set_item("gas_refunded", gas_refunded)?;
                dict.set_item("logs", logs.iter().map(|log| convert_log_to_dict(log.clone(), py)).collect::<Vec<&PyDict>>())?;
                dict.set_item("output", convert_output_to_dict(output))?;

                Ok(dict)
            },
            ExecutionResult::Revert { gas_used, output } => {
                let dict = PyDict::new(py);

                dict.set_item("gas_used", gas_used)?;
                dict.set_item("output", output.to_vec().iter().map(|val| val.to_string()).collect::<Vec<String>>())?;

                Ok(dict)
            },
            ExecutionResult::Halt { reason, gas_used } => {
                let dict: &'a PyDict = PyDict::new(py);

                dict.set_item("gas_used", gas_used)?;
                // dict.set_item("reason", );

                Ok(dict)
            }
        }
    }
}

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