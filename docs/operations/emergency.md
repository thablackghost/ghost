# Emergency Procedures

## Overview

This document defines the emergency procedures for the GHOST token program. These procedures are designed to be executed quickly and decisively in crisis situations.

## Emergency Contacts

| Role | Name | Contact Method | Response SLA |
|------|------|---------------|--------------|
| **Primary Emergency Responder** | [TBD] | PagerDuty / Phone | 5 minutes |
| **Secondary Responder** | [TBD] | Phone | 15 minutes |
| **Security Lead** | [TBD] | Signal / Email | 30 minutes |
| **Protocol Lead** | [TBD] | Signal / Email | 30 minutes |
| **Community Manager** | [TBD] | Discord / Twitter | 1 hour |

**Internal Contact Sheet:** [REDACTED — Internal Only]

## Emergency Authority

The emergency authority can pause and resume the protocol. This is a **hot wallet** designed for rapid access.

### Key Location
- Primary: Encrypted on secure device
- Backup: Encrypted offline storage
- Access: 2 of 3 team members required (if multisig)

### Capabilities
| Action | Can Do |
|--------|--------|
| Pause protocol | ✅ Yes |
| Resume protocol | ✅ Yes |
| Mint tokens | ❌ No |
| Rotate authorities | ❌ No |
| Upgrade program | ❌ No |

## Scenario 1: Unauthorized Minting Detected

### Severity: P0 — CRITICAL

**Symptoms:**
- Unexpected supply increase
- Mint transactions from unknown origin
- Monitoring alert: "Unauthorized mint detected"

### Immediate Actions (First 5 minutes)

```bash
# 1. PAUSE THE PROTOCOL IMMEDIATELY
node scripts/pause.js \
  --authority <EMERGENCY_KEYPAIR> \
  --url https://api.mainnet-beta.solana.com

# 2. Verify pause took effect
node scripts/check-config.js | grep paused
# MUST return: paused: true

# 3. Alert team
# → PagerDuty: "CRITICAL: Unauthorized mint detected. Protocol paused."
# → Discord Emergency Channel: @everyone
```

### Investigation (5-30 minutes)

```bash
# 1. Identify unauthorized mint transactions
solana transaction-history <MINT_ADDRESS> --limit 100

# 2. Calculate damage
BEFORE=$(spl-token supply <MINT_ADDRESS> --before <incident-slots>)
AFTER=$(spl-token supply <MINT_ADDRESS>)
echo "Unauthorized mint amount: $((AFTER - BEFORE))"

# 3. Trace destination accounts
# → Where were the tokens sent?
# → Any DEX swaps or transfers after mint?

# 4. Determine cause
# → Was the mint authority key compromised?
# → Was there a program bug?
# → Was there an insider threat?
```

### Recovery

```bash
# 1. Rotate mint authority to new secure key
node scripts/initiate-rotation.js \
  --authority-type mint \
  --new-authority <NEW_SECURE_PUBKEY> \
  --admin-keypair <ADMIN_KEYPAIR>

# 2. Rotate emergency authority (if compromised)
node scripts/initiate-rotation.js \
  --authority-type emergency \
  --new-authority <NEW_EMERGENCY_PUBKEY> \
  --admin-keypair <ADMIN_KEYPAIR>

# 3. Wait for timelock to expire
# TIMELOCK: 24 hours on mainnet

# 4. Confirm rotations
node scripts/confirm-rotation.js

# 5. Resume protocol
node scripts/resume.js --authority <EMERGENCY_KEYPAIR>

# 6. Publish post-mortem
```

### Communication Template

```
🚨 GHOST TOKEN EMERGENCY NOTICE 🚨

Date: [DATE]
Time: [TIME UTC]
Status: RESOLVED / ONGOING

What happened:
[MINT_AMOUNT] GHOST tokens were minted without authorization.

Actions taken:
1. Protocol paused immediately at [TIME]
2. Mint authority rotated to new secure key
3. Emergency authority rotated
4. Protocol resumed at [TIME]

Impact:
- Unauthorized mint: [AMOUNT] GHOST
- Current supply: [SUPPLY]
- No user funds affected

Next steps:
- Full security audit
- Additional authority security measures
- Incident report: [LINK]

We will provide further updates as the investigation continues.
```

## Scenario 2: Mint Authority Key Compromised

### Severity: P0 — CRITICAL

**Symptoms:**
- Mint authority private key found in unauthorized location
- Suspicious access to key storage
- Team member with key access terminated

### Actions

```bash
# 1. PAUSE immediately (even if no unauthorized mint yet)
node scripts/pause.js --authority <EMERGENCY_KEYPAIR>

# 2. Rotate mint authority
node scripts/initiate-rotation.js \
  --authority-type mint \
  --new-authority <BRAND_NEW_PUBKEY> \
  --admin-keypair <ADMIN_KEYPAIR>

# 3. Generate new key COMPLETELY offline
# On air-gapped machine:
solana-keygen new --outfile new-mint-authority.json --no-bip39-passphrase
# NEVER connect this machine to the internet

# 4. Wait for timelock
# 5. Confirm rotation
# 6. Resume

# 7. Revoke all access to compromised key
# 8. Security audit of all other keys
```

## Scenario 3: Program Vulnerability Discovered

### Severity: P1 — HIGH (if exploitable), P2 — MEDIUM (if theoretical)

