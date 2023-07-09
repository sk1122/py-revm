use revm::{primitives::{AccountInfo, B256, Bytecode, KECCAK_EMPTY}, db::DbAccount};
use pyo3::{prelude::*, types::PyBytes};
use ruint::aliases::U256;

#[pyclass]
#[derive(Debug, Clone)]
pub struct RAccountInfo(pub AccountInfo);

#[pymethods]
impl RAccountInfo {
    // TODO: Is there a way to avoid all this boilerplate somehow?
    #[getter]
    fn balance(_self: PyRef<'_, Self>) -> u128 {
        _self.0.balance.to()
    }
    #[getter]
    fn nonce(_self: PyRef<'_, Self>) -> u64 {
        _self.0.nonce
    }
    #[getter]
    fn code(_self: PyRef<'_, Self>) -> Vec<u8> {
        _self
            .0
            .code
            .as_ref()
            .map(|x| x.bytes().to_vec())
            .unwrap_or_default()
    }
    #[getter]
    fn code_hash(_self: PyRef<'_, Self>) -> [u8; 32] {
        _self.0.code_hash.to_fixed_bytes()
    }

    #[new]
    #[pyo3(signature=(balance, code_hash, code, nonce = 0))]
    fn new(
        balance: Option<u128>,
        code_hash: Option<&PyBytes>,
        code: Option<&PyBytes>,
        nonce: u64,
    ) -> PyResult<Self> {
        println!("dsa2");
        let code_hash = code_hash
            .map(|bytes| {
                let bytes = bytes.as_bytes();
                B256::from_slice(bytes)
            })
            .unwrap_or(KECCAK_EMPTY);
        println!("dsa1");
        let code = code
            .map(|bytes| {
                let bytes = bytes.as_bytes();
                bytes.to_vec()
            })
            .map(|bytes| Bytecode::new_raw(bytes.into()));

println!("dsa");
        Ok(RAccountInfo(AccountInfo {
            balance: U256::from(balance.unwrap_or_default()),
            code_hash,
            code,
            nonce,
        }))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}


#[pyclass]
#[derive(Debug, Clone)]
pub struct RDbAccount(pub DbAccount);