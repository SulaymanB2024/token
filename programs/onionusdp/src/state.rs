// MIT License
//
// Copyright (c) 2025 Solana Program Library

use bytemuck::{Pod, Zeroable};
use solana_program_pack::{IsInitialized, Pack, Sealed};
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

/// Global configuration for OnionUSD-P program
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct Config {
    /// Float percentage for reserve calculations
    pub float_pct: u8,
    /// Risk percentage for reserve calculations  
    pub risk_pct: u8,
    /// Authority that can modify config
    pub signer: Pubkey,
    /// Padding for alignment
    pub _padding: [u8; 5],
}

/// Redemption allowance for a specific wallet
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct RedeemAllow {
    /// Maximum amount that can be redeemed
    pub limit: u64,
}

/// Payroll escrow batch information
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct PayrollEscrow {
    /// Total amount in the escrow
    pub total: u64,
    /// Unix timestamp when tokens can be released
    pub release_at: i64,
}

impl Sealed for Config {}
impl Sealed for RedeemAllow {}
impl Sealed for PayrollEscrow {}

impl IsInitialized for Config {
    fn is_initialized(&self) -> bool {
        self.signer != Pubkey::default()
    }
}

impl IsInitialized for RedeemAllow {
    fn is_initialized(&self) -> bool {
        true // RedeemAllow can have zero limit
    }
}

impl IsInitialized for PayrollEscrow {
    fn is_initialized(&self) -> bool {
        self.total > 0 || self.release_at > 0
    }
}

impl Pack for Config {
    const LEN: usize = 1 + 1 + 32 + 5; // 39 bytes

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let data = bytemuck::bytes_of(self);
        dst[..Self::LEN].copy_from_slice(data);
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        bytemuck::try_from_bytes(&src[..Self::LEN])
            .map_err(|_| ProgramError::InvalidAccountData)
            .map(|x| *x)
    }
}

impl Pack for RedeemAllow {
    const LEN: usize = 8; // 8 bytes

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let data = bytemuck::bytes_of(self);
        dst[..Self::LEN].copy_from_slice(data);
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        bytemuck::try_from_bytes(&src[..Self::LEN])
            .map_err(|_| ProgramError::InvalidAccountData)
            .map(|x| *x)
    }
}

impl Pack for PayrollEscrow {
    const LEN: usize = 8 + 8; // 16 bytes

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let data = bytemuck::bytes_of(self);
        dst[..Self::LEN].copy_from_slice(data);
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        bytemuck::try_from_bytes(&src[..Self::LEN])
            .map_err(|_| ProgramError::InvalidAccountData)
            .map(|x| *x)
    }
}

/// Seeds for generating PDAs
pub mod seeds {
    pub const CONFIG: &[u8] = b"config";
    pub const RESERVE: &[u8] = b"reserve";
    pub const REDEEM: &[u8] = b"redeem";
    pub const BATCH: &[u8] = b"batch";
}