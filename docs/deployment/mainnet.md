# Mainnet Deployment Guide

## ⚠️ WARNING

**Mainnet deployment is irreversible.** Tokens minted on mainnet are real assets with real value. Follow every step carefully. Any mistake could result in permanent loss of funds or authority.

## Prerequisites

### Hardware Security
- **Hardware wallet** (Ledger, Trezor) for all authority keypairs
- **Air-gapped machine** for key generation (recommended)
- **Offline backup** of all seed phrases and keypair files (encrypted)

### Software Requirements
| Tool | Version |
|------|---------|
| Solana CLI | 1.18+ (stable) |
| Anchor CLI | 0.30+ |
| Rust | 1.75+ |

### RPC Access
- Dedicated RPC endpoint (Helius, QuickNode, Triton, or self-hosted)
- Fallback RPC endpoints configured
- WebSocket connection for event monitoring

### Wallet Setup
```bash
# NEVER run on a network-connected machine for real keys
# This is for illustration only — use hardware wallet in practice

solana-keygen new --outfile ~/.config/solana/mainnet-deploy.json
```

**Recommended:** Use a multisig (Squads) for all production authorities.

## Pre-Deployment Security Checklist

### 1. Code Audit
- [ ] External security audit completed (report in `audits/reports/`)
- [ ] Internal security review completed
- [ ] All test suites passing: `anchor test`
- [ ] Fuzz testing completed with no findings
- [ ] Security checklist (50 items) all passed

### 2. Authority Planning

| Authority | Storage | Backup |
|-----------|---------|--------|
| Deploy key | Hardware wallet | Encrypted offline backup |
| Mint authority | Ledger Nano X | Seed phrase (fireproof safe) |
| Admin authority | Trezor Model T | Seed phrase (separate location) |
| Emergency authority | Hot wallet (mobile) | Seed phrase (secure backup) |
| Metadata authority | Hardware wallet | Same as admin |

### 3. Keypair Verification

```bash
# Verify each keypair (from offline/air-gapped machine)
solana-keygen verify <pubkey> <path-to-keypair.json>

# Export pubkeys for record
solana-keygen pubkey <path-to-keypair.json>
```

### 4. Program Verification

```bash
# Build from clean state
cargo clean
anchor build --verifiable

# Verify the build is reproducible
anchor verify Gh0st111111111111111111111111111111111111111
```

### 5. RPC Endpoint Verification

```bash
# Test mainnet RPC
solana cluster-version --url https://api.mainnet-beta.solana.com

# Verify slot is advancing
solana slot --url https://api.mainnet-beta.solana.com
```

## Step 1: Fund Deploy Account

```bash
# Transfer ~5 SOL to deploy account
solana transfer <deploy-pubkey> 5 \
  --from <funded-account> \
  --url https://api.mainnet-beta.solana.com \
  --fee-payer <funded-account-keypair.json> \
  --allow-unfunded-recipient

# Verify balance
solana balance <deploy-pubkey> --url https://api.mainnet-beta.solana.com
```

## Step 2: Deploy Program

```bash
# Deploy to mainnet
solana program deploy \
  --url https://api.mainnet-beta.solana.com \
  --program-id target/deploy/ghost_token-keypair.json \
  --keypair ~/.config/solana/mainnet-deploy.json \
  --max-sign-attempts 5 \
  --with-compute-unit-price 100000 \
  target/deploy/ghost_token.so

# Record program ID
# Gh0st111111111111111111111111111111111111111
```

**NOTE:** Mainnet deployment requires the vanity keypair. This keypair was generated specifically for program ID `Gh0st111111111111111111111111111111111111111`.

## Step 3: Verify Program Deployment

```bash
# Check deployed program
solana program show Gh0st111111111111111111111111111111111111111 \
  --url https://api.mainnet-beta.solana.com

# Verify on Explorer
# https://explorer.solana.com/address/Gh0st111111111111111111111111111111111111111
```

