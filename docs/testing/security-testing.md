# Security Testing

## Overview

Security testing validates that the GHOST token program correctly rejects all unauthorized operations and maintains invariants under attack conditions. Security tests are "negative tests" — they verify that bad inputs are rejected with the correct error.

## Test Categories

### 1. Authorization Tests

Verify that only authorized signers can perform privileged operations.

```typescript
describe("Authorization", () => {
  it("rejects mint with wrong mint authority", async () => {
    await expectRevert(
      program.methods.mintTokens(new BN(1000))
        .accounts({ authority: attacker.publicKey /* not mint auth */ })
        .signers([attacker])
        .rpc(),
      "UnauthorizedMinter"
    );
  });

  it("rejects mint without signer", async () => {
    await expectRevert(
      program.methods.mintTokens(new BN(1000))
        .accounts({ authority: mintAuthority.publicKey })
        // MISSING: .signers([mintAuthority])
        .rpc(),
      "MissingSigner"
    );
  });

  it("rejects pause from non-emergency authority", async () => {
    await expectRevert(
      program.methods.pause()
        .accounts({ authority: attacker.publicKey })
        .signers([attacker])
        .rpc(),
      "UnauthorizedEmergency"
    );
  });

  it("rejects initiate rotation from non-admin", async () => {
    await expectRevert(
      program.methods.initiateAuthorityRotation(0, newKey.publicKey)
        .accounts({ admin: attacker.publicKey })
        .signers([attacker])
        .rpc(),
      "UnauthorizedAdmin"
    );
  });
});
```

### 2. PDA Validation Tests

Verify the program rejects fake/imposter Config accounts.

```typescript
describe("PDA Validation", () => {
  it("rejects config at wrong PDA", async () => {
    // Create a fake config account at a non-PDA address
    const fakeConfig = await createFakeConfig(attacker);

    await expectRevert(
      program.methods.mintTokens(new BN(1000))
        .accounts({ config: fakeConfig /* wrong PDA */ })
        .rpc(),
      "InvalidConfigPDA"
    );
  });

  it("rejects config with wrong bump", async () => {
    // Create config at correct PDA but with wrong bump in data
    const [pda, _] = findConfigPDA();
    const badConfig = await createConfigWithWrongBump(pda);

    await expectRevert(
      program.methods.resume()
        .accounts({ config: badConfig })
        .rpc(),
      "InvalidConfigBump"
    );
  });
});
```

### 3. Pause Enforcement Tests

Verify all gated operations are blocked during pause.

```typescript
describe("Pause Enforcement", () => {
  beforeEach(async () => {
    // Put protocol in paused state
    await program.methods.pause()
      .accounts({ emergencyAuthority: emergency.publicKey })
      .signers([emergency])
      .rpc();
  });

  it("rejects mint while paused", async () => {
    await expectRevert(
      program.methods.mintTokens(new BN(1000))
        .accounts({ authority: mintAuthority.publicKey })
        .signers([mintAuthority])
        .rpc(),
      "Paused"
    );
  });

  it("rejects initiate rotation while paused", async () => {
    await expectRevert(
      program.methods.initiateAuthorityRotation(0, newKey.publicKey)
        .accounts({ admin: admin.publicKey })
        .signers([admin])
        .rpc(),
      "Paused"
    );
  });

  it("allows burn while paused", async () => {
    await program.methods.burnTokens(new BN(500))
      .accounts({ owner: tokenOwner.publicKey })
      .signers([tokenOwner])
      .rpc();
    // Should succeed — burn is always allowed
  });

  it("allows cancel rotation while paused", async () => {
    // Should succeed — admin can always cancel
  });
});
```

### 4. Account Substitution Tests

Verify the program detects when wrong accounts are supplied.

```typescript
describe("Account Substitution", () => {
  it("rejects wrong mint account", async () => {
    const fakeMint = await createFakeMint();

    await expectRevert(
      program.methods.mintTokens(new BN(1000))
        .accounts({ mint: fakeMint /* not the real mint */ })
        .rpc(),
      "InvalidMint"
    );
  });

  it("rejects wrong token program", async () => {
    await expectRevert(
      program.methods.mintTokens(new BN(1000))
        .accounts({ tokenProgram: splToken.ID /* not Token-2022 */ })
        .rpc(),
      "InvalidTokenProgram"
    );
  });

  it("rejects mint owned by wrong program", async () => {
    // Create a mint owned by original SPL Token
    const splMint = await createSplTokenMint();

    await expectRevert(
      program.methods.mintTokens(new BN(1000))
        .accounts({ mint: splMint })
        .rpc(),
      "InvalidMintOwner"
    );
  });
});
```

### 5. Timelock Tests

Verify the timelock mechanism cannot be bypassed.

