# Security Checklist

## 50-Point Security Verification Checklist

### Authentication & Authorization (1-10)

- [ ] 1. Mint instruction requires mint authority signer
- [ ] 2. Initiate rotation requires admin authority signer
- [ ] 3. Pause requires emergency authority signer
- [ ] 4. Resume requires emergency authority signer
- [ ] 5. Burn does NOT require special authority (permissionless)
- [ ] 6. Confirm rotation allows ANY signer (permissionless)
- [ ] 7. All signer checks use `is_signer` field, not key matching alone
- [ ] 8. Authority comparisons use Config PDA stored values, not hardcoded keys
- [ ] 9. No hardcoded authority pubkeys in program code
- [ ] 10. Missing signer produces clear error message

### Account Validation (11-20)

- [ ] 11. Config account address verified via PDA derivation
- [ ] 12. Config account bump matched against derived bump
- [ ] 13. Mint account address verified against Config.mint
- [ ] 14. Mint account owner verified as Token-2022 program
- [ ] 15. Token account owner verified as Token-2022 program
- [ ] 16. Config account owner verified as this program
- [ ] 17. All writable accounts checked with `is_writable`
- [ ] 18. No unchecked account iteration
- [ ] 19. Accounts deserialized safely (no unwrap on data)
- [ ] 20. Invalid account data returns clear error

### CPI Security (21-30)

- [ ] 21. Token-2022 program ID is a compile-time constant
- [ ] 22. System program ID is a compile-time constant
- [ ] 23. No user-supplied program IDs for CPI
- [ ] 24. All CPI accounts validated before invocation
- [ ] 25. CPI errors propagated, not swallowed
- [ ] 26. PDA signing seeds verified (correct bump)
- [ ] 27. No cross-program reentrancy possible
- [ ] 28. Token-2022 CPI uses correct instruction builder
- [ ] 29. System program CPI uses correct instruction builder
- [ ] 30. CPI account order matches Token-2022 expectations

### PDA Security (31-35)

- [ ] 31. Config PDA seeds use static values (`b"config"`)
- [ ] 32. No dynamic seed derivation from user input
- [ ] 33. PDA bump stored and verified on read
- [ ] 34. PDAs are off-curve (verified at test time)
- [ ] 35. PDA signing correctly uses `invoke_signed` with seeds

### Arithmetic Safety (36-40)

- [ ] 36. All arithmetic uses `checked_*` operations
- [ ] 37. `#[deny(arithmetic_overflow)]` enabled
- [ ] 38. No unchecked casts between integer types
- [ ] 39. Amount validation for zero values (reject or allow as designed)
- [ ] 40. Maximum amount handling for u64 boundary values

### State Management (41-45)

- [ ] 41. Config initialized exactly once (initialize fails if already exists)
- [ ] 42. Config.mint field immutable after initialization
- [ ] 43. Paused state correctly enforced on gated operations
- [ ] 44. Pending rotation mutex (only one at a time)
- [ ] 45. Timelock enforced using Clock sysvar

### Upgrade & Deployment (46-50)

- [ ] 46. Upgrade authority key stored securely (not in repo)
- [ ] 47. Program version tracked in Config.version
- [ ] 48. Event emission for all state changes
- [ ] 49. No debug logging or panic messages in release build
- [ ] 50. Build is reproducible (`anchor build --verifiable`)

## Verification Methods

| # | Check | Verification Method |
|---|-------|-------------------|
| 1-10 | Auth | Security tests + code review |
| 11-20 | Account validation | Security tests + code review |
| 21-30 | CPI | Code review + fuzz testing |
| 31-35 | PDA | Unit tests + code review |
| 36-40 | Arithmetic | Fuzz testing + compiler flags |
| 41-45 | State | Integration tests + property tests |
| 46-50 | Upgrade | Operational review + CI verification |

## Review Status

| Reviewer | Date | Findings |
|----------|------|----------|
| Core Team (Internal) | 2024-01 | All 50 items reviewed. No critical findings. |