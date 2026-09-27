mod common;

use common::{expected_pda, instruction, send, setup, test_payer};
use solana_pda_account_factory::FUNDING_LAMPORTS;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn creates_the_expected_program_owned_pda() {
    let alice = test_payer();
    let (mut context, program_id) = setup(&[(&alice, 2_000_000)]).await;
    let target = expected_pda();
    send(
        &mut context,
        instruction(program_id, alice.pubkey(), target),
        Some(&alice),
    )
    .await
    .unwrap();
    let account = context
        .banks_client
        .get_account(target)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(account.owner, program_id);
    assert_eq!(account.lamports, FUNDING_LAMPORTS);
    assert_eq!(account.data, vec![0; 4]);
}

#[tokio::test]
async fn rejects_the_wrong_pda() {
    let alice = test_payer();
    let (mut context, program_id) = setup(&[(&alice, 2_000_000)]).await;
    let wrong = Keypair::new().pubkey();
    assert!(send(
        &mut context,
        instruction(program_id, alice.pubkey(), wrong),
        Some(&alice)
    )
    .await
    .is_err());
    assert!(context
        .banks_client
        .get_account(wrong)
        .await
        .unwrap()
        .is_none());
}
