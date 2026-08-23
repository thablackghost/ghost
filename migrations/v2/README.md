# V2 Migration Notes

## Version: v1.0.0 → v2.0.0

## Status: PLACEHOLDER

This document is a placeholder for future V2 migration instructions. V2 has not been planned or implemented yet.

## Planned Features (Tentative)

Potential features for a V2 upgrade:

- [ ] Transfer hook support for custom transfer logic
- [ ] Confidential transfers (Token-2022 extension)
- [ ] Interest-bearing token mechanics (Token-2022 extension)
- [ ] Governance integration (realms/DAO)
- [ ] Multi-sig mint authority natively enforced
- [ ] Rate limiting on mints
- [ ] Token vesting/lockup schedules
- [ ] Metaplex metadata compatibility layer
- [ ] Improved event emission

## State Format Changes (Expected)

V2 may introduce additional fields to the Config PDA:

```rust
pub struct ConfigV2 {
    // V1 fields (unchanged)
    pub bump: u8,
    pub version: u8,       // = 1 for V2
    pub paused: bool,
    pub padding: [u8; 5],
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub admin_authority: Pubkey,
    pub emergency_authority: Pubkey,
    pub pending_rotation: Option<PendingRotation>,

    // V2 additions (tentative)
    pub mint_rate_limit: Option<RateLimit>,
    pub governance_program: Option<Pubkey>,
    pub metaplex_metadata: Option<Pubkey>,
}
```

## Migration Path (Future)

```bash
# 1. Pause protocol
node scripts/pause.js

# 2. Deploy V2 program
solana program deploy target/deploy/ghost_token.so \
  --program-id Gh0st111111111111111111111111111111111111111

# 3. Run migration (updates Config PDA)
node scripts/migrate-v1-to-v2.js

# 4. Verify state
node scripts/verify-v2-state.js

# 5. Test operations
node scripts/test-v2-operations.js

# 6. Resume protocol
node scripts/resume.js
```

## Rollback V2 → V1

```bash
# 1. Pause protocol
node scripts/pause.js

# 2. Deploy V1 program
git checkout v1.0.0
anchor build
solana program deploy target/deploy/ghost_token.so \
  --program-id Gh0st111111111111111111111111111111111111111

# 3. Reverse migration
node scripts/migrate-v2-to-v1.js

# 4. Verify V1 state
node scripts/verify-v1-state.js

# 5. Resume
node scripts/resume.js
```

## Notes

- This document will be updated with concrete details when V2 is planned
- All V1 features will be backward-compatible
- Migration will be tested extensively on devnet before mainnet
- Community vote may be required before V2 deployment