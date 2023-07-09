use db::DB;
use pyo3::prelude::*;
use revm::{EVM as REVM, db::{CacheDB, EmptyDB}};
use types::{evm_env::*, evm_result::RSS, database::Database, account::{RAccountInfo, RDbAccount}};

#[pyclass]
struct EVM(REVM<CacheDB<EmptyDB>>);

#[pymethods]
impl EVM {
    #[new]
    fn new() -> PyResult<EVM> {
        let evm = EVM(REVM::new());
        
        Ok(evm)
    }

    fn database(&mut self, db: Database) {
        self.0.database(db.into())
    }

    fn transact_ref(&self) -> PyResult<RSS> {       
        Ok(RSS(self.0.transact_ref().unwrap()))
    }

    fn set_env(&mut self, env: REnv) {
        self.0.env = env.into();
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
    m.add_class::<Database>()?;
    m.add_class::<RAccountInfo>()?;
    m.add_class::<RDbAccount>()?;
    m.add_class::<DB>()?;

    Ok(())
}

pub mod types;
pub mod utils;
pub mod db;