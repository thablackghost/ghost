# Security Model

## Defense-in-Depth Strategy

The GHOST token program employs multiple overlapping security layers. A single layer failure should not result in compromise.

```
┌─────────────────────────────────────────────────────────┐
│ LAYER 1: Solana Runtime (Sealevel)                      │
│   - Transaction atomicity                               │
│   - Account ownership enforcement                        │
│   - Signature verification                               │
│   - BPF sandbox                                          │
├─────────────────────────────────────────────────────────┤
│ LAYER 2: SPL Token-2022 Program                          │
│   - Mint authority enforcement                           │
│   - Token account ownership                              │
│   - Supply tracking                                      │
├─────────────────────────────────────────────────────────┤
│ LAYER 3: Ghost Program Account Validation                │
│   - Signer checks                                        │
│   - PDA verification                                     │
│   - Owner validation                                     │
│   - Writable account checks                              │
├─────────────────────────────────────────────────────────┤
│ LAYER 4: Ghost Program Business Logic                    │
│   - Authority matching                                   │
│   - Pause state gating                                   │
│   - Timelock enforcement                                  │
│   - Config integrity checks                              │
├─────────────────────────────────────────────────────────┤
│ LAYER 5: Operational Security                             │
│   - Authority key management                             │
│   - Upgrade authority multisig                           │
│   - Monitoring & alerting                                │
│   - Incident response plan                               │
└─────────────────────────────────────────────────────────┘
```

## Validation Layers

### Layer 1: Instruction Deserialization

All instructions are deserialized via Borsh (binary serializer). Invalid data triggers immediate rejection:

```rust
let instruction = GhostInstruction::try_from_slice(input)
    .map_err(|_| ProgramError::InvalidInstructionData)?;
```

### Layer 2: Account Ownership

Every account is validated for correct ownership before any operation:

```rust
// Token-2022 mint owned by Token-2022 program
require!(mint.owner == &spl_token_2022::id(), InvalidMintOwner);

// Config PDA owned by this program
require!(config.owner == program_id, InvalidConfigOwner);

// Token account owned by Token-2022 program
require!(token_account.owner == &spl_token_2022::id(), InvalidTokenAccount);
```

### Layer 3: PDA Verification

Config account address is derived and compared every time:

```rust
let (expected_pda, expected_bump) = Pubkey::find_program_address(
    &[b"config"],
    program_id,
);
require!(config.key == expected_pda, InvalidConfigPDA);
require!(config.bump == expected_bump, InvalidConfigBump);
```

### Layer 4: Signer Verification

Critical operations require specific signers:

```rust
// Signer must match expected authority
require!(signer.is_signer, MissingSigner);
require!(signer.key == expected_authority, Unauthorized);
```

### Layer 5: State Validation

Config state is validated before operations:

```rust
// Pause check for mint operations
require!(!config.paused, Paused);

// Pending rotation check
require!(config.pending_rotation.is_none(), RotationAlreadyPending);
```

### Layer 6: Return Value Validation

CPI return values are checked:

```rust
let result = invoke_signed(&mint_ix, account_infos, &[&seeds]);
// Token-2022 returns Ok(()) on success; error propagated automatically
```

## CPI Validation Model

All Cross-Program Invocations are restricted:

| Rule | Implementation |
|------|----------------|
| Fixed program IDs | Only `spl_token_2022::id()` and `system_program::id()` are invoked |
| No dynamic program invocation | Program IDs are compile-time constants |
| All accounts pre-validated | Accounts passed to CPI are validated before invocation |
| PDA signing only | Program signs CPIs using its PDA seeds, never user keys |
| Error propagation | All CPI errors bubble up |

## Authority Model

| Authority | On-Chain Storage | Rotation Method | Timelock |
|-----------|-----------------|-----------------|----------|
| Mint Authority | Config PDA | Two-step via admin | Configurable |
| Admin Authority | Config PDA | Two-step via admin | Configurable |
| Emergency Authority | Config PDA | Direct by admin | None |
| Upgrade Authority | Program buffer | `solana program set-upgrade-authority` | None |

See [authority-model.md](authority-model.md) for full details.

## Pause Mechanism

### Purpose

Emergency circuit-breaker to halt minting if abnormal activity is detected.

### Activation

- Only emergency authority can pause
- Single instruction: `Pause`
- Immediate effect (same transaction)

### Scope

| Operation | Allowed When Paused? |
|-----------|---------------------|
| Mint tokens | ❌ Blocked |
| Initiate authority rotation | ❌ Blocked |
| Confirm authority rotation | ❌ Blocked |
| Cancel authority rotation | ✅ Allowed |
| Burn tokens | ✅ Always allowed |
| Resume | ✅ Allowed |

### Deactivation

- Only emergency authority can resume
- Single instruction: `Resume`
- Immediate effect

### Monitoring

Recommended monitoring for pause events:
- On-chain event emission on pause/resume
- Off-chain alerting via RPC subscription
- Social media announcement for transparency

## Upgrade Authority

### Current State

The upgrade authority is the keypair that deployed the program. It can:
- Upgrade the program to a new version
- Close the program and recover rent
- Transfer upgrade authority to another key
- Set upgrade authority to `None` (immutability)

### Recommended Configuration

- **During active development**: Single keypair owned by core team
- **Post-audit**: Transfer to multisig (e.g., Squads or Serum multisig)
- **Long-term**: Set to `None` (immutable) after community governance vote

### Immutability Path

```
Deployer Keypair
    │
    ▼
Core Team Multisig (2-of-3)
    │
    ▼
DAO Multisig (governance-controlled)
    │
    ▼
Authority set to None (IMMUTABLE)
```

## Supply Tracking

The SPL Token-2022 program maintains the supply counter on-chain:

```rust
// Read current supply
let mint_data = spl_token_2022::state::Mint::unpack(&mint.data.borrow())?;
let current_supply = mint_data.supply;

// After mint/burn, supply is automatically updated by Token-2022
```

No separate supply tracking is needed; Token-2022 provides the canonical supply.

## Event Emission

All state-changing operations emit Solana events for off-chain monitoring:

| Event | Trigger |
|-------|---------|
| `TokensMinted` | MintTokens instruction |
| `TokensBurned` | BurnTokens instruction |
| `AuthorityRotationInitiated` | InitiateAuthorityRotation |
| `AuthorityRotationConfirmed` | ConfirmAuthorityRotation |
| `AuthorityRotationCancelled` | CancelAuthorityRotation |
| `EmergencyPaused` | Pause instruction |
| `EmergencyResumed` | Resume instruction |