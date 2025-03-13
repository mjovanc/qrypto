use thiserror::Error;

#[derive(Error, Debug)]
pub enum QryptoError {
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("Decapsulation failed")]
    DecapsulationFailed,
    #[error("RNG error")]
    RngError(#[from] rand::Error),
}
