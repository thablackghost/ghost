# Supply Documentation

## Token Supply Model

### Overview

GHOST is an inflationary meme token with no fixed supply cap. The token was launched with an initial supply of 10 billion (10,000,000,000) GHOST tokens. Additional tokens can be minted at any time by the mint authority.

## Token Details

| Property | Value |
|----------|-------|
| **Token Name** | GHOST |
| **Token Symbol** | GHOST |
| **Decimals** | 9 |
| **Initial Supply** | 10,000,000,000 (10 billion) |
| **Initial Supply (raw)** | 10,000,000,000,000,000,000 (10B × 10^9) |
| **Maximum Supply** | Unlimited (no cap) |
| **Mint Authority** | Controlled by Config PDA mint authority |
| **Token Standard** | SPL Token-2022 |

## Initial Supply Allocation

The initial 10 billion GHOST tokens were minted during the `Initialize` instruction to a single token account controlled by the team.

```
10,000,000,000 GHOST
    │
    └──► Initial Token Account (team-controlled)
            │
            ├──► Liquidity Pool (future)
            ├──► Community Distribution (future)
            ├──► Team Allocation (future)
            └──► Treasury (future)
```

**Note:** The initial supply allocation strategy is off-chain. The on-chain program only controls minting mechanics, not distribution.

## Unlimited Minting Model

### Philosophy

GHOST embraces unlimited minting as a feature, not a bug. As a meme token, the supply is flexible and can be adjusted based on community needs, market conditions, and ecosystem requirements.

### Who Can Mint

| Actor | Can Mint? | Conditions |
|-------|-----------|------------|
| Mint Authority | ✅ Yes | Unlimited amounts, any time |
| Admin Authority | ❌ No | Cannot mint |
| Emergency Authority | ❌ No | Cannot mint |
| Token Holders | ❌ No | Cannot mint (can only burn own tokens) |

### Minting Mechanism

```
Mint Authority signs transaction
    │
    ▼
Ghost Token Program validates:
    - Signer == config.mint_authority
    - Protocol is not paused
    - Mint account matches config.mint
    ▼
CPI to Token-2022: mint_to()
    ▼
Supply increases by minted amount
Tokens credited to destination account
```

### Transparency

All minting operations are:
- **On-chain**: Every mint transaction is recorded on Solana
- **Verifiable**: Supply can be checked via `spl-token supply` or Explorer
- **Auditable**: Mint history is available via transaction history
- **Tracked**: Events emitted for off-chain monitoring

## Supply Tracking

### Current Supply

```bash
# Check current supply
spl-token supply <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Output format:
# 10000000000000000000 (raw)
# = 10,000,000,000.000000000 GHOST (display)
```

### Circulating Supply

```
Circulating Supply = Total Supply - Burned Tokens

Where:
- Total Supply = tracked by Token-2022 mint account
- Burned Tokens = tokens sent to burn addresses or burned via burn()
```

### Supply History

The supply can only increase (via mint) or decrease (via burn):

```
Initial:    10,000,000,000 GHOST
Mint:      + X GHOST (mint authority decides)
Burn:      - Y GHOST (any holder can burn)
Current:   10,000,000,000 + Σ(mints) - Σ(burns)
```

## Why No Supply Cap?

### Design Rationale

1. **Meme Token Nature**: GHOST doesn't pretend to be a deflationary store of value. It's transparent about its inflationary design.

2. **Flexibility**: Unlimited minting allows the protocol to adapt to future needs:
   - Liquidity incentives
   - Community rewards
   - Ecosystem grants
   - Partnership allocations

3. **Simplicity**: No complex supply schedules, halving events, or emission curves. The mint authority decides.

4. **Trust Model**: Trust is placed in the mint authority. The program provides transparency (on-chain verification) and security (authority rotation, emergency pause).

### Comparison

| Token | Supply Model | Cap |
|-------|-------------|-----|
| GHOST | Unlimited minting | None |
| Bitcoin | Fixed schedule | 21,000,000 |
| USDC | Fiat-backed mint/burn | None (but backed) |
| DOGE | Fixed per-block + tail emission | None |
| SHIB | Fixed initial supply | 1 Quadrillion |
| PEPE | Fixed initial supply | 420.69 Trillion |

GHOST's model is most similar to DOGE — inflationary with no hard cap, controlled by a known authority.

## Supply API

### On-Chain

```rust
// Read supply from mint account
use spl_token_2022::state::Mint;

let mint_data = Mint::unpack(&mint_account.data.borrow())?;
let supply_raw: u64 = mint_data.supply;
let supply_display: f64 = supply_raw as f64 / 10f64.powi(9);
```

### Off-Chain (Solana RPC)

```bash
curl https://api.mainnet-beta.solana.com -X POST -H "Content-Type: application/json" -d '
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getTokenSupply",
  "params": ["<MINT_ADDRESS>"]
}'
```

### Explorer

```
https://explorer.solana.com/address/<MINT_ADDRESS>
→ Token Info → Current Supply
```

## Supply Events

Monitor these events for supply changes:

| Event | Meaning | Supply Effect |
|-------|---------|---------------|
| `TokensMinted(amount)` | New tokens created | Supply ↑ |
| `TokensBurned(amount)` | Tokens destroyed | Supply ↓ |
| `AuthorityRotationConfirmed(type: mint)` | Mint authority changed | New entity controls supply |

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Excessive inflation | Mint authority reputation; community monitoring; emergency pause |
| Hidden minting | All mints are on-chain and auditable |
| Mint authority compromise | Two-step rotation with timelock; emergency pause |
| Supply confusion | Clear communication; on-chain verification tools |
| Market manipulation | Transparent minting schedule; community oversight |

## FAQ

### "Is there a maximum supply?"
No. There is no hard cap on the total supply of GHOST tokens.

### "Can supply decrease?"
Yes. Any holder can burn their tokens, permanently reducing the supply.

### "How do I verify the supply?"
Use `spl-token supply <MINT_ADDRESS>` or check Solana Explorer.

### "Who decides when to mint?"
The mint authority (a designated keypair/multisig) decides when and how much to mint.

### "Can the mint authority be changed?"
Yes. The admin authority can initiate a rotation to a new mint authority with a timelock.

### "Can the unlimited minting be changed?"
Not without a program upgrade. The unlimited minting is hardcoded in the program logic.