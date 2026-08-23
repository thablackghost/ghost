# Testing Strategy

## Overview

The GHOST token program employs a comprehensive testing strategy to ensure correctness, security, and reliability. Testing is organized into multiple layers from unit tests to property-based fuzzing.

## Test Pyramid

```
                    ┌─────────────┐
                    │     Fuzz     │  10+ targets
                    │   Property   │  Randomized inputs
                    └─────────────┘
                  ┌─────────────────┐
                  │   Integration   │  50+ scenarios
                  │  & End-to-End   │  Full instruction flow
                  └─────────────────┘
                ┌───────────────────────┐
                │    Security Tests     │  30+ attack scenarios
                │   (Negative Tests)    │  Expected failures
                └───────────────────────┘
              ┌──────────────────────────────┐
              │         Unit Tests           │  100+ tests
              │  Isolated function testing   │  per instruction
              └──────────────────────────────┘
```

## Test Categories

### 1. Unit Tests

**Location:** `programs/ghost-token/src/` (Rust `#[cfg(test)]` modules)

**Scope:**
- Instruction deserialization (valid/invalid inputs)
- PDA derivation correctness
- State transition logic
- Validation helpers (signer checks, owner checks)
- Error code mapping

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_pda_derivation() {
        let program_id = Pubkey::from_str("Gh0st111111111111111111111111111111111111111").unwrap();
        let (pda, bump) = Pubkey::find_program_address(&[b"config"], &program_id);
        assert_ne!(pda, program_id); // PDA != program ID
        assert!(bump > 0 && bump <= 255);
    }

    #[test]
    fn test_initialize_deserialization() {
        let data = Initialize { initial_supply: 10_000_000_000 };
        let serialized = data.try_to_vec().unwrap();
        let deserialized = Initialize::try_from_slice(&serialized).unwrap();
        assert_eq!(deserialized.initial_supply, 10_000_000_000);
    }

    #[test]
    fn test_invalid_instruction_rejected() {
        let data = vec![255u8]; // Invalid instruction tag
        let result = GhostInstruction::try_from_slice(&data);
        assert!(result.is_err());
    }
}
```

### 2. Integration Tests

**Location:** `tests/` (TypeScript/Anchor tests)

**Scope:**
- Full initialize → mint → burn lifecycle
- Authority rotation (initiate → confirm)
- Emergency pause and resume
- Multi-instruction sequences
- CPI interactions with Token-2022

**Example:**
```typescript
describe("GHOST Token Integration", () => {
  it("completes full lifecycle", async () => {
    // Initialize
    await program.methods.initialize(new BN(10_000_000_000))
      .accounts({...}).rpc();

    // Mint
    await program.methods.mintTokens(new BN(1_000_000))
      .accounts({...}).signers([mintAuthority]).rpc();

    // Burn
    await program.methods.burnTokens(new BN(500_000))
      .accounts({...}).signers([tokenOwner]).rpc();

    // Verify supply
    const mint = await getMint(connection, mintAddress);
    expect(mint.supply).to.equal(new BN(10_000_000_000 + 1_000_000 - 500_000));
  });
});
```

### 3. Security Tests (Negative Tests)

**Location:** `tests/security/`

**Scope:**
- Unauthorized mint attempts (wrong signer, no signer)
- PDA collision attempts (fake config accounts)
- Account substitution attacks
- Pause enforcement (mint while paused, rotation while paused)
- Timelock bypass attempts
- Invalid account ownership
- Replay attempts (expired blockhash)
- Arithmetic edge cases (max u64, zero amounts)

### 4. Property-Based Tests

**Location:** `programs/ghost-token/tests/proptest/` or `tests/property/`

**Scope:**
- "Minting increases supply by exactly the minted amount"
- "Burning decreases supply by exactly the burned amount"
- "Mint + Burn of same amount equals original supply"
- "Authority rotation preserves all other config fields"
- "Pause does not affect non-mint operations"

**Framework:** `proptest` (Rust) or `fast-check` (TypeScript)

### 5. Fuzz Tests

**Location:** `fuzz/`

**Scope:**
- Instruction data fuzzing (generate random bytes → expect no panics)
- Account list fuzzing (random account permutations)
- State fuzzing (random valid/invalid states → check invariants)

**Framework:** `cargo-fuzz` (libfuzzer) or `trident` (Anchor-native)

### 6. Invariant Tests

**Scope:**
- "Supply can never be negative"
- "Config.mint can never change after initialize"
- "Config.paused only changes via Pause/Resume"
- "Only one pending rotation at a time"
- "Timelock cannot be bypassed"
- "PDA bump never changes"

### 7. Regression Tests

**Location:** `tests/regression/`

**Scope:**
- Tests for previously fixed bugs
- Tests for edge cases discovered during audits
- Tests for specific transaction sequences that caused issues

## Test Environment

### Local Development

```bash
# Run all tests
anchor test

