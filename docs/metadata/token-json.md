# Token JSON Specification

## Overview

This document specifies the JSON metadata schema for the GHOST token. This JSON file is hosted off-chain (preferably on Arweave) and linked from the on-chain Token-2022 metadata URI field.

## Canonical File

The canonical metadata JSON is stored at `metadata/token.json` in this repository.

## JSON Structure

```json
{
  "name": "GHOST",
  "symbol": "GHOST",
  "description": "A meme token called GHOST.",
  "image": "https://arweave.net/placeholder-ghost-logo",
  "external_url": "https://ghost-token.io",
  "attributes": [
    {
      "trait_type": "category",
      "value": "meme"
    },
    {
      "trait_type": "blockchain",
      "value": "solana"
    },
    {
      "trait_type": "token_standard",
      "value": "token-2022"
    }
  ]
}
```

## Field Reference

### `name`

| Property | Value |
|----------|-------|
| **Type** | `string` |
| **Required** | Yes |
| **Max Length** | 32 characters |
| **Description** | The human-readable name of the token |

**Constraints:**
- Must match the on-chain Token-2022 `name` field exactly
- UTF-8 encoded
- No leading/trailing whitespace
- Case-sensitive: "GHOST" not "ghost" or "Ghost"

### `symbol`

| Property | Value |
|----------|-------|
| **Type** | `string` |
| **Required** | Yes |
| **Max Length** | 10 characters |
| **Description** | The token ticker symbol |

**Constraints:**
- Must match the on-chain Token-2022 `symbol` field exactly
- Uppercase alphanumeric only (convention, not enforced)
- No spaces or special characters

### `description`

| Property | Value |
|----------|-------|
| **Type** | `string` |
| **Required** | Yes |
| **Max Length** | 500 characters (recommended) |
| **Description** | A human-readable description of the token |

**Best Practices:**
- Start with what the token IS
- Mention key features (Token-2022, unlimited minting)
- Keep it factual, not promotional
- Avoid price predictions or financial claims
- Avoid comparisons to other projects

**Example:**
```
"A meme token called GHOST. Built on Solana using the SPL
Token-2022 standard with 9 decimals and unlimited minting
controlled by a designated mint authority."
```

### `image`

| Property | Value |
|----------|-------|
| **Type** | `string` (URI) |
| **Required** | Yes |
| **Max Length** | 200 characters (practical URL limit) |
| **Description** | URL to the token's logo/image |

**Specifications:**
- Recommended: 512x512 PNG with transparent background
- Acceptable: SVG, WebP, JPEG, PNG, GIF
- Hosting: Arweave (preferred), IPFS (acceptable), HTTPS (least preferred)
- Content-addressed URIs recommended for immutability

**Example values:**
```
"https://arweave.net/abc123def456ghostlogo"
"ipfs://QmHashHereForGhostLogo"
"https://ghost-token.io/images/logo.png"  (not recommended)
```

### `external_url`

| Property | Value |
|----------|-------|
| **Type** | `string` (URI) |
| **Required** | No |
| **Max Length** | 200 characters |
| **Description** | Link to the project's website or primary resource |

**Example:**
```
"https://ghost-token.io"
```

### `attributes`

| Property | Value |
|----------|-------|
| **Type** | `array` of `{trait_type, value}` objects |
| **Required** | No |
| **Description** | Key-value pairs describing token traits |

**Standard Attributes for GHOST:**

| trait_type | value | Purpose |
|------------|-------|---------|
| `category` | `meme` | Token category for filtering |
| `blockchain` | `solana` | Primary blockchain |
| `token_standard` | `token-2022` | SPL standard used |
| `decimals` | `9` | Token decimal places |
| `mint_policy` | `unlimited` | Minting policy |
| `initial_supply` | `10000000000` | Initial token supply |

**Constraints:**
- `trait_type`: string, max 32 characters
- `value`: string, max 64 characters
- No duplicate `trait_type` values
- Order is not significant

## Complete Example (All Fields)

See `metadata/examples/metadata_example.json`:

```json
{
  "name": "GHOST",
  "symbol": "GHOST",
  "description": "A meme token called GHOST. Built on Solana using SPL Token-2022. Features unlimited minting, emergency pause, and authority rotation security.",
  "image": "https://arweave.net/placeholder-ghost-logo",
  "external_url": "https://ghost-token.io",
  "attributes": [
    {"trait_type": "category", "value": "meme"},
    {"trait_type": "blockchain", "value": "solana"},
    {"trait_type": "token_standard", "value": "token-2022"},
    {"trait_type": "decimals", "value": "9"},
    {"trait_type": "initial_supply", "value": "10000000000"},
    {"trait_type": "mint_policy", "value": "unlimited"},
    {"trait_type": "program_id", "value": "Gh0st111111111111111111111111111111111111111"}
  ]
}
```

## Validation

### JSON Schema Validation

Validate against the schema:

```bash
# Using ajv (Node.js)
npx ajv validate -s metadata/schema.json -d metadata/token.json

# Using Python
python -c "import json; json.load(open('metadata/token.json'))"
```

### Field-Level Validation

```javascript
// Validation rules
const validators = {
  name: (v) => typeof v === 'string' && v.length > 0 && v.length <= 32,
  symbol: (v) => typeof v === 'string' && v.length > 0 && v.length <= 10,
  description: (v) => typeof v === 'string' && v.length > 0 && v.length <= 500,
  image: (v) => typeof v === 'string' && v.startsWith('https://') || v.startsWith('ipfs://'),
  external_url: (v) => v === undefined || (typeof v === 'string' && v.startsWith('https://')),
  attributes: (v) => v === undefined || (Array.isArray(v) && v.every(a =>
    typeof a.trait_type === 'string' && typeof a.value === 'string'
  )),
};
```

## Versioning

The metadata JSON does not have an explicit version field. Changes are tracked by the Arweave transaction hash:

- **v1**: Initial metadata (placeholder image)
- **v2+**: Future updates with final artwork

Each update creates a new Arweave transaction with a new hash, and the on-chain URI is updated to point to the new hash.

## Hosting Requirements

| Aspect | Requirement |
|--------|-------------|
| Availability | 99.9%+ uptime |
| Latency | < 500ms global response |
| Immutability | Content-addressed storage (Arweave/IPFS) |
| CORS | `Access-Control-Allow-Origin: *` |
| Content-Type | `application/json` |
| Cache | 24-hour cache recommended (ETag/If-None-Match support) |

## Integration Points

### Wallets
- Phantom, Solflare, Backpack read Token-2022 metadata for token display
- Wallets may cache metadata for hours — plan for propagation delay

### Explorers
- Solana Explorer: Reads Token-2022 metadata natively
- Solscan: Reads both Token-2022 and Metaplex metadata
- SolanaFM: Reads both formats
- Birdeye: Reads Metaplex, falls back to Token-2022

### DEXs
- Jupiter: Token list + on-chain metadata
- Raydium: Token list + on-chain metadata
- Orca: Token list + Metaplex metadata

## Security

- **Content Integrity**: Use Arweave transaction hash as verification
- **Update Authorization**: Only metadata authority can update on-chain pointer
- **Freeze**: Disable metadata authority after final content
- **Supply Chain**: Verify JSON content before uploading to Arweave