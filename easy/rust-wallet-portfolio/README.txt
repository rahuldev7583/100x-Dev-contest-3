Wallet Portfolio Builder

Build a token portfolio from a slice of entries. The same symbol may appear more than
once.

Implement

Complete build_portfolio and balance_of in src/lib.rs. The type and function
signatures are provided.

Rules

- Add amounts for repeated symbols. Keep different symbols separate.
- An empty input produces an empty map.
- An entry with amount 0 is valid. Keep its symbol in the map even if its total is
  0.
- A missing symbol has balance 0. Looking it up must not add it to the map.
- Do not change the input entries. Symbols are compared exactly, including letter case.
- You may assume each total fits in u64. Map iteration order does not matter.

For example, SOL 2, ETH 1, SOL 3 produces SOL 5 and ETH 1. Looking up BTC
returns 0.

Run

  cargo test --locked

This is an in-memory Rust question. No blockchain access, prices, files, or threads are
needed.
