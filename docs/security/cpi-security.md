# CPI Security

## Cross-Program Invocation Model

The GHOST token program performs Cross-Program Invocations (CPI) to interact with the SPL Token-2022 program and the System Program. All CPIs follow strict security rules.

## Validated Program IDs

Only two external programs are ever invoked via CPI:

```rust
pub const TOKEN_2022_PROGRAM_ID: &Pubkey = &spl_token_2022::ID;
pub const SYSTEM_PROGRAM_ID: &Pubkey = &solana_program::system_program::ID;
```

These are compile-time constants — **no dynamic program ID selection is allowed**.

| Program | Purpose | Invoked By |
|---------|---------|------------|
| `spl_token_2022` | Mint tokens, burn tokens, initialize mint | Initialize, MintTokens, BurnTokens |
| `system_program` | Create Config PDA account | Initialize |

## Account Validation Before CPI

Every account passed to a CPI is validated BEFORE the invocation:

### 1. Expected Accounts Pattern

```rust
// ✅ CORRECT: Validate all accounts before CPI
fn mint_tokens(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let mint = next_account_info(account_iter)?;
    let token_account = next_account_info(account_iter)?;
    let authority = next_account_info(account_iter)?;
    let config = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;

    // Validate BEFORE CPI
    require!(authority.is_signer, MissingSigner);
    require!(authority.key == config_data.mint_authority, UnauthorizedMinter);
    require!(!config_data.paused, Paused);
    require!(mint.key == config_data.mint, InvalidMint);
    require!(token_program.key == &spl_token_2022::ID, InvalidTokenProgram);

    // Now safe to CPI
    invoke(&mint_to_instruction, &[mint, token_account, authority])?;
    Ok(())
}
```

```rust
// ❌ WRONG: No validation before CPI
fn mint_tokens_unchecked(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let mint = &accounts[0];
    let token_account = &accounts[1];
    // MISSING: authority check, pause check, mint check
    invoke(&mint_to_instruction, &[mint, token_account, &accounts[2]])?;
    Ok(())
}
```

### 2. No Arbitrary Program Invocation

The program never uses a user-supplied program ID for CPI:

```rust
// ❌ FORBIDDEN: User supplies program ID
// let user_program = next_account_info(account_iter)?;
// invoke(&instruction, &[..., user_program])?;

// ✅ CORRECT: Program IDs are constants
spl_token_2022::mint_to(
    token_program.key,  // must match spl_token_2022::ID (validated above)
    mint.key,
    token_account.key,
    authority.key,
    &[],
    amount,
)?;
```

### 3. Account Ownership Chain

```
User (signer)
    │
    ▼
Ghost Program (program_id)
    │  CPI via invoke_signed with PDA seeds
    ▼
SPL Token-2022 Program (spl_token_2022::ID)
    │
    ▼
Token Mint Account (owned by Token-2022)
Token Account (owned by Token-2022)
```

### 4. PDA Signing

When the program needs to authorize CPI operations, it signs using PDA seeds:

```rust
let seeds: &[&[&[u8]]] = &[&[b"mint-authority", &[bump]]];
spl_token_2022::mint_to(
    token_program.key,
    mint.key,
    token_account.key,
    mint_authority_pda.key,  // This account signs via PDA
    seeds,
    amount,
)?;
```

## CPI Invocation Details

### Mint CPI

```rust
// Instruction construction
let mint_to_ix = spl_token_2022::instruction::mint_to(
    &spl_token_2022::ID,
    mint.key,
    token_account.key,
    authority.key,  // mint_authority (PDA or signer key)
    &[],
    amount,
)?;

// Invocation
invoke_signed(
    &mint_to_ix,
    &[mint.clone(), token_account.clone(), authority.clone()],
    seeds,  // PDA signer seeds if needed
)?;
```

### Burn CPI

```rust
let burn_ix = spl_token_2022::instruction::burn(
    &spl_token_2022::ID,
    token_account.key,
    mint.key,
    owner.key,  // token account owner
    &[],
    amount,
)?;

invoke(
    &burn_ix,
    &[token_account.clone(), mint.clone(), owner.clone()],
)?;
```

### Initialize Mint CPI

```rust
let init_mint_ix = spl_token_2022::instruction::initialize_mint(
    &spl_token_2022::ID,
    mint.key,
    mint_authority.key,  // PDA
    None,                // freeze authority = None
    9,                   // decimals
)?;

invoke_signed(
    &init_mint_ix,
    &[mint.clone(), rent.clone()],
    &[&[b"mint-authority", &[bump]]],
)?;
```

## Error Handling

All CPI calls return `ProgramResult`. Errors are not swallowed:

```rust
// ✅ Propagate errors
spl_token_2022::mint_to(
    token_program.key,
    mint.key,
    token_account.key,
    authority.key,
    &[],
    amount,
)?;  // Error from Token-2022 propagates up

// ❌ Never silently ignore CPI errors
// let _ = invoke(&ix, &accounts); // BAD
```

## CPI Security Checklist

- [ ] All CPI program IDs are compile-time constants
- [ ] No user-supplied program IDs for CPI targets
- [ ] All accounts validated before CPI invocation
- [ ] PDA seeds verified (bump included)
- [ ] Signer requirements enforced before CPI
- [ ] CPI errors propagated, not swallowed
- [ ] No re-entrancy possible (single-level CPI)
- [ ] Account ownership verified for all CPI accounts
- [ ] Token-2022 program ID matches expected value
- [ ] No delegate or close-authority set unexpectedly

## Known CPI Risks

### 1. Re-entrancy via Token-2022

Token-2022 supports transfer hooks which could theoretically call back into ghost-token. **Mitigation:** Ghost token does not implement transfer hooks and does not call transfer instructions. Only `mint_to` and `burn` are used, neither of which invoke transfer hooks.

### 2. Account Data Mismatch

If an attacker supplies a mint account that is owned by Token-2022 but has malicious data, CPI might succeed on wrong state. **Mitigation:** Mint address is verified against Config before CPI. Config PDA is immutable for the mint address after initialization.

### 3. Missing Account Checks

CPI will fail at the Token-2022 level if accounts are wrong, but this is a last-resort check. **Mitigation:** Pre-validation means errors are caught with clear custom error messages before CPI.