# Mint Authority

## Role Definition

The **Mint Authority** is the on-chain authority that controls the creation of new GHOST tokens. It is stored in the `Config.mint_authority` field on the Config PDA.

## Responsibilities

### Primary
- **Mint tokens**: Create new GHOST tokens and send them to any token account
- **Supply management**: Control the total circulating supply through minting

### Scope Limitations
- ❌ Cannot burn tokens (burning is permissionless for token holders)
- ❌ Cannot rotate authorities (this is the admin's role)
- ❌ Cannot pause/resume the protocol (this is the emergency authority's role)
- ❌ Cannot update token metadata (this is the metadata authority's role)

## Security Considerations

### Risk Level: CRITICAL

The mint authority is the most sensitive key in the GHOST token ecosystem:

| If Compromised | Impact |
|----------------|--------|
| Attacker can mint unlimited tokens | Infinite inflation |
| Attacker can dump newly minted tokens | Market manipulation |
| Attacker can dilute all existing holders | Permanent value loss |
| Attacker can mint to any address without detection | Stealth inflation |

### Protection Requirements

#### Key Storage
```
Recommended: Hardware Security Module (HSM)
  → Ledger Nano X / Trezor Model T
  → Cold storage (air-gapped device)
  → Encrypted offline backup
  → Split key (MPC/SSS) in institutional setting

Required: NEVER on internet-connected machine
  → No cloud storage
  → No email attachments
  → No messaging apps
  → No version control
```

#### Operational Security

1. **Transaction signing**: Air-gapped machine only
2. **Minting schedule**: Pre-announced, batched operations
3. **Amount limits**: Internal soft cap per mint operation
4. **Multisig**: Transition to multisig (e.g., Squads) for production
5. **Rotation**: Rotate quarterly at minimum
6. **Monitoring**: Real-time alerts for any mint transaction

### Compromise Response

If the mint authority is compromised:

```
IMMEDIATE:
1. Emergency authority → PAUSE protocol
2. Verify compromise (check recent mint transactions)
3. Assess damage (how many tokens were minted?)

RECOVERY:
4. Admin authority → Initiate mint authority rotation
5. Wait for timelock to expire
6. Confirm rotation to new secure key
7. Emergency authority → RESUME protocol
8. Public announcement with timeline and impact assessment
```

## How to Rotate the Mint Authority

### Overview

Mint authority rotation is a **two-step process** initiated by the admin authority with a mandatory timelock.

### Step 1: Initiate Rotation

```bash
# Admin authority initiates rotation
node scripts/initiate-rotation.js \
  --authority-type mint \
  --new-authority <NEW_MINT_AUTHORITY_PUBKEY> \
  --admin-keypair admin-authority.json \
  --url https://api.mainnet-beta.solana.com
```

This transaction:
1. Verifies the signer is the current admin authority
2. Checks no rotation is already pending
3. Sets `config.pending_rotation` to:
   - `authority_type: 0` (mint)
   - `new_authority: <NEW_MINT_AUTHORITY_PUBKEY>`
   - `effective_at: current_timestamp + TIMELOCK_DURATION`
4. Emits `AuthorityRotationInitiated` event

### Step 2: Wait for Timelock

The timelock period must expire before confirmation:
- **Devnet:** 60 seconds (for testing)
- **Mainnet:** 86,400 seconds (24 hours) recommended

During the timelock:
- Protocol remains operational (minting continues)
- Old mint authority can still mint
- New mint authority cannot mint yet
- Anyone can confirm after timelock expires
- Admin can cancel the rotation at any time

### Step 3: Confirm Rotation

```bash
# Anyone can confirm after timelock expires
node scripts/confirm-rotation.js \
  --url https://api.mainnet-beta.solana.com
```

This transaction:
1. Verifies a rotation is pending
2. Verifies timelock has expired (`clock.unix_timestamp >= effective_at`)
3. Sets `config.mint_authority = pending.new_authority`
4. Clears `config.pending_rotation = None`
5. Emits `AuthorityRotationConfirmed` event

### Cancel Rotation (If Needed)

```bash
# Admin can cancel at any time before confirmation
node scripts/cancel-rotation.js \
  --admin-keypair admin-authority.json \
  --url https://api.mainnet-beta.solana.com
```

## Verification

### Before Rotation
```bash
# Check current mint authority
spl-token display <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Or read from Config PDA
node scripts/check-config.js
```

### After Rotation
```bash
# Verify new mint authority
spl-token display <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Test mint with new authority
node scripts/mint.js --amount 1000 --authority new-mint-authority.json
```

## Audit Trail

All mint authority operations are recorded on-chain:

| Event | What to Monitor |
|-------|----------------|
| `TokensMinted` | Every mint transaction with amount and destination |
| `AuthorityRotationInitiated` | When rotation starts, new authority pubkey |
| `AuthorityRotationConfirmed` | When rotation completes |
| `AuthorityRotationCancelled` | When rotation is cancelled |

## Recommended Configuration

| Environment | Key Storage | Rotation Frequency | Timelock |
|-------------|------------|-------------------|----------|
| Devnet | Local keypair | As needed for testing | 60s |
| Testnet | Encrypted keypair | Monthly | 3600s |
| Mainnet | Hardware wallet | Quarterly | 86400s (24h) |
| Production | Multisig (3-of-5) | Semi-annual | 604800s (7d) |