Personal PDA Account Factory

Complete a native Solana program that creates one data account for each payer.
The account address is a PDA (program-derived address) of this program. The payer
does not create a second keypair for it.

Implement

Complete process_instruction in src/lib.rs. The entrypoint, imports, four-byte space
and funding constants, and local test setup are provided.

Rules

- The instruction receives three accounts in order: payer, target PDA, System Program.
- The payer must sign and be writable. The target PDA must be writable. Check that the
  third account is the System Program.
- Derive the expected PDA and canonical bump using [b"student", payer_pubkey.as_ref()]
  and the current program_id. Reject any other target address.
- Create the account with a System Program create_account CPI and invoke_signed. Use
  the fixed label, payer key, and bump as signer seeds. The PDA does not sign the
  outer transaction.
- Use the supplied FUNDING_LAMPORTS and PDA_SPACE. The new account must be owned by
  this program and have four zero-initialized bytes.
- Creating the same payer's PDA again must fail without changing the existing account.
  Propagate CPI failures as errors.
- Missing accounts or failed checks return errors rather than panicking. The instruction
  data is empty.

For example, Alice and Bob derive different accounts. Bob cannot initialize Alice's PDA
as his own. Alice cannot reinitialize an account she has already created.

Run

  cargo test --locked

The tests execute the program locally. No validator, RPC endpoint, SPL token, Anchor, or
custom address cryptography is needed.
