Web3 Contest 3

You have 150 minutes (2.5 hours). Complete three questions:

1. Choose one Easy question.
2. Choose one Medium question. This choice is independent of your Easy choice.
3. Complete the CEX question.

Each folder is a separate project. Read README.txt in that folder. Edit the marked
source file, then run the tests from that folder.

Easy — choose one:

- Wallet Portfolio Builder (Rust): easy/rust-wallet-portfolio
- Signed Counter Program (Solana): easy/solana-signed-counter

Medium — choose one:

- Channel-Based Balance Service (Rust): medium/rust-channel-balances
- Personal PDA Account Factory (Solana): medium/solana-pda-account-factory

CEX — required:

- Mini Exchange: Limit Orders and Fills: cex/limit-order-matcher

Run cargo test --locked from the folder of a Rust or Solana question. Run bun test
from the CEX folder.

The Rust projects need a Rust toolchain. The CEX project needs Bun. The Solana tests run
locally through the included test code; you do not need a wallet, validator, RPC
endpoint, or devnet SOL.

Starter code compiles, but tests fail until you finish the marked work. You may add your
own tests while working. The supplied public tests show basic behavior; the written
rules define the full task.
