#![allow(dead_code)]

use borsh::BorshDeserialize;
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest, ProgramTestContext};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_signed_counter::{process_instruction, CounterInstruction, CounterState};

pub async fn setup(count: u32) -> (ProgramTestContext, Keypair, Pubkey) {
    let program_id = Pubkey::new_unique();
    let counter = Keypair::new();
    let mut test = ProgramTest::new(
        "signed_counter",
        program_id,
        processor!(process_instruction),
    );
    test.add_account(
        counter.pubkey(),
        Account {
            lamports: 1_000_000,
            data: borsh::to_vec(&CounterState { count }).unwrap(),
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    );
    (test.start_with_context().await, counter, program_id)
}

pub fn instruction(
    program_id: Pubkey,
    counter: Pubkey,
    signed: bool,
    writable: bool,
    action: CounterInstruction,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![AccountMeta {
            pubkey: counter,
            is_signer: signed,
            is_writable: writable,
        }],
        data: borsh::to_vec(&action).unwrap(),
    }
}

pub async fn send(
    context: &mut ProgramTestContext,
    instruction: Instruction,
    extra_signer: Option<&Keypair>,
) -> Result<(), solana_program_test::BanksClientError> {
    let mut signers: Vec<&dyn Signer> = vec![&context.payer];
    if let Some(signer) = extra_signer {
        signers.push(signer);
    }
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &signers,
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await
}

pub async fn count(context: &mut ProgramTestContext, key: Pubkey) -> u32 {
    let account = context
        .banks_client
        .get_account(key)
        .await
        .unwrap()
        .unwrap();
    CounterState::try_from_slice(&account.data).unwrap().count
}
