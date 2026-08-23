# STRIDE Threat Model

## Overview

This document applies the STRIDE threat modeling framework to the GHOST token program.

### STRIDE Framework

| Category | Definition | Focus |
|----------|-----------|-------|
| **S**poofing | Impersonating another user or system | Authentication |
| **T**ampering | Modifying data or code | Integrity |
| **R**epudiation | Denying an action occurred | Non-repudiation |
| **I**nformation Disclosure | Exposing information to unauthorized parties | Confidentiality |
| **D**enial of Service | Degrading or blocking service availability | Availability |
| **E**levation of Privilege | Gaining unauthorized permissions | Authorization |

## 1. Spoofing

### S-1: Spoofing the Mint Authority

| Property | Value |
|----------|-------|
| **Threat** | Attacker signs MintTokens with an account that is not the real mint authority |
| **Severity** | Critical |
| **Mitigation** | `require!(signer.key == config.mint_authority)` + `is_signer` check |
| **Validation** | Security test: "rejects mint with wrong mint authority" |

### S-2: Spoofing the Admin Authority

| Property | Value |
|----------|-------|
| **Threat** | Attacker initiates authority rotation without being admin |
| **Severity** | High |
| **Mitigation** | `require!(signer.key == config.admin_authority)` + signer check |
| **Validation** | Security test: "rejects initiate rotation from non-admin" |

### S-3: Spoofing the Emergency Authority

| Property | Value |
|----------|-------|
| **Threat** | Attacker pauses the protocol without being emergency authority |
| **Severity** | Medium |
| **Mitigation** | `require!(signer.key == config.emergency_authority)` + signer check |
| **Validation** | Security test: "rejects pause from non-emergency authority" |

### S-4: Spoofing a PDA

| Property | Value |
|----------|-------|
| **Threat** | Attacker creates an account at a non-PDA address with Config-like data |
| **Severity** | High |
| **Mitigation** | PDA derivation + address comparison + bump verification |
| **Validation** | Test: "rejects config at wrong PDA" |

### S-5: Spoofing the Token-2022 Program

| Property | Value |
|----------|-------|
| **Threat** | Attacker passes a fake Token-2022 program ID for CPI |
| **Severity** | High |
| **Mitigation** | Program ID is a compile-time constant; compared before CPI |
| **Validation** | Security test: "rejects wrong token program" |

## 2. Tampering

### T-1: Tampering with Config PDA Data

| Property | Value |
|----------|-------|
| **Threat** | Attacker directly writes to Config PDA to change authorities |
| **Severity** | Critical |
| **Mitigation** | Config PDA is owned by this program; only program can modify it via instructions; Solana runtime enforces ownership |
| **Validation** | Runtime guarantee + code review |

### T-2: Tampering with Mint Supply

| Property | Value |
|----------|-------|
| **Threat** | Attacker modifies mint supply counter directly |
| **Severity** | Critical |
| **Mitigation** | Mint owned by Token-2022; only Token-2022 can modify supply via CPI; all CPI calls are pre-validated |
| **Validation** | Security test: "rejects mint owned by wrong program" |

### T-3: Tampering with Timelock

| Property | Value |
|----------|-------|
| **Threat** | Attacker manipulates the Clock sysvar or effective_at field |
| **Severity** | High |
| **Mitigation** | `Clock::get()?` reads from runtime; effective_at is set by program; signed comparison prevents manipulation |
| **Validation** | Security test: "rejects confirm before timelock expires" |

### T-4: Tampering with Pending Rotation

| Property | Value |
|----------|-------|
| **Threat** | Attacker modifies pending_rotation to bypass checks |
| **Severity** | High |
| **Mitigation** | pending_rotation can only be modified by program instructions with proper auth; tamper attempts fail account serialization |
| **Validation** | Ownership + PDA verification |

## 3. Repudiation

### R-1: Denying a Mint Operation

| Property | Value |
|----------|-------|
| **Threat** | Mint authority claims it didn't mint tokens that it did |
| **Severity** | Low |
| **Mitigation** | All mints recorded on-chain; transaction signatures are permanent; events emitted |
| **Validation** | Solana blockchain immutability |

### R-2: Denying an Authority Rotation

| Property | Value |
|----------|-------|
| **Threat** | Admin authority claims it didn't rotate authorities |
| **Severity** | Medium |
| **Mitigation** | Rotation events emitted; Config PDA state history; transaction signatures |
| **Validation** | On-chain audit trail |

### R-3: Denying a Pause/Resume

| Property | Value |
|----------|-------|
| **Threat** | Emergency authority claims it didn't pause/resume |
| **Severity** | Medium |
| **Mitigation** | Pause/Resume events emitted; Config.paused field change; transaction signatures |
| **Validation** | On-chain audit trail |

