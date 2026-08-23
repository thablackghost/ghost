# Operations Runbook

## Overview

This runbook provides standard operating procedures for the GHOST token program. It covers common operational tasks, monitoring, and incident response.

## Daily Operations

### Health Check

```bash
#!/bin/bash
# daily-health-check.sh

PROGRAM_ID="Gh0st111111111111111111111111111111111111111"
MINT_ADDRESS="<mint-address>"
CONFIG_PDA="<config-pda>"

echo "=== GHOST Token Daily Health Check ==="
echo "Date: $(date)"

# 1. Program status
echo -e "\n[1] Program status:"
solana program show $PROGRAM_ID | grep -E "Executable|Last Deployed"

# 2. Supply check
echo -e "\n[2] Supply:"
spl-token supply $MINT_ADDRESS --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# 3. Config state
echo -e "\n[3] Config state:"
solana account $CONFIG_PDA | head -20

# 4. Recent transactions
echo -e "\n[4] Recent transactions:"
solana transaction-history $PROGRAM_ID --limit 5

echo -e "\n=== Health check complete ==="
```

## Common Procedures

### Procedure 1: Mint Tokens

**Purpose:** Create new GHOST tokens

**Prerequisites:** Mint authority keypair

```bash
# 1. Verify current supply
spl-token supply <MINT_ADDRESS>

# 2. Verify protocol is not paused
solana account <CONFIG_PDA> # Check paused flag

# 3. Mint tokens
node scripts/mint.js \
  --amount <AMOUNT_IN_RAW_UNITS> \
  --mint <MINT_ADDRESS> \
  --recipient <DESTINATION_TOKEN_ACCOUNT> \
  --authority <MINT_AUTHORITY_KEYPAIR> \
  --url https://api.mainnet-beta.solana.com

# 4. Record transaction
echo "<TX_SIGNATURE> - Minted <AMOUNT> GHOST to <DESTINATION>" >> mint-log.txt

# 5. Verify new supply
spl-token supply <MINT_ADDRESS>
```

### Procedure 2: Rotate Mint Authority

**Purpose:** Change the mint authority to a new keypair

**Prerequisites:** Admin authority keypair

```bash
# Step 1: Initiate rotation
node scripts/initiate-rotation.js \
  --authority-type mint \
  --new-authority <NEW_MINT_AUTHORITY_PUBKEY> \
  --admin-keypair <ADMIN_KEYPAIR> \
  --url https://api.mainnet-beta.solana.com

# Step 2: Monitor pending rotation
node scripts/check-config.js | grep pending_rotation

# Step 3: Wait for timelock (24 hours on mainnet)
echo "Timelock expires at: $(date -d @${EFFECTIVE_AT})"

# Step 4: Confirm rotation (after timelock expires)
node scripts/confirm-rotation.js \
  --url https://api.mainnet-beta.solana.com

# Step 5: Verify new mint authority
spl-token display <MINT_ADDRESS> | grep "Mint authority"
```

### Procedure 3: Emergency Pause

**Purpose:** Halt all minting and authority rotations

**Prerequisites:** Emergency authority keypair

```bash
# 1. Pause protocol
node scripts/pause.js \
  --authority <EMERGENCY_KEYPAIR> \
  --url https://api.mainnet-beta.solana.com

# 2. Verify paused state
node scripts/check-config.js | grep paused
# Expected: paused: true

# 3. Announce pause
# → Twitter/Discord: "GHOST protocol has been paused. Investigation underway."

# 4. Investigate incident
# → Check recent mint transactions
# → Check recent authority rotation events
# → Review access logs for authority keypairs
```

### Procedure 4: Emergency Resume

**Purpose:** Restore normal operations after pause

**Prerequisites:** Emergency authority keypair, incident resolved

```bash
# 1. Verify incident is resolved
# → Incident report completed
# → Root cause addressed
# → No further risk

# 2. Resume protocol
node scripts/resume.js \
  --authority <EMERGENCY_KEYPAIR> \
  --url https://api.mainnet-beta.solana.com

# 3. Verify active state
node scripts/check-config.js | grep paused
# Expected: paused: false

# 4. Test operations
node scripts/mint.js --amount 1000 # Small test mint
node scripts/burn.js --amount 500  # Small test burn

# 5. Announce resume
# → Twitter/Discord: "GHOST protocol resumed. Incident report: [link]"
```

### Procedure 5: Program Upgrade

**Purpose:** Deploy a new version of the program

**Prerequisites:** Upgrade authority, tested build

