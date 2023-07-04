use pyo3::prelude::*;
use revm::{EVM as REVM, DummyStateDB, db::EmptyDB};
use types::{evm_env::*, evm_result::RSS};

#[pyclass]
struct EVM(REVM<DummyStateDB>);

impl EVM {
    pub fn database(&mut self) {
        self.0.database(DummyStateDB::new(EmptyDB()));
    }
}

#[pymethods]
impl EVM {
    #[new]
    fn new(env: REnv) -> PyResult<EVM> {
        let mut evm = EVM(REVM::with_env(env.into()));

        evm.database();
        
        Ok(evm)
    }

    fn transact(mut _self: PyRefMut<'_, Self>) -> PyResult<RSS> {
        Ok(RSS(_self.0.transact().unwrap()))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_revm(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EVM>()?;
    m.add_class::<REnv>()?;
    m.add_class::<RCfgEnv>()?;
    m.add_class::<RTxEnv>()?;
    m.add_class::<RBlockEnv>()?;
    m.add_class::<RSS>()?;

    Ok(())
}

pub mod types;