# Upgrade Authority

## Overview

The **Upgrade Authority** controls the ability to deploy new versions of the GHOST token program. This is a Solana-native mechanism managed by the BPF Loader, not by the GHOST token program itself.

## Solana Program Upgrade Model

### How Program Deployment Works

```
BPFLoaderUpgradeab1e11111111111111111111111 (System Program)
    │
    ├── Program Account (executable, read-only)
    │   Address: Gh0st111111111111111111111111111111111111111
    │   Owner: BPFLoaderUpgradeab1e11111111111111111111111
    │   Data: Pointer to ProgramData account
    │
    ├── ProgramData Account (contains bytecode)
    │   Address: <derived-from-program-address>
    │   Owner: BPFLoaderUpgradeab1e11111111111111111111111
    │   Data: Compiled BPF bytecode
    │   Slot: Last deployment slot
    │
    └── Buffer Account (temporary, during upgrades)
        Address: <ephemeral>
        Owner: BPFLoaderUpgradeab1e11111111111111111111111
        Data: New bytecode before deployment
```

### Key Commands

```bash
# Deploy initial program
solana program deploy target/deploy/ghost_token.so \
  --program-id target/deploy/ghost_token-keypair.json

# Upgrade existing program
solana program deploy target/deploy/ghost_token_v2.so \
  --program-id Gh0st111111111111111111111111111111111111111

# Check current upgrade authority
solana program show Gh0st111111111111111111111111111111111111111
# Output: Authority: <current-upgrade-authority>

# Rotate upgrade authority
solana program set-upgrade-authority Gh0st111111111111111111111111111111111111111 \
  --new-upgrade-authority <new-authority-pubkey>

# Make program IMMUTABLE (authority = None)
solana program set-upgrade-authority Gh0st111111111111111111111111111111111111111 \
  --new-upgrade-authority void111111111111111111111111111111111111111
```

## Current State

| Property | Value |
|----------|-------|
| **Program ID** | `Gh0st111111111111111111111111111111111111111` |
| **Current Version** | v0.1.0 |
| **Upgrade Authority** | Deployer keypair (to be transferred to multisig) |
| **Immutable?** | No (upgradable) |

## Recommended Upgrade Authority Management

### Phase 1: Deployment (Current)
```
Upgrade Authority: Single keypair (deployer)
    Purpose: Initial deployment, early bug fixes
    Risk: Single point of failure
```

### Phase 2: Post-Audit
```
Upgrade Authority: Multisig (2-of-3 or 3-of-5)
    Purpose: Secure upgrades with team consensus
    Members: Core team members
    Platform: Squads / Serum Multisig
    Risk: Low (requires multiple key compromises)
```

### Phase 3: Maturity
```
Upgrade Authority: DAO-controlled multisig
    Purpose: Community-governed upgrades
    Members: Elected council or token-weighted voting
    Platform: Realms / Governance
    Risk: Governance attack (mitigated by timelocks)
```

### Phase 4: Immutability
```
Upgrade Authority: None (void111111111111111111111111111111111111111)
    Purpose: Maximum trust, no possible changes
    Trigger: Community vote + security audit completion
    Risk: Cannot fix future bugs
```

## Immutability Path

### When to Consider Immutability

| Condition | Status |
|-----------|--------|
| All planned features implemented | ✅ |
| External security audit completed | Required |
| No known bugs or vulnerabilities | Required |
| Tokenomics are finalized | Required |
| Community governance vote | Required |
| Minimum 6 months of stable operation | Recommended |
| Multiple independent security reviews | Recommended |

### Immutability Procedure

```bash
# 1. Announce intention publicly (minimum 30 days notice)
# 2. Hold community governance vote
# 3. If approved:

# Verify current state
solana program show Gh0st111111111111111111111111111111111111111

# Set authority to None
solana program set-upgrade-authority \
  Gh0st111111111111111111111111111111111111111 \
  --new-upgrade-authority void111111111111111111111111111111111111111 \
  --keypair upgrade-authority.json

# Verify immutability
solana program show Gh0st111111111111111111111111111111111111111
# Output: Authority: none or Authority: void111111111111111111111111111111111111111

# Verify program is still executable
solana program show Gh0st111111111111111111111111111111111111111
# Should show: Executable: Yes
```

### Verifying Immutability

The program is immutable when:
- `solana program show` displays `Authority: none` or `Authority: void111111111111111111111111111111111111111`
- Any `solana program deploy --program-id Gh0st111111111111111111111111111111111111111` command fails

## Upgrade Procedure

### Pre-Upgrade Checklist

```bash
# 1. All tests passing
cargo test
anchor test
cargo test-bpf

# 2. Fuzz tests complete
cargo fuzz run <targets>

# 3. Security audit of changes
# (internal review at minimum)

# 4. Verify build reproducibility
anchor build --verifiable

# 5. Deploy to devnet and test
# (see docs/deployment/devnet.md)

# 6. Announce upgrade with:
#    - Commit hash / version tag
#    - Changelog
#    - Migration steps (if any)
#    - Rollback plan
```

### Performing the Upgrade

```bash
# 1. Ensure program is NOT paused (upgrades are off-chain, unaffected by pause)
# 2. Deploy new version
solana program deploy target/deploy/ghost_token.so \
  --program-id Gh0st111111111111111111111111111111111111111 \
  --keypair upgrade-authority.json

# 3. Verify new version is active
solana program show Gh0st111111111111111111111111111111111111111
# Check: Last Deployed Slot updated

# 4. Run verification suite
node scripts/verify-program.js

# 5. Announce completion
```

## Security Considerations

### If Upgrade Authority is Compromised

**Impact: CRITICAL**
- Attacker can deploy malicious code
- All token holder funds at risk
- Program can be replaced entirely

**Response:**
```bash
# IMMEDIATE: Rotate upgrade authority
solana program set-upgrade-authority \
  Gh0st111111111111111111111111111111111111111 \
  --new-upgrade-authority <new-secure-pubkey> \
  --keypair <compromised-keypair>  # Must still have the key
```

**If the compromised key is lost:**
- Cannot rotate upgrade authority
- Program is stuck with compromised authority
- Consider deploying new program + token migration

### Protection Measures

| Measure | Description |
|---------|-------------|
| Multisig | Require multiple signatures for upgrades |
| Timelock | Use Squads timelock for multisig transactions |
| Offline keys | Store upgrade authority keys in cold storage |
| Monitoring | Alert on any program upgrade transaction |
| Verifiable builds | Ensure all upgrades are source-verifiable |

## Monitoring

Set up alerts for:

```
Event: Program upgrade transaction
    → Check Solana Explorer for new deployments
    → WebSocket subscription on program account
    → Alert: "Program Gh0st111111111111111111111111111111111111111 was upgraded at slot X"

Event: Upgrade authority change
    → WebSocket subscription on program data account
    → Alert: "Upgrade authority for Gh0st111... changed to <new-authority>"
```