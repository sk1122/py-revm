use pyo3::prelude::*;
use revm::primitives::{Env, CfgEnv, BlockEnv, TxEnv};

#[pyclass]
#[derive(Debug, Default, Clone)]
pub struct REnv(Env);

#[pyclass]
#[derive(Debug, Default, Clone)]
pub struct RCfgEnv(CfgEnv);

#[pyclass]
#[derive(Debug, Default, Clone)]
pub struct RBlockEnv(BlockEnv);

#[pyclass]
#[derive(Debug, Default, Clone)]
pub struct RTxEnv(TxEnv);

#[pymethods]
impl REnv {
    #[new]
    fn new(cfg: Option<RCfgEnv>, block: Option<RBlockEnv>, tx: Option<RTxEnv>) -> Self {
        REnv(Env {
            cfg: cfg.unwrap_or_default().into(),
            block: block.unwrap_or_default().into(),
            tx: tx.unwrap_or_default().into(),
        })
    }
}

impl From<Env> for REnv {
    fn from(env: Env) -> Self {
        REnv(env)
    }
}

impl From<REnv> for Env {
    fn from(env: REnv) -> Self {
        env.0
    }
}

#[pymethods]
impl RCfgEnv {
    #[new]
    fn new() -> Self {
        RCfgEnv(CfgEnv::default())
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pymethods]
impl RBlockEnv {
    #[new]
    fn new() -> Self {
        RBlockEnv(BlockEnv::default())
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pymethods]
impl RTxEnv {
    #[new]
    fn new() -> Self {
        RTxEnv(TxEnv::default())
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

impl From<RCfgEnv> for CfgEnv {
    fn from(env: RCfgEnv) -> Self {
        env.0
    }
}

impl From<RBlockEnv> for BlockEnv {
    fn from(env: RBlockEnv) -> Self {
        env.0
    }
}

impl From<RTxEnv> for TxEnv {
    fn from(env: RTxEnv) -> Self {
        env.0
    }
}