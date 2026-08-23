# Verification Guide

## Overview

This guide covers independent verification of the GHOST token program deployment, mint, supply, and authorities. All steps can be performed by any third party without access to private keys.

## 1. Program Verification on Explorer

### Via Solana Explorer (Web)

1. Navigate to: `https://explorer.solana.com/address/Gh0st111111111111111111111111111111111111111`
2. Verify the page shows "Program" account type
3. Verify the owner is `BPFLoaderUpgradeab1e11111111111111111111111`
4. Check the "Program Data" address
5. Verify the program is marked "Executable: Yes"

### Via Solana CLI

```bash
# Check program account
solana program show Gh0st111111111111111111111111111111111111111

# Expected output format:
# Program Id: Gh0st111111111111111111111111111111111111111
# Owner: BPFLoaderUpgradeab1e11111111111111111111111
# ProgramData Address: <some-address>
# Authority: <upgrade-authority-address>
# Last Deployed Slot: <slot-number>
# Data Length: <byte-count>
```

### Verify Program Binary

To verify the deployed bytecode matches the source:

```bash
# Build from source (specific commit)
git checkout <deployment-commit-hash>
anchor build --verifiable

# Compare buffer hash
solana program dump Gh0st111111111111111111111111111111111111111 deployed.so
sha256sum deployed.so target/deploy/ghost_token.so
# Expected: matching checksums
```

### Verify Vanity Address

The program ID `Gh0st111111111111111111111111111111111111111` contains the prefix "Gh0st" — this is a vanity address. Verify it is genuine:

1. The prefix "Gh0st" (5 base58 characters) appears at the start
2. The first character 'G' indicates it is a valid base58 program address
3. Base58 encoding: `Gh(0-9A-Za-z){42}` = 44 characters total
4. See `docs/vanity/verification.md` for cryptographic verification

## 2. Mint Verification

### Find Mint Address

```bash
# The mint address is stored in the Config PDA
# First, derive the Config PDA:
solana find-program-derived-address Gh0st111111111111111111111111111111111111111 "config"
# Output: <config-pda-address>

# Read config data to find mint
solana account <config-pda-address>
# Look for: mint: <32-byte-pubkey> (first pubkey after version/paused fields)
```

Alternatively, the mint address should be published by the team.

### Check Mint Configuration

```bash
# Replace <MINT_ADDRESS> with actual mint
spl-token display <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Expected:
# Name: GHOST
# Symbol: GHOST
# Decimals: 9
# Mint Authority: <mint-authority-pubkey>
# Supply: 10000000000000000000 (raw) or 10,000,000,000 (display)
```

### Verify Mint Program

```bash
solana account <MINT_ADDRESS>

# Expected:
# Owner: TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb (Token-2022)
# Not owned by: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA (Original SPL)
```

The mint MUST be owned by `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` (Token-2022 program).

## 3. Supply Verification

### Check Total Supply

```bash
spl-token supply <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Raw supply (in minimum units with 9 decimals):
# Initial: 10000000000000000000 = 10,000,000,000 * 10^9
```

### Verify Supply on Explorer

1. Go to `https://explorer.solana.com/address/<MINT_ADDRESS>`
2. Under "Token Info":
   - Check "Decimals" = 9
   - Check "Current Supply" (should be >= 10,000,000,000)
3. Click "Holders" tab to see token distribution

### Track Supply Changes

```bash
# Watch for mint events (supply increases)
# Use WebSocket or periodic polling
spl-token supply <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
```

## 4. Authority Verification

### Check Mint Authority

```bash
spl-token display <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb \
  | grep "Mint authority"

# Expected: The team's published mint authority pubkey
```

### Check Config PDA Authorities

```bash
# Read raw config data
solana account <CONFIG_PDA_ADDRESS>

# Parse manually or with script to extract:
# - mint_authority (offset depends on struct layout)
# - admin_authority
# - emergency_authority
# - paused flag
# - pending_rotation state
```

### Verify Upgrade Authority

```bash
solana program show Gh0st111111111111111111111111111111111111111 | grep Authority

# Expected: Team's upgrade authority (or None if immutable)
```

