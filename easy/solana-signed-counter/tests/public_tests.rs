mod common;

use common::{count, instruction, send, setup};
use solana_signed_counter::CounterInstruction;

#[tokio::test]
async fn signed_counter_increases_and_decreases() {
    let (mut context, counter, program_id) = setup(3).await;
    let key = solana_sdk::signature::Signer::pubkey(&counter);
    send(
        &mut context,
        instruction(program_id, key, true, true, CounterInstruction::Increase),
        Some(&counter),
    )
    .await
    .unwrap();
    assert_eq!(count(&mut context, key).await, 4);
    send(
        &mut context,
        instruction(program_id, key, true, true, CounterInstruction::Decrease),
        Some(&counter),
    )
    .await
    .unwrap();
    assert_eq!(count(&mut context, key).await, 3);
}

#[tokio::test]
async fn unsigned_counter_cannot_change_state() {
    let (mut context, counter, program_id) = setup(3).await;
    let key = solana_sdk::signature::Signer::pubkey(&counter);
    let result = send(
        &mut context,
        instruction(program_id, key, false, true, CounterInstruction::Increase),
        None,
    )
    .await;
    assert!(result.is_err());
    assert_eq!(count(&mut context, key).await, 3);
}
