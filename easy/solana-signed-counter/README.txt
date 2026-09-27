Signed Counter Program

Complete a native Solana program that changes a counter stored in an existing data
account. The test runner creates the account for you. It is a normal keypair-backed
account owned by this program, not a PDA.

Implement

Complete process_instruction in src/lib.rs. State and instruction types, Borsh
derives, the entrypoint, and local test setup are provided.

Rules

- Use the counter account supplied to the instruction. It must be owned by this program,
  writable, and a signer. A fee payer's signature alone is not enough.
- Decode the four-byte CounterState and the Borsh CounterInstruction.
- Increase adds exactly 1; Decrease subtracts exactly 1.
- Decreasing zero returns an error and leaves state unchanged.
- Write the updated state back to the same account.
- Each counter account keeps its own state. Updating one must not change another.
- Missing accounts, malformed data, or failed checks return an error rather than
  panicking.
- Valid test sequences stay below u32::MAX. If several inputs are invalid, the order
  in which you report errors is not graded.

For example, a signed counter at 3 becomes 4 after Increase, then 3 after
Decrease. An unsigned attempt must fail without changing it.

Run

  cargo test --locked

The tests execute the program in a local Solana runtime. No validator, RPC endpoint,
wallet setup, PDA, CPI, or SPL token code is needed.
