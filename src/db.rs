use std::sync::Arc;

use ethers_core::types::BlockId;
use revm::{db::EthersDB, Database};
use ethers_providers::{Http, Provider};
use pyo3::prelude::*;
use ruint::aliases::U256;

use crate::{utils::addr::addr, types::account::RAccountInfo};

#[pyclass]
pub struct DB(EthersDB<Provider<Http>>);

#[pymethods]
impl DB {
    #[new]
    fn new(client_url: &str, block_hash: Option<u64>) -> Self {
        let block = match block_hash {
            Some(x) => Some(BlockId::Number(x.into())),
            None => None
        };
        
        let provider = Provider::<Http>::try_from(client_url).unwrap();
        let provider = Arc::new(provider);

        let ethersdb = EthersDB::new(Arc::clone(&provider), block).unwrap();

        DB(ethersdb)
    }

    fn basic(&mut self, address: &str) -> PyResult<Option<RAccountInfo>> {
        let address = addr(address).unwrap();
        
        Ok(Some(RAccountInfo(self.0.basic(address).unwrap().unwrap())))
    }

    fn storage(&mut self, address: &str, index: u128) -> PyResult<String> {
        let address = addr(address).unwrap();
        let index = U256::from(index);
        Ok(self.0.storage(address, index).unwrap().to_string())
    }
}