//! Errors

use thiserror::Error;

pub type Result<T> = std::result::Result<T, BananoError>;

#[derive(Error, Debug)]
pub enum BananoError {
    #[error("Web3 provider error")]
    RpcError(#[from] reqwest::Error),
    #[error("Decoding error")]
    DecodeError(#[from] data_encoding::DecodeError),
    #[error("PublicKey error")]
    PublicKeyError(#[from] ed25519_dalek::ed25519::Error),
    #[error("Invalid Address")]
    InvalidAddress,
    #[error("Invalid Address length")]
    InvalidAddressLength(usize),
    #[error("Invalid Seed length")]
    SeedLengthError(usize),
    #[error("Parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Parse big decimal error")]
    ParseBigDecimalError(#[from] bigdecimal::ParseBigDecimalError),
    #[error("Wrong length for {msg} (expected {expected:?}, found {found:?})")]
    WrongLength {
        msg: String,
        expected: usize,
        found: usize,
    },
    #[error("Unknown character found while decoding: {0}")]
    DecodingError(char),
    #[error("From hex error: {msg} {source}")]
    FromHexError {
        msg: String,
        source: hex::FromHexError,
    },
}