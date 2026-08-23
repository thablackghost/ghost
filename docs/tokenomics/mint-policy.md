# Mint Policy

## Overview

The GHOST token has an unlimited minting model. This document defines the policies, procedures, and principles governing minting operations. These are **off-chain policies** enforced by the mint authority holder — the on-chain program does not enforce mint limits.

## Authority

### Who Can Mint

| Role | Minting Capability |
|------|-------------------|
| **Mint Authority** (on-chain) | Unlimited — the ONLY entity that can mint |
| Admin Authority | Cannot mint |
| Emergency Authority | Cannot mint |
| Token Holders | Cannot mint |

The mint authority is stored in `Config.mint_authority` and can be rotated by the admin authority.

### Current Mint Authority

- **Devnet:** Development keypair (see `devnet/configs/devnet-config.json`)
- **Mainnet:** To be announced at launch

## Minting Principles

### Transparency

1. **All mints are on-chain**: Every mint transaction is publicly visible on Solana
2. **Pre-announcement**: Significant mints (>1% of supply) should be announced 24+ hours in advance
3. **Transaction linking**: Mint transaction signatures should be shared publicly
4. **Supply reporting**: Current supply should be easily verifiable

### Accountability

1. **Defined purpose**: Every mint should have a documented reason
2. **Amount justification**: The amount minted should be proportional to the need
3. **Recipient identification**: The destination address should be publicly associated with the purpose
4. **Post-mint reporting**: A summary should be published after significant mints

### Security

1. **Multisig preferred**: Mint authority should be a multisig (e.g., Squads 3-of-5) for mainnet
2. **Transaction review**: All signers should review mint transactions before signing
3. **Amount limits**: Internal soft cap per transaction (e.g., 100M GHOST per tx)
4. **Rate limiting**: No more than one significant mint per 24-hour period
5. **Emergency pause ready**: Emergency authority available to halt minting if compromise suspected

## Minting Scenarios

### 1. Liquidity Provision

**Purpose:** Add liquidity to DEX pools (Raydium, Orca, Jupiter)

**Typical Amount:** Market-dependent

**Procedure:**
1. Announce intent to add liquidity
2. Determine required token amount
3. Mint tokens to liquidity provider account
4. Create/pair with USDC or SOL on DEX
5. Publish transaction signatures and pool address

### 2. Community Incentives

**Purpose:** Airdrops, staking rewards, community events

**Typical Amount:** Based on program design

**Procedure:**
1. Announce incentive program with eligibility criteria
2. Calculate total required tokens
3. Mint tokens to distribution account
4. Execute distribution (airdrop contract, merkle distributor)
5. Publish distribution results

### 3. Ecosystem Grants

**Purpose:** Development grants, partnerships, integrations

**Typical Amount:** Negotiated per grant

**Procedure:**
1. Grant proposal review and approval
2. Announce grant publicly
3. Mint grant amount to recipient address
4. Recipient acknowledged publicly

### 4. Treasury Top-Up

**Purpose:** Maintain operational treasury

**Typical Amount:** Budget-based

**Procedure:**
1. Quarterly budget review
2. Mint budgeted amount to treasury multisig
3. Publish budget breakdown

## Minting Schedule (Recommended)

| Type | Frequency | Pre-announcement | Post-report |
|------|-----------|-----------------|-------------|
| Small (< 10M GHOST) | As needed | Optional | Transaction link |
| Medium (10M - 100M) | Max 1/day | 24 hours | Tweet + tx link |
| Large (100M - 1B) | Max 1/week | 48 hours | Blog post + data |
| Significant (> 1B) | Exceptional | 7 days | Full transparency report |

## Audit Trail

### On-Chain Audit

Every mint transaction is permanently recorded:

```
Transaction: <signature>
  Instruction: MintTokens
  Amount: X GHOST
  Destination: <token-account>
  Timestamp: <block-time>
  Signer: <mint-authority>
```

### Off-Chain Log

Maintain a minting log:

```json
[
  {
    "id": 1,
    "date": "2024-01-15",
    "amount": "10000000.000000000",
    "purpose": "Liquidity pool seeding",
    "recipient": "<address>",
    "tx_signature": "<signature>",
    "announced": true,
    "announcement_url": "https://..."
  }
]
```

## Supply Dashboard

A public dashboard should track:

- **Total Supply**: Real-time from on-chain
- **Circulating Supply**: Total - burned - known non-circulating
- **Mint History**: Chart of cumulative mints over time
- **Recent Mints**: Table of recent mint transactions
- **Authority Status**: Current mint authority, pending rotations

## Verification

### For Token Holders

```bash
# Check current supply
spl-token supply <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Check mint history
solana transaction-history <MINT_ADDRESS> --limit 100
```

### For Exchanges / Data Aggregators

```javascript
// Monitor for mint events
const subscriptionId = connection.onLogs(
  mintAddress,
  ({ logs, signature }) => {
    if (logs.includes('Instruction: MintTo')) {
      // Parse and record mint event
    }
  }
);
```

## Emergency Procedures

### If Unauthorized Minting Detected

1. **Verify** the mint transaction is indeed unauthorized
2. **Pause** the protocol via emergency authority
3. **Investigate** how the mint authority was compromised
4. **Rotate** mint authority to a new secure key
5. **Assess** impact (how many tokens were minted?)
6. **Communicate** transparently with the community
7. **Resume** protocol after rotation is confirmed

### If Excessive Minting is Requested

The multisig structure (if implemented) prevents single-actor excessive minting:
- Each signer independently evaluates the mint request
- No single signer can force a mint through
- Disagreement results in no mint

## Governance

### Future: Community-Governed Minting

In the future, minting decisions could transition to community governance:

1. **Proposal**: Community member proposes a mint
2. **Discussion**: Forum debate on necessity and amount
3. **Vote**: Token-weighted vote on proposal
4. **Execution**: If passed, multisig executes the mint

This requires:
- Governance token or token-based voting
- Governance program (e.g., Realms)
- Integration with mint authority multisig

## Policy Updates

This mint policy is a living document. Updates require:
- Public proposal of changes
- Community feedback period (minimum 7 days)
- Core team review and approval
- Version bump and changelog update

### Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2024-01-01 | Initial mint policy |