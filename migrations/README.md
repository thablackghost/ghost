# Migration Guide

## Overview

This directory contains migration guides for upgrading the GHOST token program between major versions. Each migration from version N to N+1 is documented with step-by-step instructions.

## Version History

| Version | Status | Date | Description |
|---------|--------|------|-------------|
| v1 | Current | 2024-01-01 | Initial release |
| v2 | Planned | TBD | Future upgrade |

## Migration Process

### General Steps

1. **Read the migration guide** for the target version
2. **Verify prerequisites** (software versions, permissions)
3. **Backup all authority keypairs** before proceeding
4. **Pause the protocol** (recommended for major migrations)
5. **Deploy the new program version**
6. **Run migration scripts** (if state format changed)
7. **Verify migrated state** matches expectations
8. **Test all operations** (mint, burn, rotate, pause/resume)
9. **Resume the protocol**
10. **Announce completion** to the community

### Pre-Migration Checklist

- [ ] All tests passing on current version
- [ ] New version tested on devnet
- [ ] Migration script tested on devnet
- [ ] Rollback plan documented
- [ ] All authority keypairs verified and accessible
- [ ] Community notified (minimum 48 hours for mainnet)

### Post-Migration Checklist

- [ ] Program version verified (`solana program show`)
- [ ] Config PDA state verified
- [ ] Mint supply unchanged
- [ ] All authorities intact
- [ ] Test mint (small amount)
- [ ] Test burn (small amount)
- [ ] Test pause/resume cycle
- [ ] Metadata intact
- [ ] Explorer displays correctly

## Rollback Procedure

If migration fails:

1. **Do NOT resume** the protocol if paused
2. **Identify the failure point** from logs
3. **Execute rollback** per version-specific instructions
4. **Verify rollback** restored previous state
5. **Resume** if protocol was paused
6. **Post-mortem** the migration failure

## Migration Scripts

Migration scripts are versioned:

```
migrations/
├── README.md          # This file
├── v1/
│   └── README.md      # V1 initial state (no migration needed)
└── v2/
    └── README.md      # V2 migration guide
```

Each migration directory contains:
- `README.md`: Step-by-step guide
- `migrate.ts` or `migrate.rs`: Migration script
- `verify.ts` or `verify.rs`: Verification script

## Emergency Contacts

If something goes wrong during migration:

| Contact | Method |
|---------|--------|
| Protocol Lead | protocol@ghost-token.io |
| Security Lead | security@ghost-token.io |
| Emergency Hotline | [REDACTED - Internal Only] |