# Rollback Guide

## Overview

This guide covers procedures for reversing or mitigating changes to the GHOST token program, including program upgrades, authority changes, and state modifications.

## 1. Upgrade Authority Rollback

### Scenario: Malicious or Buggy Program Upgrade

A new version of the program was deployed that contains a bug or malicious code.

#### Immediate Response

```bash
# 1. PAUSE the protocol (blocks minting)
# Use emergency authority to pause
node scripts/pause.js --authority emergency-authority.json

# 2. Identify previous program version
solana program show Gh0st111111111111111111111111111111111111111
# Record: Last Deployed Slot: <slot>
```

#### Rollback Steps

```bash
# Build the previous (known good) version
git checkout <previous-version-tag>  # e.g., v0.1.0
anchor build --verifiable

# Deploy previous version (overwrites current)
solana program deploy \
  target/deploy/ghost_token.so \
  --program-id target/deploy/ghost_token-keypair.json \
  --keypair upgrade-authority.json

# Verify rollback
solana program show Gh0st111111111111111111111111111111111111111
```

#### Post-Rollback

```bash
# 1. Verify all functionality
node scripts/verify-program.js

# 2. Test mint (small amount)
node scripts/mint.js --amount 1000

# 3. Test burn (small amount)
node scripts/burn.js --amount 500

# 4. Resume protocol
node scripts/resume.js --authority emergency-authority.json

# 5. Announce rollback publicly
```

### Upgrade Authority Rotation

If the upgrade authority key is compromised:

```bash
# Transfer upgrade authority to a new key
solana program set-upgrade-authority \
  Gh0st111111111111111111111111111111111111111 \
  --new-upgrade-authority <new-secure-pubkey> \
  --keypair <current-upgrade-authority.json>

# Verify transfer
solana program show Gh0st111111111111111111111111111111111111111
# Authority should now be: <new-secure-pubkey>
```

## 2. Immutable State Path

### Making the Program Immutable

This is the ultimate rollback protection — prevents ALL future upgrades:

```bash
# WARNING: IRREVERSIBLE
solana program set-upgrade-authority \
  Gh0st111111111111111111111111111111111111111 \
  --new-upgrade-authority void111111111111111111111111111111111111111
```

**Before making immutable, verify:**
- [ ] All security audits completed
- [ ] No known bugs
- [ ] Community vote/consensus achieved
- [ ] All desired features implemented
- [ ] Token economics finalized

#### Implications of Immutability

| Aspect | Before | After |
|--------|--------|-------|
| Bug fixes | Can deploy | Cannot fix |
| New features | Can add | Cannot add |
| Security patches | Can deploy | Vulnerable forever |
| Malicious takeover | Possible (if key compromised) | Impossible |
| Community trust | Depends on team | Maximum trust |

## 3. Authority Rotation Reversal

### Reversing a Pending Rotation

If an unauthorized authority rotation was initiated:

```bash
# Check if rotation is pending
node scripts/check-config.js
# Look for: pending_rotation: Some(...)

# Cancel the pending rotation (must be before timelock expires)
node scripts/cancel-rotation.js --authority admin-authority.json

# Verify cancellation
node scripts/check-config.js
# Look for: pending_rotation: None
```

### Reversing a Completed Rotation

If a rotation completed before you could cancel it:

```bash
# Initiate a new rotation BACK to the original authority
node scripts/initiate-rotation.js \
  --authority-type mint \
  --new-authority <original-mint-authority-pubkey> \
  --admin-authority admin-authority.json

# Wait for timelock

# Confirm the rotation
node scripts/confirm-rotation.js
```

### Emergency: Mint Authority Compromised

If the mint authority is compromised and actively minting:

```bash
# Step 1: PAUSE immediately
node scripts/pause.js --authority emergency-authority.json

# Step 2: Initiate rotation to new mint authority
node scripts/initiate-rotation.js \
  --authority-type mint \
  --new-authority <new-secure-mint-pubkey> \
  --admin-authority admin-authority.json

# Step 3: Wait for timelock to expire
# (Pause prevents confirmation during timelock — intentional)

# Step 4: Confirm rotation
node scripts/confirm-rotation.js

# Step 5: Resume protocol
node scripts/resume.js --authority emergency-authority.json

# Step 6: Assess damage (how many tokens were maliciously minted?)
spl-token supply <MINT_ADDRESS>
```

## 4. Metadata Rollback

If metadata was updated to incorrect values:

```bash
# Update metadata back to correct values
# Requires metadata authority key

spl-token initialize-metadata \
  <MINT_ADDRESS> \
  "GHOST" \
  "GHOST" \
  "https://arweave.net/placeholder-ghost-logo" \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Or update specific fields
spl-token update-metadata <MINT_ADDRESS> \
  name "GHOST" \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
```

## 5. Migration Reversal

### From V1 to V2 (Future)

If a V1→V2 migration needs reversal:

```bash
# 1. Pause all operations
node scripts/pause.js

# 2. Deploy V1 program version
git checkout v1.0.0
anchor build
solana program deploy target/deploy/ghost_token.so \
  --program-id target/deploy/ghost_token-keypair.json

# 3. Run migration reversal script (if data format changed)
node scripts/migrate-reverse.js

# 4. Verify V1 state is correct
node scripts/verify-state.js --version 1

# 5. Resume operations
node scripts/resume.js
```

## 6. Disaster Recovery

### Complete Program Recreation (Worst Case)

If the program must be completely redeployed at a NEW program ID:

```bash
# 1. Deploy new program
solana program deploy new_program.so --keypair new-program-keypair.json
# New program ID: <new-program-id>

# 2. Create new mint
spl-token create-token \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb \
  --decimals 9 \
  --mint-authority <mint-authority-pubkey>

# 3. Initialize with new program
node scripts/initialize.js --program-id <new-program-id>

# 4. COMMUNICATE to all users:
#    - Old token address: <old-mint-address>
#    - New token address: <new-mint-address>
#    - Migration instructions for holders
# 5. Coordinate with exchanges/DEXs for token swap
```

## 7. Rollback Decision Matrix

| Scenario | Action | Urgency | Reversibility |
|----------|--------|---------|---------------|
| Buggy program upgrade | Deploy previous version | High | Fully reversible |
| Malicious upgrade | Deploy previous version + investigate | Critical | Fully reversible |
| Compromised mint authority | Pause → rotate → resume | Critical | Fully reversible |
| Compromised admin authority | Rotate using current admin (if possible) | High | Medium |
| Compromised upgrade authority | Rotate upgrade authority | Critical | Reversible |
| Wrong metadata | Update metadata | Low | Fully reversible |
| Wrong initial supply | Cannot fix (Token-2022 supply is atomic) | N/A | Irreversible |
| Lost all authority keys | Cannot recover | Critical | Irreversible |

## 8. Prevention Measures

To minimize the need for rollbacks:

1. **Test thoroughly on devnet** before mainnet deployment
2. **Run all test suites** including fuzz tests
3. **Use timelocks** for authority rotations (24-hour minimum)
4. **Implement multisig** for all critical operations
5. **Keep emergency authority** in a hot wallet for rapid response
6. **Monitor on-chain activity** continuously
7. **Maintain build reproducibility** for verifiable rollbacks
8. **Document all deployments** with commit hashes and transaction signatures