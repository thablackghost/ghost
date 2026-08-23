# Sealevel & Solana Security

## Overview

The GHOST token program runs on Solana's Sealevel parallel runtime. Understanding Sealevel's execution model is critical for writing secure Solana programs.

## Account Validation

Solana programs must explicitly validate every account they receive. Unlike EVM where `msg.sender` is implicit, Solana programs receive raw `AccountInfo` structs and must check everything.

### Required Checks for Every Account

| Check | Purpose | Example |
|-------|---------|---------|
| **Signer** | Verify the account signed the transaction | `require!(account.is_signer, MissingSigner)` |
| **Writable** | Verify the account is writable (for modifications) | `require!(account.is_writable, AccountNotWritable)` |
| **Owner** | Verify the account is owned by the correct program | `require!(account.owner == expected_program, InvalidOwner)` |
| **PDA** | Verify the account is at the expected PDA | `require!(account.key == expected_pda, InvalidPDA)` |
| **Data** | Deserialize and validate account data | `Config::try_from_slice(&account.data.borrow())?` |

### The EVERY Account Rule

**Every account passed to a Solana program can be controlled by an attacker.** This is fundamental to Sealevel security:

```rust
// ❌ NEVER trust account data without validation
fn insecure_handler(accounts: &[AccountInfo]) -> ProgramResult {
    let config = &accounts[0];  // Could be ANY account!
    let config_data = Config::try_from_slice(&config.data.borrow())?;
    // Attacker could pass their own config with themselves as mint_authority
    mint_tokens(config_data.mint_authority)?;  // DISASTER
}

// ✅ ALWAYS validate PDAs
fn secure_handler(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let config = &accounts[0];
    let (expected_pda, _) = Pubkey::find_program_address(
        &[b"config"], program_id
    );
    require!(config.key == expected_pda, InvalidConfigPDA);
    let config_data = Config::try_from_slice(&config.data.borrow())?;
    // Config is now trusted
    mint_tokens(config_data.mint_authority)?;
}
```

## Signer Checks

A `signer` in Solana means the account's private key signed the transaction. This is the ONLY proof of authorization:

```rust
// Signer check is MANDATORY for authorization
pub fn process_mint(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_iter = &mut accounts.iter();

    let mint = next_account_info(account_iter)?;
    let token_account = next_account_info(account_iter)?;
    let authority = next_account_info(account_iter)?;  // Must be signer
    let config = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;

    // CRITICAL: Check signer
    require!(authority.is_signer, MissingSigner);

    let config_data = Config::try_from_slice(&config.data.borrow())?;
    require!(authority.key == config_data.mint_authority, UnauthorizedMinter);

    // Proceed with CPI...
    Ok(())
}
```

### Signer Validation Pattern

```
Account marked as "signer" in instruction
    │
    ▼
is_signer == true  ←─ Runtime enforces this
    │
    ▼
Check account.key matches expected authority  ←─ Program enforces this
    │
    ▼
Operation proceeds
```

## Writable Checks

Accounts that will be modified MUST be marked writable:

```rust
// For CPI that writes to token accounts
require!(mint.is_writable, AccountNotWritable);
require!(token_account.is_writable, AccountNotWritable);

// For Config updates
require!(config.is_writable, AccountNotWritable);
```

### Why This Matters

Solana's runtime uses the writable flag for parallel transaction scheduling. If you modify an account that isn't marked writable, the runtime may:
1. Silently discard the write (older versions)
2. Panic or error (newer versions)

**Always explicitly check `is_writable` before writing.**

## PDA Verification

Program Derived Addresses (PDAs) are addresses derived from the program ID and seeds:

```rust
let (pda, bump) = Pubkey::find_program_address(
    &[b"config"],
    program_id,
);
```

### PDA Security Properties

1. **Off-curve**: PDAs have no associated private key. Only the program can "sign" for them via `invoke_signed`.
2. **Deterministic**: Same seeds + program ID = same PDA. Guaranteed by SHA-256.
3. **Collision-resistant**: SHA-256 ensures no two seed sets produce the same PDA (for a given program).

### PDA Verification Pattern

```rust
fn verify_config_pda(config: &AccountInfo, program_id: &Pubkey) -> ProgramResult {
    let (expected_pda, expected_bump) = Pubkey::find_program_address(
        &[b"config"],
        program_id,
    );
    require!(config.key == expected_pda, InvalidConfigPDA);

    // Also verify stored bump
    let config_data = Config::try_from_slice(&config.data.borrow())?;
    require!(config_data.bump == expected_bump, InvalidConfigBump);

    Ok(())
}
```

## Owner Validation

Every account in Solana has an `owner` — the program that controls it:

```rust
// Token-2022 owns mint and token accounts
require!(mint.owner == &spl_token_2022::ID, InvalidMintOwner);
require!(token_account.owner == &spl_token_2022::ID, InvalidTokenAccountOwner);

// Ghost program owns the Config PDA
require!(config.owner == program_id, InvalidConfigOwner);

// System program owns uninitialized accounts
require!(new_account.owner == &system_program::ID, AccountAlreadyInitialized);
```

### Owner Validation Matrix

| Account Type | Expected Owner |
|-------------|---------------|
| Config PDA | `program_id` (ghost-token) |
| Mint Account | `spl_token_2022::ID` |
| Token Account | `spl_token_2022::ID` |
| System Account (for init) | `system_program::ID` |

## Common Sealevel Attack Vectors

### 1. Fake PDA
**Attack:** Attacker creates an account that looks like the Config PDA (same data format) but at a different address.
**Defense:** Always derive and verify PDA address.

### 2. Missing Signer Check
**Attack:** Attacker provides a non-signer account with the correct public key.
**Defense:** Always check `account.is_signer`.

### 3. Type Confusion
**Attack:** Attacker passes a valid Token-2022 account where a different account type is expected.
**Defense:** Verify owner, verify address matches expected, verify data structure.

### 4. Account Data Injection
**Attack:** Attacker crafts malicious account data that deserializes into a valid Config structure.
**Defense:** PDA verification ensures only the real config is used.

### 5. Clock Manipulation
**Attack:** Attacker passes a fake Clock sysvar account.
**Defense:** Use `Clock::get()?` which reads from the runtime, not from an account. Verify the account is the real sysvar if passed explicitly.

## Sysvar Security

### Clock Sysvar

```rust
// ✅ CORRECT: Read from runtime
let clock = Clock::get()?;

// ❌ WRONG: Read from user-supplied account
// let clock_data = &accounts[5]; // Could be spoofed
// let clock = Clock::from_account_info(clock_data)?;
```

### Rent Sysvar

```rust
// ✅ CORRECT: Read from runtime
let rent = Rent::get()?;
let min_balance = rent.minimum_balance(space);
```

## Transaction Atomicity

All instructions execute atomically within a single transaction. If any instruction fails:
- All state changes in the transaction are rolled back
- No partial state updates occur
- CPI failures also roll back the entire transaction

This is guaranteed by Solana's runtime and is not something the program needs to handle explicitly.