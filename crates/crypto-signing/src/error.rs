use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Failed to generate keypair: {0}")]
    KeyGeneration(String),

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Invalid key length: expected 32, got {0}")]
    InvalidKeyLength(usize),

    #[error("Hex decode error: {0}")]
    HexDecode(String),

    #[error("Signature verification failed: {0}")]
    VerificationFailed(String),

    #[error("Signing failed: {0}")]
    SigningFailed(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}
