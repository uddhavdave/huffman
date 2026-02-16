use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("cannot serialize data")]
    SerializingError(#[from] serde_cbor::Error),
    #[error("IndexNotPresent")]
    IndexNotPresent(String),
    #[error("Data is corrupted")]
    DataCorrupted,
}
