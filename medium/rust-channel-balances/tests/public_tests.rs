use futures::executor::block_on;
use rust_channel_balances::start_service;

#[test]
fn credits_and_reads_one_user() {
    let (service, worker) = start_service();
    service.credit(7, 20).unwrap();
    service.credit(7, 15).unwrap();
    assert_eq!(block_on(service.get_balance(7)), Ok(35));
    drop(service);
    worker.join().unwrap();
}

#[test]
fn new_users_start_at_zero() {
    let (service, worker) = start_service();
    assert_eq!(block_on(service.get_balance(99)), Ok(0));
    drop(service);
    worker.join().unwrap();
}
