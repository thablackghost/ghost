# Token Metadata Documentation

## Overview

GHOST token metadata is managed using the SPL Token-2022 Metadata Extension. This is the native metadata solution for Token-2022 tokens on Solana.

## Token-2022 Metadata vs Metaplex

### Token-2022 Metadata Extension

```
Token Mint Account (owned by TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb)
├── Base Mint Data
│   ├── mint_authority
│   ├── supply
│   ├── decimals (9)
│   └── is_initialized
│
└── Metadata Extension
    ├── name: "GHOST" (32 bytes max)
    ├── symbol: "GHOST" (10 bytes max)
    ├── uri: "https://arweave.net/..." (200 bytes max)
    ├── update_authority: <pubkey>
    └── additional_fields: [] (optional key-value pairs)
```

### Key Differences

| Aspect | Token-2022 Metadata | Metaplex Token Metadata |
|--------|---------------------|------------------------|
| **Account Model** | Extension of mint account | Separate PDA account |
| **Program** | `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` | `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s` |
| **Cost** | Included in mint (no extra rent) | ~0.01 SOL for metadata account |
| **Field Limits** | Name: 32B, Symbol: 10B | Name: 32B, Symbol: 10B |
| **Attributes** | Limited (via custom fields) | Rich attribute array |
| **Creator Royalties** | Not supported | Supported |
| **Collections** | Not supported | Supported |
| **Explorer Display** | Native | Widely supported |
| **DEX Integration** | Emerging | Mature |
| **Standard** | SPL official | Community de facto |

### GHOST Token Approach

GHOST uses **Token-2022 metadata** as the primary and only metadata source. This decision was made for:

1. **Simplicity**: No extra accounts to manage
2. **Cost efficiency**: No extra rent-exempt fees
3. **Native support**: Works with Solana Explorer natively
4. **Official standard**: Backed by Solana Labs

If broader ecosystem compatibility is needed (e.g., DEXs that only read Metaplex), a Metaplex metadata account can be created later as a secondary mirror.

## URI Format

### Current URI Structure

```
https://arweave.net/<transaction-hash>
```

### Why Arweave

| Feature | Arweave | IPFS | Centralized HTTP |
|---------|---------|------|-----------------|
| Permanence | Forever (~200 years) | Pin-dependent | As long as server runs |
| Decentralization | Arweave network | Pinned nodes | Single server |
| Immutability | Content-addressed | Content-addressed | Mutable |
| Cost | One-time payment | Free (with pinning) | Ongoing hosting |
| Speed | Fast (gateways) | Variable | Fast |
| Gateway risk | Multiple gateways | Multiple gateways | Single server |

### Alternative URIs

```bash
# Arweave (recommended)
https://arweave.net/abc123def456

# IPFS (fallback)
ipfs://QmSomeHashHere
https://ipfs.io/ipfs/QmSomeHashHere

# Centralized (not recommended for production)
https://ghost-token.io/metadata.json
```

## JSON Schema

The metadata JSON hosted at the URI must follow this structure:

```json
{
  "$schema": "https://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["name", "symbol", "description", "image"],
  "properties": {
    "name": {
      "type": "string",
      "description": "Token name",
      "maxLength": 32
    },
    "symbol": {
      "type": "string",
      "description": "Token symbol",
      "maxLength": 10
    },
    "description": {
      "type": "string",
      "description": "Token description",
      "maxLength": 500
    },
    "image": {
      "type": "string",
      "format": "uri",
      "description": "URL to token logo/image"
    },
    "external_url": {
      "type": "string",
      "format": "uri",
      "description": "Project website URL"
    },
    "attributes": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["trait_type", "value"],
        "properties": {
          "trait_type": {"type": "string"},
          "value": {"type": "string"}
        }
      }
    }
  }
}
```

### Field Descriptions

| Field | Required | Max Length | Description |
|-------|----------|------------|-------------|
| `name` | ✅ | 32 chars | Token name (must match on-chain name) |
| `symbol` | ✅ | 10 chars | Token ticker (must match on-chain symbol) |
| `description` | ✅ | 500 chars | Human-readable token description |
| `image` | ✅ | 200 chars | URL to token logo (PNG/SVG/GIF, recommended 512x512) |
| `external_url` | ❌ | 200 chars | Link to project website |
| `attributes` | ❌ | Varies | Array of trait objects |

## Update Procedure

### On-Chain Update (Metadata Authority)

```bash
# 1. Update metadata on-chain
spl-token update-metadata <MINT_ADDRESS> \
  name "GHOST" \
  symbol "GHOST" \
  uri "https://arweave.net/<new-hash>" \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# 2. Verify
spl-token display <MINT_ADDRESS> \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
```

### Off-Chain Update (URI Content)

```bash
# 1. Edit metadata/token.json
# 2. Upload to Arweave
arweave deploy metadata/token.json

# 3. Record new URI
# https://arweave.net/<tx-id>

# 4. Update on-chain URI (as shown above)
```

## Metadata Freeze

### Making Metadata Immutable

After finalizing the metadata, you can remove the metadata authority:

```bash
spl-token authorize <MINT_ADDRESS> metadata --disable \
  --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
```

After this:
- Name cannot be changed
- Symbol cannot be changed
- URI cannot be changed (the on-chain pointer is frozen)
- **BUT** the off-chain content at the URI can still change if hosting is mutable (e.g., HTTP)
- Use Arweave for truly immutable metadata end-to-end

## Best Practices

### Image Requirements
- **Format**: PNG or SVG (lossless), WebP (lossy but small)
- **Resolution**: 512x512 or larger
- **Aspect ratio**: 1:1 (square)
- **Background**: Transparent PNG preferred
- **File size**: Under 1MB for fast loading
- **Hosting**: Arweave or IPFS (immutable, decentralized)

### Description Guidelines
- Clear, concise description (1-3 sentences)
- Include category (meme token)
- Mention blockchain (Solana)
- Mention token standard (SPL Token-2022)
- Avoid financial advice or price predictions

### Attributes
Use attributes for discoverability:

```json
{
  "attributes": [
    {"trait_type": "category", "value": "meme"},
    {"trait_type": "blockchain", "value": "solana"},
    {"trait_type": "token_standard", "value": "token-2022"},
    {"trait_type": "decimals", "value": "9"},
    {"trait_type": "mint_policy", "value": "unlimited"}
  ]
}
```

## Troubleshooting

### Explorer Not Showing Metadata

1. **Verify on-chain data**: `spl-token display <MINT_ADDRESS> --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`
2. **Check URI is accessible**: `curl <uri>` returns valid JSON
3. **Clear explorer cache**: Explorer may cache metadata for ~1 hour
4. **Check for Metaplex fallback**: Some explorers prioritize Metaplex metadata

### URI Too Long

- Token-2022 URI field maximum is 200 bytes
- Use URL shortener or content-addressed hashes
- Arweave transaction IDs are ~43 characters — well within limits
- Avoid long query parameters in URIs