```bash
# 1. Announce upgrade (minimum 48 hours notice)
# 2. Verify build
anchor build --verifiable
sha256sum target/deploy/ghost_token.so

# 3. Deploy upgrade
solana program deploy target/deploy/ghost_token.so \
  --program-id Gh0st111111111111111111111111111111111111111 \
  --keypair <UPGRADE_AUTHORITY_KEYPAIR> \
  --url https://api.mainnet-beta.solana.com \
  --with-compute-unit-price 50000

# 4. Verify deployment
solana program show Gh0st111111111111111111111111111111111111111

# 5. Run post-deployment tests
node scripts/verify-program.js

# 6. Announce completion
```

## Monitoring

### Key Metrics

| Metric | Check Frequency | Alert Threshold |
|--------|----------------|-----------------|
| Program executable | Every 5 min | Not executable |
| Total supply | Every 15 min | Unexpected increase |
| Config paused flag | Every 5 min | paused = true |
| Mint authority matches expected | Every hour | Changed |
| Upgrade authority matches expected | Every hour | Changed |
| Recent mint events | Every 15 min | Amount > 100M GHOST |
| Program upgrade events | Real-time | Any upgrade |
| Authority rotation events | Real-time | Any rotation |

### Monitoring Setup

```bash
# RPC WebSocket monitoring
#!/bin/bash
# monitor.sh

# Subscribe to program account changes
websocat "wss://api.mainnet-beta.solana.com" <<EOF
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "accountSubscribe",
  "params": [
    "<CONFIG_PDA>",
    {"encoding": "jsonParsed", "commitment": "confirmed"}
  ]
}
EOF
```

### Alert Channels

| Alert | Channel |
|-------|---------|
| Critical (pause, unauthorized mint) | PagerDuty + Discord + Telegram |
| High (authority rotation, upgrade) | Discord + Telegram |
| Medium (large mint, supply anomaly) | Discord |
| Low (health check failures) | Slack / Email |

## Incident Response

### Severity Levels

| Level | Definition | Response Time | Example |
|-------|-----------|---------------|---------|
| **P0 - Critical** | Unauthorized minting, program compromise | Immediate (any time) | Mint authority key leaked |
| **P1 - High** | Protocol paused, supply anomaly | < 1 hour | Unexpected pause detected |
| **P2 - Medium** | Degraded operations, monitoring alert | < 4 hours | Health check failure |
| **P3 - Low** | Minor issue, cosmetic | < 24 hours | Metadata update needed |

### Incident Response Flow

```
Alert Triggered
    │
    ▼
1. ACKNOWLEDGE: Responder acknowledges alert
    │
    ▼
2. TRIAGE: Determine severity (P0-P3)
    │
    ▼
3. CONTAIN: Pause protocol if necessary
    │
    ▼
4. INVESTIGATE: Determine root cause
    │
    ▼
5. RESOLVE: Apply fix
    │
    ▼
6. RECOVER: Verify state, resume if paused
    │
    ▼
7. POST-MORTEM: Document incident, improve processes
```

### Contact Information

| Role | Contact |
|------|---------|
| Primary On-Call | [REDACTED] |
| Secondary On-Call | [REDACTED] |
| Security Lead | security@ghost-token.io |
| Protocol Lead | protocol@ghost-token.io |

## Backup & Recovery

### Key Backups

| Item | Backup Location | Encryption |
|------|----------------|------------|
| Mint authority keypair | Hardware wallet + encrypted offline | AES-256 |
| Admin authority keypair | Hardware wallet + encrypted offline | AES-256 |
| Emergency authority keypair | Encrypted cloud + hardware wallet | AES-256 |
| Upgrade authority keypair | Hardware wallet + safe deposit box | AES-256 |
| Metadata authority keypair | Hardware wallet | AES-256 |

### State Backup

On-chain state is backed up by Solana validators. No additional backup needed for Config PDA and mint state.

### Recovery Testing

Quarterly recovery drills:
1. Simulate authority key loss
2. Execute rotation to new key
3. Verify all operations function with new key
4. Document any issues

## Useful Commands Reference

```bash
# Program info
solana program show Gh0st111111111111111111111111111111111111111

# Mint info
spl-token display <MINT> --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Supply
spl-token supply <MINT> --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Config PDA
solana find-program-derived-address Gh0st111111111111111111111111111111111111111 "config"
solana account <CONFIG_PDA>

# Transaction history
solana transaction-history <ADDRESS> --limit 50

# Token accounts
spl-token accounts --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Balance
spl-token balance <TOKEN_ACCOUNT> --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
```