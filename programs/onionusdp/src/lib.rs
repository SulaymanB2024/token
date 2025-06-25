// MIT License
//
// Copyright (c) 2025 Solana Program Library
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

use solana_account_info::AccountInfo;
use solana_program_entrypoint::entrypoint;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_program::{declare_id, msg};

pub mod instruction;
pub mod processor;
pub mod state;
pub mod error;

pub use instruction::*;
pub use processor::*;
pub use state::*;
pub use error::*;

declare_id!("Fv2FHYdUGBj3kKz23zHcPfwZHR5sMmH2NftD2V7U46fq");

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

/// Program entrypoint
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    processor::process(program_id, accounts, instruction_data)
}