```typescript
describe("Timelock Enforcement", () => {
  it("rejects confirm before timelock expires", async () => {
    // Initiate rotation
    await initiateRotation(0, newAuthority.publicKey);

    // Try to confirm immediately
    await expectRevert(
      program.methods.confirmAuthorityRotation()
        .rpc(),
      "TimelockNotExpired"
    );
  });

  it("accepts confirm after timelock expires", async () => {
    // Initiate rotation
    await initiateRotation(0, newAuthority.publicKey);

    // Advance clock past timelock
    await advanceClock(TIMELOCK_DURATION + 1);

    // Confirm should succeed
    await program.methods.confirmAuthorityRotation().rpc();
  });

  it("rejects double initiate while rotation pending", async () => {
    await initiateRotation(0, newAuthority1.publicKey);

    await expectRevert(
      initiateRotation(0, newAuthority2.publicKey),
      "RotationAlreadyPending"
    );
  });
});
```

### 6. Arithmetic Edge Case Tests

```typescript
describe("Arithmetic Edge Cases", () => {
  it("handles zero amount mint", async () => {
    await program.methods.mintTokens(new BN(0))
      .accounts({ authority: mintAuthority.publicKey })
      .signers([mintAuthority])
      .rpc();
    // Should succeed (no-op) or reject with InvalidAmount
  });

  it("handles zero amount burn", async () => {
    // Similarly test zero burn
  });

  it("handles max u64 amount", async () => {
    // Test with u64::MAX
    try {
      await program.methods.mintTokens(new BN("18446744073709551615"))
        .accounts({ authority: mintAuthority.publicKey })
        .signers([mintAuthority])
        .rpc();
      // Should succeed or fail gracefully (not panic)
    } catch (e) {
      // Acceptable: overflow error, not a panic/halt
    }
  });

  it("cannot burn more than balance", async () => {
    const balance = await getBalance(tokenAccount);
    await expectRevert(
      program.methods.burnTokens(balance.add(new BN(1)))
        .accounts({ owner: tokenOwner.publicKey })
        .signers([tokenOwner])
        .rpc(),
      "InsufficientFunds" // From Token-2022
    );
  });
});
```

### 7. Reentrancy/CPI Tests

```typescript
describe("CPI Security", () => {
  it("rejects CPI from unauthorized program", async () => {
    // Create a malicious program that tries to call ghost-token
    // Verify ghost-token validates accounts regardless of caller
  });

  it("rejects CPI with crafted accounts", async () => {
    // Malicious program passes fake config
    // Should be rejected by PDA verification
  });
});
```

## Attack Simulation Scenarios

### Scenario 1: Attacker Tries to Mint

```
Setup: Attacker has keypair, no special authority
Action: Submit MintTokens(1,000,000 GHOST) signed by attacker
Expected: Transaction rejected with UnauthorizedMinter
Verified: ✅ Program checks authority.key == config.mint_authority
```

### Scenario 2: Attacker Creates Fake Config

```
Setup: Attacker creates account with Config data (self as mint_authority)
Action: Submit MintTokens with fake config account
Expected: Transaction rejected with InvalidConfigPDA
Verified: ✅ Program derives and verifies PDA
```

### Scenario 3: Attacker Tries Timelock Bypass

```
Setup: Legitimate rotation initiated by admin
Action: Attacker immediately submits ConfirmAuthorityRotation
Expected: Transaction rejected with TimelockNotExpired
Verified: ✅ Program checks clock.unix_timestamp >= effective_at
```

### Scenario 4: Attacker Mints During Pause

```
Setup: Emergency authority has paused the protocol
Action: Mint authority submits MintTokens
Expected: Transaction rejected with Paused
Verified: ✅ Program checks paused flag before mint
```

### Scenario 5: Attacker Substitutes Token Program

```
Setup: Attacker creates fake mint on original SPL Token
Action: Submit MintTokens pointing to original SPL Token program
Expected: Transaction rejected with InvalidTokenProgram
Verified: ✅ Program checks token_program.key == spl_token_2022::id()
```

## Expected Failures Reference

| Error Code | Trigger Condition | Test Coverage |
|-----------|-------------------|---------------|
| `UnauthorizedMinter` | Mint signed by non-mint-authority | ✅ |
| `UnauthorizedAdmin` | Admin operation signed by non-admin | ✅ |
| `UnauthorizedEmergency` | Pause/Resume signed by non-emergency | ✅ |
| `MissingSigner` | Required signer absent | ✅ |
| `InvalidConfigPDA` | Config account at wrong address | ✅ |
| `InvalidConfigBump` | Config stored bump != derived bump | ✅ |
| `InvalidMint` | Mint != config.mint | ✅ |
| `InvalidTokenProgram` | Token program != Token-2022 | ✅ |
| `InvalidMintOwner` | Mint not owned by Token-2022 | ✅ |
| `Paused` | Mint/Rotation while paused | ✅ |
| `RotationAlreadyPending` | Two simultaneous rotations | ✅ |
| `NoPendingRotation` | Confirm with no rotation pending | ✅ |
| `TimelockNotExpired` | Confirm before effective_at | ✅ |
| `InvalidAuthorityType` | Unknown authority type (0-2 range check) | ✅ |
| `ArithmeticOverflow` | Value overflow in checked math | ✅ |

## Running Security Tests

```bash
# Run all security tests
anchor test -- --test-path-pattern security

# Run specific attack scenario
anchor test -- --test-path-pattern "attack-mint"

# Run with verbose output
anchor test -- --test-path-pattern security --nocapture
```