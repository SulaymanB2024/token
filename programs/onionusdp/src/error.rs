// MIT License
//
// Copyright (c) 2025 Solana Program Library

use solana_decode_error::DecodeError;
use solana_program_error::ProgramError;
use thiserror::Error;

/// Errors that may be returned by the OnionUSD-P program
#[derive(Error, Debug, Copy, Clone)]
pub enum OnionUSDError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Invalid authority")]
    InvalidAuthority,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Account not initialized")]
    NotInitialized,
    #[error("Account already initialized")]
    AlreadyInitialized,
    #[error("Payroll not yet unlocked")]
    PayrollLocked,
    #[error("Invalid percentage")]
    InvalidPercentage,
}

impl From<OnionUSDError> for ProgramError {
    fn from(e: OnionUSDError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for OnionUSDError {
    fn type_of() -> &'static str {
        "OnionUSDError"
    }
}