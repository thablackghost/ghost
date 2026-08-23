# Fuzzing Documentation

## Overview

Fuzz testing feeds random, unexpected, or invalid data to the GHOST token program to discover bugs, crashes, and security vulnerabilities that structured testing might miss.

## Fuzzing Architecture

```
┌──────────────────────────────────────────┐
│              Fuzz Harness                 │
│                                           │
│  ┌─────────┐   ┌────────────┐            │
│  │ Random  │──►│ Instruction │──► Result  │
│  │ Input   │   │  Handler    │    (Panic, │
│  │ Gen     │   │  (ghost)    │     Error, │
│  └─────────┘   └────────────┘     Ok)    │
│                                           │
│  Fuzzer: cargo-fuzz (libfuzzer)           │
│  Coverage-guided mutation                 │
└──────────────────────────────────────────┘
```

## Fuzz Targets

### Target 1: Instruction Deserialization

**File:** `fuzz/fuzz_targets/instruction_deser.rs`

**Purpose:** Ensure program never panics on arbitrary instruction data.

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Attempt to deserialize any byte sequence
    // Should return Err, never panic
    let _ = ghost_token::instruction::GhostInstruction::try_from_slice(data);
});
```

**Invariants:**
- No panics
- Invalid data returns `Err`
- Valid data returns `Ok`

### Target 2: Mint Operation

**File:** `fuzz/fuzz_targets/mint_operation.rs`

```rust
fuzz_target!(|input: (u64, bool, bool)| {
    let (amount, signed, paused) = input;
    // Simulate mint with varying parameters
    // Verify:
    // - If paused, always fails
    // - If not signed, always fails
    // - Valid case succeeds
});
```

### Target 3: Burn Operation

**File:** `fuzz/fuzz_targets/burn_operation.rs`

```rust
fuzz_target!(|input: (u64, u64)| {
    let (amount, balance) = input;
    // Simulate burn
    // Verify: amount <= balance succeeds, amount > balance fails
});
```

### Target 4: Authority Rotation

**File:** `fuzz/fuzz_targets/authority_rotation.rs`

```rust
fuzz_target!(|input: (u8, u64, u64)| {
    let (auth_type, current_time, effective_at) = input;
    // Verify timelock logic
    // current_time >= effective_at → allow confirm
    // current_time < effective_at → reject confirm
});
```

### Target 5: State Transition

**File:** `fuzz/fuzz_targets/state_transition.rs`

```rust
fuzz_target!(|input: (u8, u8, bool)| {
    let (from_state, operation, paused) = input;
    // Verify state machine transitions
    // Only valid transitions succeed
});
```

### Target 6: PDA Derivation

**File:** `fuzz/fuzz_targets/pda_derivation.rs`

```rust
fuzz_target!(|seeds: Vec<Vec<u8>>| {
    // Test PDA derivation with random seeds
    // Verify no collisions with different seeds
    // Verify PDAs are off-curve
});
```

### Target 7: Config Serialization

**File:** `fuzz/fuzz_targets/config_serde.rs`

```rust
fuzz_target!(|data: &[u8]| {
    // Attempt to deserialize arbitrary bytes as Config
    // Should not panic, should return Err for invalid data
    let _ = ghost_token::state::Config::try_from_slice(data);
});
```

### Target 8: Account Validation

**File:** `fuzz/fuzz_targets/account_validation.rs`

```rust
fuzz_target!(|input: (u64, u8, u8, u8)| {
    // Random account metadata (owner, writable, signer combos)
    // Verify all invalid combos are rejected
});
```

### Target 9: Amount Arithmetic

**File:** `fuzz/fuzz_targets/amount_arithmetic.rs`

```rust
fuzz_target!(|input: (u64, u64, u8)| {
    let (a, b, op) = input;
    match op % 3 {
        0 => { let _ = a.checked_add(b); }    // Should never panic
        1 => { let _ = a.checked_mul(b); }    // Should never panic
        _ => { let _ = a.checked_sub(b); }    // Returns None if underflow
    }
});
```

### Target 10: CPI Invocation

**File:** `fuzz/fuzz_targets/cpi_invocation.rs`

```rust
fuzz_target!(|program_ids: Vec<[u8; 32]>| {
    // Random program IDs
    // Verify only Token-2022 and System program are accepted
});
```

## Running Fuzz Tests

### Prerequisites

```bash
# Install cargo-fuzz
cargo install cargo-fuzz