## 4. Information Disclosure

### I-1: Leaking Authority Private Keys

| Property | Value |
|----------|-------|
| **Threat** | Authority keypairs exposed through operational security failures |
| **Severity** | Critical |
| **Mitigation** | Off-chain: hardware wallets, cold storage, encrypted backups; On-chain: two-step rotation + emergency pause |
| **Validation** | Operational procedures, not code |

### I-2: Leaking Mint Amounts Before Public

| Property | Value |
|----------|-------|
| **Threat** | Mempool observation reveals planned mint amounts |
| **Severity** | Low |
| **Mitigation** | Not a Solana concern (no public mempool); transactions are private until confirmed |
| **Validation** | Solana architecture |

### I-3: Leaking Program State to Unauthorized Programs

| Property | Value |
|----------|-------|
| **Threat** | Another program reads Config PDA data |
| **Severity** | None |
| **Mitigation** | Config PDA is public data on Solana; this is by design |
| **Validation** | N/A — public data |

## 5. Denial of Service

### D-1: Excessive Pause/Resume

| Property | Value |
|----------|-------|
| **Threat** | Emergency authority repeatedly pauses and resumes, disrupting service |
| **Severity** | Medium |
| **Mitigation** | Admin can rotate emergency authority without timelock; community pressure |
| **Validation** | Operational, not code-level |

### D-2: Compute Budget Exhaustion

| Property | Value |
|----------|-------|
| **Threat** | Attacker crafts transactions that consume excessive compute units |
| **Severity** | Low |
| **Mitigation** | Solana enforces per-transaction compute limits; program has no unbounded loops |
| **Validation** | Code review for loop bounds |

### D-3: Account Exhaustion

| Property | Value |
|----------|-------|
| **Threat** | Attacker creates many accounts to exhaust rent or storage |
| **Severity** | Low |
| **Mitigation** | Program only creates Config PDA (one-time); no dynamic account creation |
| **Validation** | Code review |

### D-4: Spam Transactions

| Property | Value |
|----------|-------|
| **Threat** | Attacker floods the network with valid but useless transactions |
| **Severity** | Low |
| **Mitigation** | Solana transaction fees + priority fees make spam costly |
| **Validation** | Solana network-level mitigation |

## 6. Elevation of Privilege

### E-1: Unauthorized Mint Authority Acquisition

| Property | Value |
|----------|-------|
| **Threat** | Attacker becomes mint authority through rotation exploit or config manipulation |
| **Severity** | Critical |
| **Mitigation** | Two-step rotation with timelock; admin-only initiation; pause blocks confirmation; PDA ownership prevents direct writes |
| **Validation** | Integration tests for rotation lifecycle |

### E-2: Unauthorized Admin Authority Acquisition

| Property | Value |
|----------|-------|
| **Threat** | Attacker becomes admin authority |
| **Severity** | High |
| **Mitigation** | Two-step rotation with timelock; self-rotation allowed but timelocked |
| **Validation** | Integration tests for rotation lifecycle |

### E-3: Unauthorized Program Upgrade

| Property | Value |
|----------|-------|
| **Threat** | Attacker deploys malicious program version |
| **Severity** | Critical |
| **Mitigation** | Upgrade authority key stored in cold storage/multisig; eventual immutability path |
| **Validation** | Off-chain key management |

### E-4: Bypassing Pause

| Property | Value |
|----------|-------|
| **Threat** | Attacker finds way to mint while protocol is paused |
| **Severity** | High |
| **Mitigation** | Pause check is the FIRST validation in mint instruction; no early returns bypass it |
| **Validation** | Security test: "rejects mint while paused" |

### E-5: Timelock Bypass

| Property | Value |
|----------|-------|
| **Threat** | Attacker confirms rotation before timelock expires |
| **Severity** | High |
| **Mitigation** | Clock.get()?.unix_timestamp >= effective_at check; effective_at set by program on initiate |
| **Validation** | Security test: "rejects confirm before timelock expires" |

## Summary

| STRIDE Category | Threats Identified | Critical | High | Medium | Low |
|-----------------|-------------------|----------|------|--------|-----|
| Spoofing | 5 | 1 | 3 | 1 | 0 |
| Tampering | 4 | 2 | 2 | 0 | 0 |
| Repudiation | 3 | 0 | 0 | 2 | 1 |
| Information Disclosure | 3 | 1 | 0 | 0 | 2 |
| Denial of Service | 4 | 0 | 0 | 1 | 3 |
| Elevation of Privilege | 5 | 2 | 3 | 0 | 0 |
| **Total** | **24** | **6** | **8** | **4** | **6** |

All identified threats have documented mitigations implemented in code and/or operational procedures.