#![allow(dead_code)]

use solana_pda_account_factory::process_instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest, ProgramTestContext};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    signature::{keypair_from_seed, Keypair, Signer},
    transaction::Transaction,
};
use solana_sdk_ids::system_program;

pub async fn setup(payers: &[(&Keypair, u64)]) -> (ProgramTestContext, Pubkey) {
    let program_id = Pubkey::new_from_array([42; 32]);
    let mut test = ProgramTest::new(
        "pda_account_factory",
        program_id,
        processor!(process_instruction),
    );
    for (payer, lamports) in payers {
        test.add_account(
            payer.pubkey(),
            Account {
                lamports: *lamports,
                data: vec![],
                owner: system_program::id(),
                executable: false,
                rent_epoch: 0,
            },
        );
    }
    (test.start_with_context().await, program_id)
}

pub fn test_payer() -> Keypair {
    keypair_from_seed(&[7; 32]).unwrap()
}

pub fn expected_pda() -> Pubkey {
    "3YHXpKQHmJAawKQqJ47TpUEm8dJAEjRYrwJh7RGStGGa"
        .parse()
        .unwrap()
}

pub fn instruction(program_id: Pubkey, payer: Pubkey, target: Pubkey) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(target, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: vec![],
    }
}

pub async fn send(
    context: &mut ProgramTestContext,
    instruction: Instruction,
    payer: Option<&Keypair>,
) -> Result<(), solana_program_test::BanksClientError> {
    let mut signers: Vec<&dyn Signer> = vec![&context.payer];
    if let Some(payer) = payer {
        signers.push(payer);
    }
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &signers,
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await
}
