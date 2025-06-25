// MIT License
//
// Copyright (c) 2025 Solana Program Library

use crate::{
    error::OnionUSDError,
    instruction::OnionUSDInstruction,
    state::{Config, PayrollEscrow, seeds},
};
use solana_account_info::{next_account_info, AccountInfo};
use solana_program_error::ProgramError;
use solana_program_pack::{IsInitialized, Pack};
use solana_pubkey::Pubkey;
use solana_program::{
    msg,
    pubkey::Pubkey as ProgramPubkey,
    system_instruction,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
    program::invoke_signed,
};

/// Instruction processing router
pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let instruction = OnionUSDInstruction::unpack(instruction_data)?;
    
    match instruction {
        OnionUSDInstruction::InitializeConfig { float_pct, risk_pct } => {
            process_initialize_config(program_id, accounts, float_pct, risk_pct)
        }
        OnionUSDInstruction::DepositUSDC { amount } => {
            process_deposit_usdc(program_id, accounts, amount)
        }
        OnionUSDInstruction::RedeemUSDC { amount } => {
            process_redeem_usdc(program_id, accounts, amount)
        }
        OnionUSDInstruction::SchedulePayroll { 
            total_amount, 
            release_at, 
            batch_id 
        } => {
            process_schedule_payroll(program_id, accounts, total_amount, release_at, batch_id)
        }
        OnionUSDInstruction::ThawPayroll { amount } => {
            process_thaw_payroll(program_id, accounts, amount)
        }
        #[cfg(feature = "future-yield")]
        OnionUSDInstruction::AccrueYieldStub => {
            process_accrue_yield_stub(program_id, accounts)
        }
    }
}

/// Initialize the global configuration
fn process_initialize_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    float_pct: u8,
    risk_pct: u8,
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let config_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;

    // Convert to ProgramPubkey for find_program_address
    let program_id_key = ProgramPubkey::from(program_id.to_bytes());
    
    // Verify PDA
    let (config_pda, config_bump) = ProgramPubkey::find_program_address(
        &[seeds::CONFIG],
        &program_id_key,
    );
    if config_info.key != &Pubkey::from(config_pda.to_bytes()) {
        return Err(ProgramError::InvalidAccountData);
    }

    // Verify authority is signer
    if !authority_info.is_signer {
        return Err(OnionUSDError::InvalidAuthority.into());
    }

    // Validate percentages
    if float_pct > 100 || risk_pct > 100 {
        return Err(OnionUSDError::InvalidPercentage.into());
    }

    // Create the config account
    let rent = Rent::get()?;
    let space = Config::LEN;
    let lamports = rent.minimum_balance(space);

    let create_account_ix = system_instruction::create_account(
        authority_info.key,
        config_info.key,
        lamports,
        space as u64,
        program_id,
    );

    invoke_signed(
        &create_account_ix,
        &[
            authority_info.clone(),
            config_info.clone(),
            system_program_info.clone(),
        ],
        &[&[seeds::CONFIG, &[config_bump]]],
    )?;

    // Initialize config data
    let mut config = Config::default();
    config.float_pct = float_pct;
    config.risk_pct = risk_pct;
    config.signer = *authority_info.key;

    Config::pack(config, &mut config_info.try_borrow_mut_data()?)?;

    msg!("OnionUSD-P config initialized with float_pct: {}, risk_pct: {}", 
         float_pct, risk_pct);

    Ok(())
}

/// Deposit USDC and mint OnionUSD tokens
fn process_deposit_usdc(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let config_info = next_account_info(account_info_iter)?;
    let _user_usdc_info = next_account_info(account_info_iter)?;
    let _reserve_vault_info = next_account_info(account_info_iter)?;
    let _onionusd_mint_info = next_account_info(account_info_iter)?;
    let _user_onionusd_info = next_account_info(account_info_iter)?;
    let user_info = next_account_info(account_info_iter)?;
    let _token_program_info = next_account_info(account_info_iter)?;

    // Convert for PDA verification
    let program_id_key = ProgramPubkey::from(program_id.to_bytes());
    let (config_pda, _) = ProgramPubkey::find_program_address(&[seeds::CONFIG], &program_id_key);
    if config_info.key != &Pubkey::from(config_pda.to_bytes()) {
        return Err(ProgramError::InvalidAccountData);
    }

    // Load and verify config
    let config = Config::unpack(&config_info.try_borrow_data()?)?;
    if !config.is_initialized() {
        return Err(OnionUSDError::NotInitialized.into());
    }

    // Verify user is signer
    if !user_info.is_signer {
        return Err(OnionUSDError::InvalidAuthority.into());
    }

    // TODO: Implement CPI calls to:
    // 1. Transfer USDC from user to reserve vault
    // 2. Mint equivalent OnionUSD to user's token account
    msg!("TODO: Implement deposit_usdc for amount: {}", amount);
    
    Ok(())
}

