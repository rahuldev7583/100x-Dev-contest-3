Channel-Based Balance Service

Complete a small balance service. One worker thread owns a map of user balances.
Callers send credit and read requests through an MPSC channel (many senders, one
receiver). Each read gets its answer through its own one-shot reply channel.

Implement

In src/lib.rs, complete run_worker, BalanceService::credit, and
BalanceService::get_balance. The message type, service startup, error type, reply
helper, and test executor are provided.

Rules

- The worker owns the only mutable balance map and processes messages one at a time.
- A credit adds to the existing balance. An unseen user starts at 0.
- A credit of 0 is valid. You may assume sums fit in u64.
- A read returns the balance when the worker processes that read. An unseen user reads
  as 0.
- Send each read result through that request's reply sender.
- The worker exits normally when all request senders are gone.
- Return ServiceError::WorkerStopped if sending a request fails. Return
  ServiceError::ReplyCanceled if a read's reply is canceled. Do not panic for these
  expected failures.
- If a read requester goes away before receiving its answer, the worker keeps processing
  later requests.
- A successful credit means the request was sent to the channel. It does not wait for
  a reply.

For example, credit user 7 by 20, then by 15, then read user 7: the result is
35. Reading user 99 gives 0.

Requests from different threads may arrive in any order. The tests synchronize sends
before asserting a final total.

Run

  cargo test --locked

No HTTP, database, withdrawal, or second worker is needed.
