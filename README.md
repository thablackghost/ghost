# BLACK GHOST (GHOST)

A meme token on Solana built with SPL Token-2022. Transparent, authority-controlled unlimited minting with comprehensive security features.

## Token Info

| Parameter | Value |
|-----------|-------|
| **Name** | BLACK GHOST |
| **Symbol** | GHOST |
| **Decimals** | 9 |
| **Supply** | 1,000,000,000 (1 billion) |
| **Raw Supply** | 1,000,000,000,000,000,000 |
| **Max Supply** | **Unlimited** (authority-controlled minting) |
| **Token Standard** | SPL Token-2022 |
| **Tax** | 0% — no hidden fees |
| **Website** | [thablackghost.com](https://thablackghost.com) |

## Contract

```
DK9MPoDFDPyCZG16ESkY6mUPrpKJaeGVpmEjwvWghost
```

[View on Solana Explorer](https://explorer.solana.com/address/DK9MPoDFDPyCZG16ESkY6mUPrpKJaeGVpmEjwvWghost)

## Logo

![BLACK GHOST](https://raw.githubusercontent.com/thablackghost/ghost/main/metadata/logo/ghost-logo.png)

## Build

```bash
anchor build
```

## Deploy

```bash
anchor deploy --provider.cluster mainnet
```

## Authority Model

| Authority | Permissions |
|-----------|------------|
| **Admin** | Mint, pause, update config, rotate authorities |
| **Metadata** | Update token metadata (name, symbol, URI) |
| **Emergency** | Emergency pause/resume |

- 24-hour timelock for authority rotations
- Metadata can be frozen permanently
- No freeze authority on token accounts

## Security

- Authority-gated minting
- Timelocked authority rotation
- Emergency pause system
- Checked arithmetic (no overflow)
- PDA enforcement for all state
- Supply invariants on every mint/burn

## License

MIT