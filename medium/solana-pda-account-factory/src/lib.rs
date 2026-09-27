#![allow(unexpected_cfgs)]

#[allow(unused_imports)]
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program::invoke_signed,
    program_error::ProgramError, pubkey::Pubkey,
};
#[allow(unused_imports)]
use solana_sdk_ids::system_program;
#[allow(unused_imports)]
use solana_system_interface::instruction::create_account;

entrypoint!(process_instruction);

pub const PDA_SPACE: u64 = 4;
pub const FUNDING_LAMPORTS: u64 = 1_000_000;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let _ = (program_id, accounts, instruction_data);
    // TODO: validate the accounts and create the payer's PDA through a signed CPI.
    Err(ProgramError::InvalidInstructionData)
}
