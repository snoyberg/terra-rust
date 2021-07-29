#![allow(missing_docs)]
#![allow(missing_docs)]
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerraRustWalletError {
    #[error(transparent)]
    KeyringError(#[from] crate::keyring::KeyringError),
    #[error(transparent)]
    SerdeJsonError(#[from] ::serde_json::Error),

    #[error("Terra Wallet `{key}` Key not found Error")]
    KeyNotFound {
        key: String,
        source: crate::keyring::KeyringError,
    },

    #[error("unknown Terra-Rust Wallet error")]
    Unknown,
}
