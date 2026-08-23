# V1 Migration Notes

## Version: v0.1.0 → v1.0.0

## Status: INITIAL STATE

This is the initial deployment version. No migration is needed — this is the starting point for all future migrations.

## Program State

| Field | Value |
|-------|-------|
| Program ID | `Gh0st111111111111111111111111111111111111111` |
| Config Version | 0 |
| Mint Decimals | 9 |
| Initial Supply | 10,000,000,000 GHOST |

## State Format (V1)

### Config PDA

```rust
pub struct Config {
    pub bump: u8,
    pub version: u8,       // = 0
    pub paused: bool,
    pub padding: [u8; 5],
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub admin_authority: Pubkey,
    pub emergency_authority: Pubkey,
    pub pending_rotation: Option<PendingRotation>,
}
```

### Instructions (V1)

1. `Initialize` — Create mint, mint 10B tokens, create Config PDA
2. `MintTokens` — Mint additional tokens
3. `BurnTokens` — Burn tokens
4. `InitiateAuthorityRotation` — Start authority rotation
5. `ConfirmAuthorityRotation` — Complete rotation after timelock
6. `CancelAuthorityRotation` — Cancel pending rotation
7. `Pause` — Emergency pause
8. `Resume` — Resume from pause

### Events

- `TokensMinted`
- `TokensBurned`
- `AuthorityRotationInitiated`
- `AuthorityRotationConfirmed`
- `AuthorityRotationCancelled`
- `EmergencyPaused`
- `EmergencyResumed`

## Changes from Initial Deployment

**None.** This is the first version.

## Migration Required?

**No.** This document exists as a baseline for future migrations.

## Reference

- Program design: `docs/architecture/program-design.md`
- Deployment guide: `docs/deployment/mainnet.md`