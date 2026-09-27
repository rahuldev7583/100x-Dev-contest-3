use futures::channel::oneshot;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

use crate::BalanceMessage::{Credit, Read};

// Complete the three functions marked with todo! using the rules in README.txt.

pub enum BalanceMessage {
    Credit {
        user_id: u32,
        amount: u64,
    },
    Read {
        user_id: u32,
        reply: oneshot::Sender<u64>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceError {
    WorkerStopped,
    ReplyCanceled,
}

#[derive(Clone)]
pub struct BalanceService {
    sender: Sender<BalanceMessage>,
}

impl BalanceService {
    // Supplied so tests can exercise a disconnected request channel.
    pub fn from_sender(sender: Sender<BalanceMessage>) -> Self {
        Self { sender }
    }

    pub fn credit(&self, user_id: u32, amount: u64) -> Result<(), ServiceError> {
        let _ = (user_id, amount);
        self.sender.send(BalanceMessage::Credit { user_id, amount });
        Ok(())
    }

    pub async fn get_balance(&self, user_id: u32) -> Result<u64, ServiceError> {
        let _ = user_id;

        Ok(())
    }
}

pub fn start_service() -> (BalanceService, JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();
    let worker = thread::spawn(move || run_worker(receiver));

    (BalanceService { sender }, worker)
}

// Supplied: a caller may cancel a read while the worker is replying.
pub fn deliver_reply(reply: oneshot::Sender<u64>, balance: u64) {
    let _ = reply.send(balance);
}

pub fn run_worker(receiver: Receiver<BalanceMessage>) {
    let rc = receiver;
    //todo!("process requests until the channel closes")

    println!("rc: {:?}", rc);

    let msg = rc.recv();

    match msg {
        Ok(bal_msg) => match bal_msg {
            Credit { user_id, amount } => todo!(),
            Read { user_id, reply } => todo!(),
        },
        Err(_) => todo!(),
    }
}