# Ensure nightly toolchain (required by libfuzzer)
rustup install nightly
```

### List All Targets

```bash
cd programs/ghost-token
cargo fuzz list
```

Output:
```
instruction_deser
mint_operation
burn_operation
authority_rotation
state_transition
pda_derivation
config_serde
account_validation
amount_arithmetic
cpi_invocation
```

### Run Individual Target

```bash
# Run for 60 seconds
cargo fuzz run instruction_deser -- -max_total_time=60

# Run with specific seed (reproduce a crash)
cargo fuzz run instruction_deser artifacts/instruction_deser/crash-xxx

# Run with ASAN (Address Sanitizer)
RUSTFLAGS="-Z sanitizer=address" cargo +nightly fuzz run instruction_deser
```

### Run All Targets (Script)

```bash
#!/bin/bash
# fuzz-all.sh — Run all fuzz targets
for target in $(cargo fuzz list); do
  echo "Fuzzing: $target"
  timeout 300 cargo fuzz run "$target" -- -max_total_time=300
  echo "Completed: $target"
done
```

### Run in CI

```bash
# CI runs with limited time per target
cargo fuzz run instruction_deser -- -max_total_time=120 -rss_limit_mb=2048
```

## Interpreting Results

### Normal Output

```
#12345: cov: 5423 ft: 892 corp: 234/45Kb exec/s: 1234 rss: 67Mb
#67890: cov: 5430 ft: 895 corp: 245/46Kb exec/s: 1240 rss: 68Mb
```

- `cov`: Code coverage blocks reached
- `ft`: Features (unique code paths)
- `corp`: Corpus size (inputs saved)
- `exec/s`: Executions per second
- No crashes found (good!)

### Crash Found

```
==12345== ERROR: AddressSanitizer: heap-buffer-overflow
READ of size 8 at 0x7f1234567890
...
Fuzzing: instruction_deser
Crash: artifacts/instruction_deser/crash-abc123
```

**Response:**
1. Save the crash input: `artifacts/instruction_deser/crash-abc123`
2. Reproduce: `cargo fuzz run instruction_deser artifacts/instruction_deser/crash-abc123`
3. Debug the crash
4. Fix the root cause
5. Add the crash input as a regression test
6. Re-run fuzzing to verify fix

### Timeout

```
ALARM: working on the last Unit for X seconds
```

**Response:**
- The fuzzer found an input that causes the program to hang
- This is a bug — infinite loops or excessive computation
- Reproduce and fix

### OOM (Out of Memory)

```
==12345== ERROR: libFuzzer: out-of-memory
```

- Reduce `-rss_limit_mb` or fix memory leak
- Check for unbounded allocations

## Coverage Analysis

```bash
# Generate coverage report from fuzzing corpus
cargo fuzz coverage instruction_deser

# Convert to HTML
llvm-cov show -format=html \
  target/x86_64-unknown-linux-gnu/coverage/instruction_deser \
  -instr-profile=coverage.profdata \
  > coverage.html
```

## Corpus Management

### Minimize Corpus

```bash
# Reduce corpus to minimal set that preserves coverage
cargo fuzz cmin instruction_deser
```

### Merge Corpora

```bash
# Merge corpora from multiple fuzzing runs
cargo fuzz merge instruction_deser corpus1/ corpus2/
```

## Continuous Fuzzing

### Recommended Schedule

| Frequency | Target | Duration |
|-----------|--------|----------|
| Every PR | Top 3 targets | 5 min each |
| Nightly | All targets | 30 min each |
| Weekly | All + ASAN/UBSAN | 2 hours each |
| Pre-release | All targets | 24 hours continuous |

### CI Integration

```yaml
# In .github/workflows/fuzz.yml
fuzz:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: nightly
    - name: Install cargo-fuzz
      run: cargo install cargo-fuzz
    - name: Fuzz instruction_deser
      run: cargo fuzz run instruction_deser -- -max_total_time=120
    - name: Fuzz mint_operation
      run: cargo fuzz run mint_operation -- -max_total_time=120
```

## Known Limitations

1. **No BPF runtime**: Fuzz targets run natively (x86), not in BPF. Runtime differences may exist.
2. **No Solana accounts**: Cannot fuzz full Solana account model. Tests mock accounts.
3. **No Cross-Program**: CPIs are mocked, not real.
4. **Coverage gaps**: Some branches may be unreachable in native mode.

## Alternative Fuzzing Tools

| Tool | Pros | Cons |
|------|------|------|
| `cargo-fuzz` (libfuzzer) | Integrated, fast | Native only, no BPF |
| `trident` | Anchor-native, Solana-aware | Newer, smaller community |
| `honggfuzz` | Hardware-assisted | Complex setup |
| `afl.rs` | AFL algorithm | Slower on Rust |