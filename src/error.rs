#![allow(dead_code)]
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("cannot serialize data")]
    SerializingError(#[from] serde_cbor::Error),
    #[error("IndexNotPresent")]
    IndexNotPresent(String),
    #[error("Unknown")]
    Unknown,
    #[error("Data is corrupted")]
    DataCorrupted,
}