**Symptoms:**
- Bug report from security researcher
- Internal code review finding
- Audit finding

### Actions

```bash
# 1. Assess exploitability
# → Can the vulnerability be triggered by ANY user?
# → What is the impact (theft, freeze, mint)?
# → Is it already being exploited?

# 2. If exploitable and severe:
# PAUSE protocol
node scripts/pause.js --authority <EMERGENCY_KEYPAIR>

# 3. Develop fix
# → Create patch in new branch
# → All tests passing
# → Security review of fix

# 4. Deploy fix
anchor build --verifiable
solana program deploy target/deploy/ghost_token.so \
  --program-id Gh0st111111111111111111111111111111111111111

# 5. Verify fix doesn't introduce new issues
anchor test

# 6. Resume
node scripts/resume.js --authority <EMERGENCY_KEYPAIR>

# 7. Publish security advisory
```

## Scenario 4: Upgrade Authority Compromised

### Severity: P0 — CRITICAL

**Symptoms:**
- Upgrade authority key found in unauthorized location
- Suspicious program upgrade

### Actions

```bash
# 1. Check if malicious upgrade occurred
solana program show Gh0st111111111111111111111111111111111111111
# Compare "Last Deployed Slot" with known good deployment

# 2. If malicious upgrade detected:
# PAUSE immediately
node scripts/pause.js

# 3. Rotate upgrade authority (if still have key)
solana program set-upgrade-authority \
  Gh0st111111111111111111111111111111111111111 \
  --new-upgrade-authority <NEW_SECURE_PUBKEY>

# 4. Redeploy known good version
git checkout <last-known-good-commit>
anchor build
solana program deploy target/deploy/ghost_token.so \
  --program-id Gh0st111111111111111111111111111111111111111

# 5. Verify state integrity
# → Check supply hasn't changed
# → Check Config PDA values are correct
# → Check all authority pubkeys match expected

# 6. Resume
node scripts/resume.js
```

## Scenario 5: Network-Level Attack

### Severity: P2 — MEDIUM (Solana-level, outside direct control)

**Symptoms:**
- Solana network congestion
- Transaction failures
- RPC unavailability

### Actions

```bash
# 1. Assess Solana network status
solana cluster-version
solana slot

# 2. If network is degraded but functional:
# → Increase transaction priority fees
# → Use fallback RPC endpoints
# → Monitor until network recovers

# 3. If network is down:
# → No action possible on-chain
# → Communicate estimated recovery time
# → Monitor Solana status page

# 4. After network recovery:
# → Verify program state
# → Replay any pending transactions
# → Publish status update
```

## Scenario 6: DEX / Liquidity Emergency

### Severity: P1 — HIGH

**Symptoms:**
- Extreme price volatility
- Liquidity pool drained
- DEX integration broken

### Actions

**Note:** The program cannot control DEX behavior. These are off-chain actions.

```bash
# 1. Assess on-chain impact
spl-token supply <MINT_ADDRESS>
# → Verify no unauthorized minting

# 2. Communicate with DEX partners
# → Coordinate response
# → Consider temporary liquidity withdrawal

# 3. If price manipulation via minting:
# PAUSE protocol
node scripts/pause.js

# 4. Community communication
# → Acknowledge situation
# → Share facts, not speculation
# → Provide regular updates
```

## Recovery Checklist

After any emergency:

- [ ] Root cause identified and documented
- [ ] Fix implemented and tested
- [ ] All compromised keys rotated
- [ ] Program state verified (supply, authorities, config)
- [ ] Protocol resumed (if applicable)
- [ ] Post-mortem written
- [ ] Community notified of resolution
- [ ] Security improvements implemented
- [ ] Test cases added for discovered vulnerability
- [ ] Monitoring alerts updated

## Post-Mortem Template

```markdown
# Incident Post-Mortem: [TITLE]

## Summary
[1-2 sentence summary]

## Timeline
- [TIME UTC]: [Event]
- [TIME UTC]: [Response action]
- [TIME UTC]: [Resolution]

## Root Cause
[Technical explanation]

## Impact
- Duration: [DURATION]
- Users affected: [COUNT or "None"]
- Financial impact: [AMOUNT or "None"]
- Supply impact: [AMOUNT or "None"]

## What Went Well
- [Positive aspect]

## What Went Wrong
- [Negative aspect]

## Action Items
- [ ] [Action 1]
- [ ] [Action 2]

## Lessons Learned
[Key takeaways]
```

## Emergency Drills

### Schedule

| Drill | Frequency | Participants |
|-------|-----------|-------------|
| Pause/Resume | Monthly | Emergency responders |
| Authority rotation | Quarterly | Admin + emergency |
| Key compromise simulation | Quarterly | All team |
| Program upgrade rollback | Semi-annual | Dev + ops |

### Drill Procedure

1. Announce drill (internal only)
2. Execute scenario
3. Time each response step
4. Document issues encountered
5. Update procedures based on findings
6. File drill report

## Emergency Kit

Keep a secure, accessible emergency kit:

```
emergency-kit/
├── emergency-authority-keypair.json (ENCRYPTED)
├── admin-authority-keypair.json (ENCRYPTED)
├── scripts/
│   ├── pause.js
│   ├── resume.js
│   ├── initiate-rotation.js
│   ├── confirm-rotation.js
│   ├── check-config.js
│   └── verify-program.js
├── contacts.md
├── communication-templates.md
└── recovery-checklist.md
```