## Step 4: Initialize Token

```bash
# Run initialize using the SDK (this creates the mint and config PDA)
node scripts/initialize-mainnet.js \
  --url https://api.mainnet-beta.solana.com \
  --mint-authority <mint-authority-pubkey> \
  --admin-authority <admin-authority-pubkey> \
  --emergency-authority <emergency-authority-pubkey> \
  --token-account <initial-token-account>
```

This transaction:
1. Creates Token-2022 mint with 9 decimals
2. Sets mint authority to the specified pubkey
3. Mints 10,000,000,000 GHOST (10 billion)
4. Creates Config PDA with authority assignments
5. Sets token metadata (name: GHOST, symbol: GHOST)

## Step 5: Post-Deployment Verification

### Verify Mint
```bash
spl-token display <mint-address> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb \
  --url https://api.mainnet-beta.solana.com

# Expected:
# Decimals: 9
# Supply: 10,000,000,000
# Mint Authority: <mint-authority-pubkey>
```

### Verify Supply
```bash
spl-token supply <mint-address> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb \
  --url https://api.mainnet-beta.solana.com

# Expected: 10000000000000000000 (raw, 10B * 10^9)
```

### Verify Authorities
```bash
# Check Config PDA
solana account <config-pda-address> \
  --url https://api.mainnet-beta.solana.com

# Verify mint_authority, admin_authority, emergency_authority match expected
```

### Verify Metadata
```bash
# Check Token-2022 metadata extension
spl-token display <mint-address> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Expected metadata fields should be present
```

## Post-Deployment Checklist

- [ ] Program deployed and verified on Explorer
- [ ] Mint created with correct decimals (9)
- [ ] Initial supply: exactly 10,000,000,000 GHOST
- [ ] Mint authority matches planned keypair
- [ ] Admin authority matches planned keypair
- [ ] Emergency authority matches planned keypair
- [ ] Metadata is correct on Explorer
- [ ] Test mint transaction (small amount)
- [ ] Test burn transaction (small amount)
- [ ] Test pause/resume cycle
- [ ] All authority keypairs backed up securely
- [ ] Upgrade authority recorded
- [ ] Deployment transaction signatures archived
- [ ] Token registered on Jupiter/Raydium (optional)
- [ ] Social media announcement prepared
- [ ] Monitoring and alerting configured

## Important Security Notes

### 1. Program Immutability

The program currently has the deploy keypair as upgrade authority. After thorough auditing and community consensus, consider:

```bash
# Make the program IMMUTABLE (irreversible!)
solana program set-upgrade-authority \
  Gh0st111111111111111111111111111111111111111 \
  --new-upgrade-authority <void111111111111111111111111111111111111111> \
  --url https://api.mainnet-beta.solana.com
```

**WARNING:** This cannot be undone. Ensure this is the desired state.

### 2. Authority Key Security

- **NEVER** store authority keypairs on internet-connected machines
- Use hardware wallets (Ledger, Trezor) for production authorities
- Implement multisig (Squads) for all critical operations
- Use MPC wallets for distributed key management
- Consider social recovery mechanisms

### 3. Monitoring Setup

Immediately after deployment:
- Set up RPC log monitoring for program activity
- Configure alerts for mint events > threshold
- Configure alerts for authority rotation events
- Configure alerts for pause/resume events
- Set up balance monitoring for authority accounts

## Emergency Contacts

| Role | Contact |
|------|---------|
| Security Lead | security@ghost-token.io |
| Protocol Lead | protocol@ghost-token.io |
| Emergency Hotline | [REDACTED - Internal Only] |

## Deployment Artifacts

Archive the following for future reference:

```
mainnet-deployment-YYYY-MM-DD/
├── deploy-transaction-signature.txt
├── initialize-transaction-signature.txt
├── mint-address.txt
├── config-pda-address.txt
├── authority-pubkeys.json
├── metadata.json
└── deployment-log.md
```