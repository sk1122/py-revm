use std::str::FromStr;

use revm::{db::CacheDB, db::{EmptyDB, DbAccount}, primitives::AccountInfo};
use pyo3::prelude::*;
use ruint::aliases::U256;

use crate::utils::addr::{addr, from_addr};

use super::account::{RAccountInfo, RDbAccount};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Database(CacheDB<EmptyDB>);

#[pymethods]
impl Database {
    #[new]
    fn new() -> Self {
        Database(CacheDB::new(EmptyDB::default()))
    }

    fn insert_account_info(&mut self, address: &str, info: RAccountInfo) {
        self.0.insert_account_info(addr(address).unwrap(), info.into())
    }

    fn insert_account_storage(&mut self, address: &str, slot: u128, value: &str) {
        self.0.insert_account_storage(addr(address).unwrap(), U256::from(slot), U256::from_str(value).unwrap()).unwrap()
    }

    #[getter]
    fn accounts(&self) -> PyResult<Vec<(String, RDbAccount)>> {
        let accounts = self.0.accounts.iter().map(|(k, v)| (from_addr(k), v.into())).collect::<Vec<(String, RDbAccount)>>();
        
        Ok(accounts)
    }
}

impl From<Database> for CacheDB<EmptyDB> {
    fn from(value: Database) -> Self {
        value.0
    }
}

impl From<RAccountInfo> for AccountInfo {
    fn from(value: RAccountInfo) -> Self {
        value.0
    }
}

impl From<&DbAccount> for RDbAccount {
    fn from(value: &DbAccount) -> Self {
        RDbAccount(value.clone())
    }
}