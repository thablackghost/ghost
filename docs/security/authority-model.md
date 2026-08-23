# Authority Model

## Overview

The GHOST token program uses a multi-authority model with clear separation of concerns. No single authority has unrestricted control; each role is scoped to specific operations.

## Authority Roles

### 1. Mint Authority

| Property | Value |
|----------|-------|
| **Stored in** | `Config.mint_authority` |
| **Can be rotated by** | Admin Authority (via two-step process) |
| **Responsibilities** | Mint new GHOST tokens (no supply cap) |
| **Risk level** | **CRITICAL** — Can cause infinite inflation |

**Scope:**
- ✅ `MintTokens` — mint any amount to any token account
- ❌ `BurnTokens` — cannot burn other users' tokens
- ❌ Authority rotation — cannot rotate any authority
- ❌ Pause/Resume — cannot pause the protocol

**Best Practices:**
- Store in hardware wallet or multisig
- Rotate regularly (quarterly recommended)
- Monitor all mint transactions for anomalies

### 2. Admin Authority

| Property | Value |
|----------|-------|
| **Stored in** | `Config.admin_authority` |
| **Can be rotated by** | Admin Authority (via two-step process, self-rotation) |
| **Responsibilities** | Rotate mint and emergency authorities |
| **Risk level** | **HIGH** — Can install new mint authority |

**Scope:**
- ✅ `InitiateAuthorityRotation` — start rotation for mint/emergency/admin authorities
- ✅ `CancelAuthorityRotation` — cancel pending rotation
- ❌ `ConfirmAuthorityRotation` — anyone can confirm after timelock
- ❌ `MintTokens` — cannot mint directly
- ❌ `Pause/Resume` — cannot pause

**Best Practices:**
- Cold storage keypair (offline)
- Used only for scheduled rotations
- Transaction signing via air-gapped machine

### 3. Emergency Authority

| Property | Value |
|----------|-------|
| **Stored in** | `Config.emergency_authority` |
| **Can be rotated by** | Admin Authority (direct, no timelock) |
| **Responsibilities** | Pause and resume protocol operations |
| **Risk level** | **MEDIUM** — Can cause temporary service disruption (recoverable) |

**Scope:**
- ✅ `Pause` — halt minting and authority rotation
- ✅ `Resume` — restore normal operations
- ❌ `MintTokens` — cannot mint
- ❌ Authority rotation — cannot rotate authorities

**Best Practices:**
- Hot wallet for rapid response
- Separate from admin/mint keys
- Multi-party access (multiple team members)

### 4. Metadata Authority

| Property | Value |
|----------|-------|
| **Stored in** | Token-2022 Mint metadata extension |
| **Can be rotated by** | Metadata authority itself (via Token-2022 instruction) |
| **Responsibilities** | Update token metadata (name, symbol, URI, image) |
| **Risk level** | **LOW** — Cosmetic changes only |

**Scope:**
- ✅ Update metadata fields
- ❌ Cannot mint tokens
- ❌ Cannot pause or rotate authorities

**Best Practices:**
- Separate keypair from program authorities
- Used for URI/image updates
- Consider freezing metadata authority for immutability

### 5. Upgrade Authority (Off-Chain)

| Property | Value |
|----------|-------|
| **Stored in** | Program buffer account |
| **Can be rotated by** | Upgrade authority (Solana native instruction) |
| **Responsibilities** | Deploy program upgrades |
| **Risk level** | **CRITICAL** — Can replace entire program logic |

**Scope:**
- ✅ Deploy new program versions
- ✅ Transfer upgrade authority
- ✅ Set upgrade authority to None (immutability)

**Best Practices:**
- Multisig (e.g., Squads, 3-of-5)
- Eventually set to None after community vote
- All upgrades publicly announced and code-audited

## Separation of Concerns Matrix

| Action | Mint Auth | Admin Auth | Emergency Auth | Upgrade Auth | Meta Auth |
|--------|-----------|------------|----------------|--------------|-----------|
| Mint tokens | ✅ | ❌ | ❌ | ❌ | ❌ |
| Burn own tokens | ✅ (as owner) | ✅ (as owner) | ✅ (as owner) | ❌ | ❌ |
| Initiate rotation | ❌ | ✅ | ❌ | ❌ | ❌ |
| Confirm rotation | ✅ | ✅ | ✅ | ❌ | ❌ |
| Cancel rotation | ❌ | ✅ | ❌ | ❌ | ❌ |
| Pause protocol | ❌ | ❌ | ✅ | ❌ | ❌ |
| Resume protocol | ❌ | ❌ | ✅ | ❌ | ❌ |
| Upgrade program | ❌ | ❌ | ❌ | ✅ | ❌ |
| Update metadata | ❌ | ❌ | ❌ | ❌ | ✅ |

## Authority Rotation Process

### Two-Step Rotation (Mint, Admin)

```
STEP 1: Initiate
    Admin signs InitiateAuthorityRotation
    → Config.pending_rotation is set
    → effective_at = clock.unix_timestamp + TIMELOCK_DURATION
    → Event: AuthorityRotationInitiated

    [Wait for timelock period]

STEP 2: Confirm
    Anyone signs ConfirmAuthorityRotation
    → Verify timelock expired
    → Config.<authority_field> = pending_rotation.new_authority
    → Config.pending_rotation = None
    → Event: AuthorityRotationConfirmed

CANCEL (any time before confirm):
    Admin signs CancelAuthorityRotation
    → Config.pending_rotation = None
    → Event: AuthorityRotationCancelled
```

### Direct Rotation (Emergency Authority)

Emergency authority can be rotated immediately without timelock (for rapid response). This is an intentional design choice — if the emergency authority is compromised, admin must act fast.

### Self-Rotation (Admin Authority)

Admin can initiate rotation of its own key. The two-step process still applies. This means:
- Admin initiates rotation to a new admin key
- During timelock, both old and new admin cannot initiate new rotations (only one pending at a time)
- Old admin can cancel if compromise is detected

## Rotation Security Considerations

1. **Timelock Duration:** Default is 24 hours (86,400 seconds). Should be long enough for monitoring to detect unauthorized rotations.

2. **Pending Rotation Mutex:** Only one rotation can be pending at a time. Admin must cancel a pending rotation before initiating a new one.

3. **Confirm by Anyone:** Confirmation is permissionless after timelock. This prevents admin from blocking a legitimate rotation.

4. **No Instant Takeover:** Two-step ensures an attacker who compromises admin cannot instantly install themselves as mint authority. The timelock gives defenders time to pause via emergency authority.

5. **Emergency Override:** If admin is compromised and starts a malicious rotation:
   - Emergency authority pauses the protocol (blocks confirmation)
   - Timelock prevents immediate confirmation
   - Community/team can respond

## Monitoring Recommendations

| Event | Alert Threshold | Response |
|-------|----------------|----------|
| Authority rotation initiated | Immediate | Verify legitimacy, prepare to pause |
| Authority rotation confirmed | Immediate | Verify new authority, update key registry |
| Pause activated | Immediate | Investigate cause |
| Resume activated | Immediate | Confirm resolution |
| Large mint (>1B tokens) | Immediate | Verify with mint authority |