### Verify Metadata Authority

```bash
# For Token-2022 metadata extension
solana account <MINT_ADDRESS> --output json | jq .
# Check for metadata_update_authority field in extension data
```

## 5. Config PDA Verification

### Derive Expected PDA

```rust
// Anyone can compute this:
use solana_program::pubkey::Pubkey;

let program_id = Pubkey::from_str("Gh0st111111111111111111111111111111111111111").unwrap();
let (config_pda, bump) = Pubkey::find_program_address(
    &[b"config"],
    &program_id,
);
// config_pda should match the published Config PDA address
```

```bash
# Via CLI:
solana find-program-derived-address Gh0st111111111111111111111111111111111111111 "config"
```

### Verify Config Data Integrity

```bash
# Dump config data
solana account <CONFIG_PDA_ADDRESS> --output json > config.json

# Verify fields:
# - bump: matches the derived bump
# - paused: false (unless emergency paused)
# - mint: matches the known mint address
# - All three authority pubkeys match published values
```

## 6. Metadata Verification

### Check Token-2022 Metadata

```bash
spl-token display <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Expected metadata:
# - Name: "GHOST"
# - Symbol: "GHOST"
# - URI: "https://arweave.net/placeholder-ghost-logo"
```

### Verify Metadata URI

```bash
# Fetch metadata JSON from the URI
curl https://arweave.net/placeholder-ghost-logo

# Expected JSON structure:
# {
#   "name": "GHOST",
#   "symbol": "GHOST",
#   "description": "A meme token called GHOST.",
#   "image": "https://arweave.net/placeholder-ghost-logo",
#   ...
# }
```

## 7. Transaction History Verification

### View Initialize Transaction

```bash
solana transaction-history Gh0st111111111111111111111111111111111111111 \
  --show-transactions \
  --limit 5
```

### Verify No Suspicious Activity

- Check for unexpected mint events
- Check for unexpected authority rotations
- Check for unexpected pause events
- Compare transaction volume against expected patterns

## 8. Automated Verification Script

```bash
#!/bin/bash
# verify-ghost.sh — Complete verification script

PROGRAM_ID="Gh0st111111111111111111111111111111111111111"
CLUSTER="${1:-mainnet-beta}"
URL="https://api.${CLUSTER}.solana.com"

echo "=== Verifying GHOST Token ==="
echo "Cluster: $CLUSTER"

# 1. Program check
echo -e "\n[1/5] Checking program..."
solana program show $PROGRAM_ID --url $URL

# 2. Config PDA
echo -e "\n[2/5] Deriving Config PDA..."
CONFIG_PDA=$(solana find-program-derived-address $PROGRAM_ID "config" | head -1)
echo "Config PDA: $CONFIG_PDA"

# 3. Read mint address from config
echo -e "\n[3/5] Reading mint from config..."
solana account $CONFIG_PDA --url $URL

# 4. Mint check
echo -e "\n[4/5] Checking mint..."
# Requires MINT_ADDRESS from previous step
# spl-token display $MINT --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# 5. Supply check
echo -e "\n[5/5] Checking supply..."
# spl-token supply $MINT --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

echo -e "\n=== Verification Complete ==="
```

## 9. Continuous Monitoring

Set up ongoing verification:

- **Periodic supply check:** Daily automated script
- **Authority change alert:** WebSocket subscription on Config PDA
- **Mint event alert:** Monitor token program for `mintTo` instructions
- **Program upgrade alert:** Monitor program buffer for new deployments
- **Explorer monitoring:** Use public APIs to verify Explorer data matches on-chain data

## 10. Red Flags

If any of these are observed, investigate immediately:

- [ ] Supply is less than 10B (unexpected burn)
- [ ] Supply increased without team announcement
- [ ] Mint authority changed unexpectedly
- [ ] Admin authority changed unexpectedly
- [ ] Config PDA shows `paused: true` without announcement
- [ ] Program upgraded without announcement
- [ ] Metadata updated to misleading values
- [ ] Mint owner is not Token-2022 program
- [ ] Program is not at expected address