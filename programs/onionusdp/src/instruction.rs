// MIT License
//
// Copyright (c) 2025 Solana Program Library

use solana_instruction::{AccountMeta, Instruction};
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_program::system_program;

/// Instructions supported by the OnionUSD-P program
#[derive(Clone, Debug, PartialEq)]
pub enum OnionUSDInstruction {
    /// Initialize the global configuration
    ///
    /// Accounts expected:
    /// 0. `[writable, signer]` Config account (PDA)
    /// 1. `[signer]` Authority
    /// 2. `[]` System program
    InitializeConfig {
        float_pct: u8,
        risk_pct: u8,
    },

    /// Deposit USDC and mint OnionUSD tokens
    ///
    /// Accounts expected:
    /// 0. `[]` Config account (PDA)
    /// 1. `[writable]` User's USDC token account
    /// 2. `[writable]` Reserve vault USDC account (PDA)
    /// 3. `[writable]` OnionUSD mint
    /// 4. `[writable]` User's OnionUSD token account
    /// 5. `[signer]` User
    /// 6. `[]` Token program
    DepositUSDC {
        amount: u64,
    },

    /// Redeem OnionUSD tokens back to USDC
    ///
    /// Accounts expected:
    /// 0. `[]` Config account (PDA)
    /// 1. `[writable]` User's OnionUSD token account
    /// 2. `[writable]` OnionUSD mint
    /// 3. `[writable]` Reserve vault USDC account (PDA)
    /// 4. `[writable]` User's USDC token account
    /// 5. `[signer]` User
    /// 6. `[]` Token program
    RedeemUSDC {
        amount: u64,
    },

    /// Schedule payroll batch with frozen tokens
    ///
    /// Accounts expected:
    /// 0. `[]` Config account (PDA)
    /// 1. `[writable]` Payroll escrow account (PDA)
    /// 2. `[writable]` OnionUSD mint
    /// 3. `[writable]` Escrow token account
    /// 4. `[signer]` Paymaster
    /// 5. `[]` Token program
    /// 6. `[]` System program
    SchedulePayroll {
        total_amount: u64,
        release_at: i64,
        batch_id: u64,
    },

    /// Release payroll tokens after time lock expires
    ///
    /// Accounts expected:
    /// 0. `[]` Config account (PDA)
    /// 1. `[writable]` Payroll escrow account (PDA)
    /// 2. `[writable]` Escrow token account
    /// 3. `[writable]` Employee token account
    /// 4. `[signer]` Paymaster or employee
    /// 5. `[]` Token program
    /// 6. `[]` Clock sysvar
    ThawPayroll {
        amount: u64,
    },

    /// Stub for future yield accrual functionality
    #[cfg(feature = "future-yield")]
    AccrueYieldStub,
}

impl OnionUSDInstruction {
    /// Unpacks a byte buffer into an OnionUSDInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use std::convert::TryInto;

        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        
        Ok(match tag {
            0 => {
                if rest.len() < 2 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                Self::InitializeConfig {
                    float_pct: rest[0],
                    risk_pct: rest[1],
                }
            }
            1 => {
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Self::DepositUSDC { amount }
            }
            2 => {
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Self::RedeemUSDC { amount }
            }
            3 => {
                if rest.len() < 24 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let total_amount = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                let release_at = i64::from_le_bytes(rest[8..16].try_into().unwrap());
                let batch_id = u64::from_le_bytes(rest[16..24].try_into().unwrap());
                Self::SchedulePayroll {
                    total_amount,
                    release_at,
                    batch_id,
                }
            }
            4 => {
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Self::ThawPayroll { amount }
            }
            #[cfg(feature = "future-yield")]
            5 => Self::AccrueYieldStub,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    /// Packs an OnionUSDInstruction into a byte buffer
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            Self::InitializeConfig { float_pct, risk_pct } => {
                buf.push(0);
                buf.push(*float_pct);
                buf.push(*risk_pct);
            }
            Self::DepositUSDC { amount } => {
                buf.push(1);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::RedeemUSDC { amount } => {
                buf.push(2);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::SchedulePayroll {
                total_amount,
                release_at,
                batch_id,
            } => {
                buf.push(3);
                buf.extend_from_slice(&total_amount.to_le_bytes());
                buf.extend_from_slice(&release_at.to_le_bytes());
                buf.extend_from_slice(&batch_id.to_le_bytes());
            }
            Self::ThawPayroll { amount } => {
                buf.push(4);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            #[cfg(feature = "future-yield")]
            Self::AccrueYieldStub => {
                buf.push(5);
            }
        }
        buf
    }
}

/// Create an `InitializeConfig` instruction
pub fn initialize_config(
    program_id: &Pubkey,
    config: &Pubkey,
    authority: &Pubkey,
    float_pct: u8,
    risk_pct: u8,
) -> Result<Instruction, ProgramError> {
    let data = OnionUSDInstruction::InitializeConfig { float_pct, risk_pct }.pack();

    let accounts = vec![
        AccountMeta::new(*config, false),
        AccountMeta::new(*authority, true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create a `DepositUSDC` instruction
pub fn deposit_usdc(
    program_id: &Pubkey,
    config: &Pubkey,
    user_usdc_account: &Pubkey,
    reserve_vault: &Pubkey,
    onionusd_mint: &Pubkey,
    user_onionusd_account: &Pubkey,
    user: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = OnionUSDInstruction::DepositUSDC { amount }.pack();

    let accounts = vec![
        AccountMeta::new_readonly(*config, false),
        AccountMeta::new(*user_usdc_account, false),
        AccountMeta::new(*reserve_vault, false),
        AccountMeta::new(*onionusd_mint, false),
        AccountMeta::new(*user_onionusd_account, false),
        AccountMeta::new(*user, true),
        // TODO: Replace with correct SPL token program ID when implementing CPI
        AccountMeta::new_readonly(system_program::id(), false), 
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}