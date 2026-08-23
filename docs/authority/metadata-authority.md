# Metadata Authority

## Overview

The **Metadata Authority** controls the on-chain metadata associated with the GHOST token. This is a Token-2022 metadata extension, separate from the program's Config PDA authorities.

## What It Controls

### Token-2022 Metadata Extension Fields

| Field | Description | Example Value |
|-------|-------------|---------------|
| **Name** | Token name (max 32 bytes) | "GHOST" |
| **Symbol** | Token symbol (max 10 bytes) | "GHOST" |
| **URI** | URI pointing to off-chain metadata JSON | "https://arweave.net/..." |
| **Additional Metadata** | Key-value pairs (future Token-2022 extension) | Various |

### What It DOES NOT Control

- ❌ **Token supply** — controlled by mint authority
- ❌ **Token minting** — controlled by mint authority
- ❌ **Protocol pause** — controlled by emergency authority
- ❌ **Program upgrades** — controlled by upgrade authority
- ❌ **Token transfers** — permissionless (SPL standard)
- ❌ **Token burning** — permissionless for holders

## Metadata Authority Setup

### Initial Setup (During Initialize)

```bash
# The metadata authority is set during mint initialization
spl-token initialize-metadata \
  <MINT_ADDRESS> \
  "GHOST" \
  "GHOST" \
  "https://arweave.net/placeholder-ghost-logo" \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb \
  --metadata-authority <METADATA_AUTHORITY_KEYPAIR>
```

### Verification

```bash
# Check metadata on-chain
spl-token display <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Read raw metadata from mint account
solana account <MINT_ADDRESS> --output json | jq .
```

## Updating Metadata

### Update Name/Symbol

```bash
# Update just the name
spl-token update-metadata <MINT_ADDRESS> \
  name "GHOST v2" \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Update just the symbol
spl-token update-metadata <MINT_ADDRESS> \
  symbol "GHOST" \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
```

### Update URI

```bash
# Update the metadata URI (e.g., new Arweave upload)
spl-token update-metadata <MINT_ADDRESS> \
  uri "https://arweave.net/new-ghost-metadata-hash" \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
```

### Add Custom Fields (Token-2022)

```bash
# Add custom key-value pairs (if supported by Token-2022)
spl-token update-metadata <MINT_ADDRESS> \
  field "website" "https://ghost-token.io" \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
```

## Freeze (Immutable Metadata)

### Making Metadata Immutable

Once the metadata is finalized, you should freeze it to prevent any changes:

```bash
# Remove metadata authority (freeze metadata forever)
spl-token authorize <MINT_ADDRESS> metadata \
  --disable \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Verify: metadata authority is now None
spl-token display <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
```

**When to freeze:**
- [ ] Token name is finalized
- [ ] Token symbol is finalized
- [ ] Metadata JSON on Arweave is permanent
- [ ] Logo/icon is final
- [ ] Community agrees on final metadata

**WARNING:** Freezing metadata is **IRREVERSIBLE**. The name, symbol, and URI can never be changed after freezing.

## URI Management

### Best Practices

1. **Use Arweave for permanence**
   ```
   ❌ https://ghost-token.io/metadata.json  (can be changed, server can go down)
   ✅ https://arweave.net/<permanent-hash>   (immutable, decentralized storage)
   ```

2. **Pin on IPFS as fallback**
   ```
   ipfs://Qm... (requires IPFS gateway)
   https://ipfs.io/ipfs/Qm... (public gateway)
   ```

3. **Use content-addressed URIs**
   - The URI hash changes when content changes
   - Verify the hash matches the expected content

### URI Update Procedure

```bash
# 1. Prepare new metadata JSON (metadata/token.json)
# 2. Upload to Arweave
arweave deploy metadata/token.json --wallet arweave-wallet.json

# 3. Record the new URI
# https://arweave.net/<new-tx-hash>

# 4. Update on-chain (requires metadata authority)
spl-token update-metadata <MINT_ADDRESS> \
  uri "https://arweave.net/<new-tx-hash>" \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# 5. Verify
spl-token display <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# 6. Announce change publicly
```

## Security Considerations

### Risk Level: LOW

| If Compromised | Impact |
|----------------|--------|
| Attacker changes token name | Confusion (not financial loss) |
| Attacker changes token symbol | Confusion (wallets/exchanges may flag) |
| Attacker changes URI | Users see wrong image/description |

While the financial impact is low, reputation damage can be significant.

### Protection

1. **Separate key from program authorities**
   - Metadata authority ≠ mint authority
   - Metadata authority ≠ admin authority

2. **Freeze when ready**
   - Remove metadata authority after final content is settled
   - This eliminates the attack vector entirely

3. **Monitor for changes**
   - Alert on any metadata authority transactions
   - Verify metadata content matches expectations

## Comparison: Token-2022 vs Metaplex Metadata

| Feature | Token-2022 Metadata | Metaplex Token Metadata |
|---------|---------------------|------------------------|
| Storage | On-chain in mint account | Separate Metaplex metadata account |
| Fields | Name, Symbol, URI | Name, Symbol, URI + additional attributes |
| Standard | SPL Token-2022 extension | Metaplex standard (de facto) |
| Explorer support | Native in Solana Explorer | Most wallets/explorers |
| DEX support | Growing | Universal |
| Cost | No extra account (in mint) | Extra rent-exempt account |
| Update | Token-2022 CLI/SDK | Metaplex SDK |
| Freeze | Disable metadata authority | Set update authority to None |

### Current GHOST Token Approach

GHOST uses **Token-2022 metadata** as the primary metadata. A Metaplex compatibility layer may be added in the future if needed for better wallet/DEX integration.

## Off-Chain Metadata JSON Reference

The URI points to a JSON file following this schema:

```json
{
  "name": "GHOST",
  "symbol": "GHOST",
  "description": "A meme token called GHOST.",
  "image": "https://arweave.net/placeholder-ghost-logo",
  "external_url": "https://ghost-token.io",
  "attributes": [
    {"trait_type": "category", "value": "meme"},
    {"trait_type": "blockchain", "value": "solana"},
    {"trait_type": "token_standard", "value": "token-2022"}
  ]
}
```

See `metadata/token.json` for the canonical metadata file.