/// Redeem OnionUSD tokens back to USDC
fn process_redeem_usdc(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let config_info = next_account_info(account_info_iter)?;
    let _user_onionusd_info = next_account_info(account_info_iter)?;
    let _onionusd_mint_info = next_account_info(account_info_iter)?;
    let _reserve_vault_info = next_account_info(account_info_iter)?;
    let _user_usdc_info = next_account_info(account_info_iter)?;
    let user_info = next_account_info(account_info_iter)?;
    let _token_program_info = next_account_info(account_info_iter)?;

    // Convert for PDA verification
    let program_id_key = ProgramPubkey::from(program_id.to_bytes());
    let (config_pda, _) = ProgramPubkey::find_program_address(&[seeds::CONFIG], &program_id_key);
    if config_info.key != &Pubkey::from(config_pda.to_bytes()) {
        return Err(ProgramError::InvalidAccountData);
    }

    // Load and verify config
    let config = Config::unpack(&config_info.try_borrow_data()?)?;
    if !config.is_initialized() {
        return Err(OnionUSDError::NotInitialized.into());
    }

    // Verify user is signer
    if !user_info.is_signer {
        return Err(OnionUSDError::InvalidAuthority.into());
    }

    // TODO: Implement CPI calls to:
    // 1. Burn OnionUSD tokens from user
    // 2. Transfer equivalent USDC from reserve vault to user
    msg!("TODO: Implement redeem_usdc for amount: {}", amount);
    
    Ok(())
}

/// Schedule payroll batch with frozen tokens
fn process_schedule_payroll(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    total_amount: u64,
    release_at: i64,
    batch_id: u64,
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let config_info = next_account_info(account_info_iter)?;
    let _escrow_info = next_account_info(account_info_iter)?;
    let _onionusd_mint_info = next_account_info(account_info_iter)?;
    let _escrow_token_info = next_account_info(account_info_iter)?;
    let paymaster_info = next_account_info(account_info_iter)?;
    let _token_program_info = next_account_info(account_info_iter)?;
    let _system_program_info = next_account_info(account_info_iter)?;

    // Convert for PDA verification
    let program_id_key = ProgramPubkey::from(program_id.to_bytes());
    let (config_pda, _) = ProgramPubkey::find_program_address(&[seeds::CONFIG], &program_id_key);
    if config_info.key != &Pubkey::from(config_pda.to_bytes()) {
        return Err(ProgramError::InvalidAccountData);
    }

    // Load and verify config
    let config = Config::unpack(&config_info.try_borrow_data()?)?;
    if !config.is_initialized() {
        return Err(OnionUSDError::NotInitialized.into());
    }

    // Verify paymaster authority
    if paymaster_info.key != &config.signer || !paymaster_info.is_signer {
        return Err(OnionUSDError::InvalidAuthority.into());
    }

    // Verify release time is in the future
    let clock = Clock::get()?;
    if release_at <= clock.unix_timestamp {
        return Err(ProgramError::InvalidArgument);
    }

    // TODO: Implement:
    // 1. Create payroll escrow PDA account
    // 2. Mint OnionUSD tokens into frozen escrow
    // 3. Store payroll batch information
    msg!("TODO: Implement schedule_payroll for amount: {}, release_at: {}, batch_id: {}", 
         total_amount, release_at, batch_id);
    
    Ok(())
}

/// Release payroll tokens after time lock expires
fn process_thaw_payroll(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let config_info = next_account_info(account_info_iter)?;
    let escrow_info = next_account_info(account_info_iter)?;
    let _escrow_token_info = next_account_info(account_info_iter)?;
    let _employee_token_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let _token_program_info = next_account_info(account_info_iter)?;
    let _clock_info = next_account_info(account_info_iter)?;

    // Convert for PDA verification
    let program_id_key = ProgramPubkey::from(program_id.to_bytes());
    let (config_pda, _) = ProgramPubkey::find_program_address(&[seeds::CONFIG], &program_id_key);
    if config_info.key != &Pubkey::from(config_pda.to_bytes()) {
        return Err(ProgramError::InvalidAccountData);
    }

    // Load and verify config
    let config = Config::unpack(&config_info.try_borrow_data()?)?;
    if !config.is_initialized() {
        return Err(OnionUSDError::NotInitialized.into());
    }

    // Verify authority is signer
    if !authority_info.is_signer {
        return Err(OnionUSDError::InvalidAuthority.into());
    }

    // Load escrow data
    let escrow = PayrollEscrow::unpack(&escrow_info.try_borrow_data()?)?;
    if !escrow.is_initialized() {
        return Err(OnionUSDError::NotInitialized.into());
    }

    // Check if payroll can be released
    let clock = Clock::get()?;
    if clock.unix_timestamp < escrow.release_at {
        return Err(OnionUSDError::PayrollLocked.into());
    }

    // TODO: Implement token transfer from escrow to employee
    msg!("TODO: Implement thaw_payroll for amount: {}", amount);
    
    Ok(())
}

#[cfg(feature = "future-yield")]
/// Stub for future yield accrual functionality
fn process_accrue_yield_stub(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    let clock = Clock::get()?;
    msg!("Yield accrual stub called at timestamp: {} - feature coming soon", 
         clock.unix_timestamp);
    Ok(())
}