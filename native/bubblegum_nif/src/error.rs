use thiserror::Error;

#[derive(Error, Debug)]
pub enum BubblegumError {
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Invalid instruction: {0}")]
    InvalidInstruction(String),

    #[error("Merkle tree error: {0}")]
    MerkleTreeError(String),

    #[error("Invalid metadata: {0}")]
    InvalidMetadata(String),

    #[error("Invalid transfer: {0}")]
    InvalidTransfer(String),
}

impl From<BubblegumError> for rustler::Error {
    fn from(error: BubblegumError) -> Self {
        match error {
            BubblegumError::InvalidPublicKey(msg) => rustler::Error::Term(Box::new(format!("Invalid public key: {}", msg))),
            BubblegumError::TransactionError(msg) => rustler::Error::Term(Box::new(format!("Transaction error: {}", msg))),
            BubblegumError::NetworkError(msg) => rustler::Error::Term(Box::new(format!("Network error: {}", msg))),
            BubblegumError::SerializationError(msg) => rustler::Error::Term(Box::new(format!("Serialization error: {}", msg))),
            BubblegumError::RpcError(msg) => rustler::Error::Term(Box::new(format!("RPC error: {}", msg))),
            BubblegumError::CacheError(msg) => rustler::Error::Term(Box::new(format!("Cache error: {}", msg))),
            BubblegumError::InvalidInstruction(msg) => rustler::Error::Term(Box::new(format!("Invalid instruction: {}", msg))),
            BubblegumError::MerkleTreeError(msg) => rustler::Error::Term(Box::new(format!("Merkle tree error: {}", msg))),
            BubblegumError::InvalidMetadata(msg) => rustler::Error::Term(Box::new(format!("Invalid metadata: {}", msg))),
            BubblegumError::InvalidTransfer(msg) => rustler::Error::Term(Box::new(format!("Invalid transfer: {}", msg))),
        }
    }
}