#![allow(unexpected_cfgs)]

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CounterState {
    pub count: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterInstruction {
    Increase,
    Decrease,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let _ = (program_id, accounts, instruction_data);
    // TODO: validate the account, update its counter, and write the state back.
    Err(ProgramError::InvalidInstructionData)
}