# Run with specific validator features
anchor test -- --features "test-bpf"

# Run specific test file
anchor test -- --test-path-pattern integration
```

### CI Environment

```bash
# Full test suite (CI)
cargo test-bpf
cargo test
yarn test
yarn test:security
yarn test:integration
```

## Test Coverage Targets

| Category | Target Coverage | Measurement |
|----------|----------------|-------------|
| Line coverage | > 90% | `cargo tarpaulin` |
| Branch coverage | > 80% | `cargo tarpaulin` |
| Instruction coverage | 100% (all 8 instructions) | Manual review |
| Error path coverage | 100% (all error variants tested) | Manual review |
| Authority matrix | 100% (all authority combinations) | Test matrix |

## Test Data Management

### Fixtures

```json
// tests/fixtures/test-authorities.json
{
  "mint_authority": [1,2,3,...,64],
  "admin_authority": [65,66,67,...,128],
  "emergency_authority": [129,130,131,...,192],
  "token_owner": [193,194,195,...,256]
}
```

### Deterministic Keypairs

Tests use deterministic keypairs derived from known seeds to ensure reproducibility:

```rust
let seed = b"test-mint-authority-001";
let keypair = Keypair::from_seed(seed).unwrap();
```

## Running Tests

### Quick Test (Smoke)

```bash
anchor test --skip-build --skip-deploy
```

### Full Test Suite

```bash
# Sequential (slower, more reliable on CI)
anchor test

# Parallel (faster, requires independent test state)
anchor test -- --test-threads 4
```

### Security Tests Only

```bash
anchor test -- --test-path-pattern security
```

### Fuzz Tests

```bash
cd programs/ghost-token
cargo fuzz list          # List fuzz targets
cargo fuzz run target1   # Run specific target
cargo fuzz run target1 -- -max_total_time=300  # 5 minutes
```

### Coverage Report

```bash
cargo tarpaulin --out Html --output-dir coverage
open coverage/tarpaulin-report.html
```

## CI Integration

All tests run on every PR via GitHub Actions:

- **`ci.yml`**: Lint + unit tests + Anchor build
- **`tests.yml`**: Full integration test suite
- **`security.yml`**: Security + fuzz tests
- **`coverage.yml`**: Coverage report generation

## Test Writing Guidelines

1. **Name tests descriptively**: `test_mint_rejected_when_paused` not `test_mint_2`
2. **One assertion concept per test**: Easier to diagnose failures
3. **Test the happy path first**: Establish baseline correctness
4. **Then test every error path**: Every `require!` should have a test
5. **Test boundary values**: 0, 1, max, max-1 for numeric inputs
6. **Test transaction ordering**: Instructions in different sequences
7. **Clean up state between tests**: Use fresh validator or reset state

## Test Matrix

### Authority Combination Matrix

| Test | Mint Auth | Admin Auth | Emergency Auth | Other | Expected |
|------|-----------|------------|----------------|-------|----------|
| Mint (valid) | ✅ | — | — | — | ✅ Success |
| Mint (wrong signer) | ❌ | ✅ | — | — | ❌ UnauthorizedMinter |
| Mint (unsigned) | — | — | — | — | ❌ MissingSigner |
| Initiate Rotation | — | ✅ | — | — | ✅ Success |
| Initiate Rotation (mint auth) | ✅ | ❌ | — | — | ❌ UnauthorizedAdmin |
| Pause | — | — | ✅ | — | ✅ Success |
| Pause (mint auth) | ✅ | — | ❌ | — | ❌ UnauthorizedEmergency |
| Mint while paused | ✅ | — | — | — | ❌ Paused |
| Burn while paused | — | — | — | ✅ (owner) | ✅ Success |