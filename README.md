# GHOST Token (GHOST)

A production-grade SPL Token-2022 project on Solana. Authority-controlled minting,
standard transfers, burn support, and comprehensive security features including
timelock-based authority rotation and emergency pause.

## Token Parameters

| Parameter       | Value                                                              |
|-----------------|--------------------------------------------------------------------|
| **Name**        | GHOST                                                              |
| **Symbol**      | GHOST                                                              |
| **Decimals**    | 9                                                                  |
| Initial Supply  | 10,000,000,000 (10 billion) GHOST                                  |
| Raw Supply      | 10,000,000,000,000,000,000 raw units (10B × 10⁹)                   |
| Max Supply      | **Unlimited** (minting controlled exclusively by designated authorities) |
| Token Program   | SPL Token-2022                                                     |
| Transfer Tax    | **None** — standard 1:1 transfers only                             |
| Burn            | Supported                                                          |

## Architecture

```
programs/ghost/src/
├── lib.rs              # Entry point, #[program] module
├── constants.rs        # Token constants, PDA seeds, limits
├── config.rs           # GlobalConfig state and validation
├── errors.rs           # Centralized error enum
├── events.rs           # Structured event emissions
├── version.rs          # Version tracking and compatibility
├── instructions/       # Instruction handler implementations
├── state/             # Program state structs (PDAs)
├── accounts/          # Anchor #[derive(Accounts)] structs
├── authorities/       # Authority management logic
├── token/             # Token operation helpers
├── metadata/          # On-chain metadata management
├── security/          # Pause, emergency-pause, invariants
├── math/              # Checked arithmetic helpers
├── validation/        # Input validation utilities
├── invariants/        # On-chain invariant assertions
└── utils/             # General utilities
```

## Quick Start

### Prerequisites

- Rust 1.80.0 (see `rust-toolchain.toml`)
- Solana CLI 1.18+
- Anchor CLI 0.30.1
- Node.js 18+ (for tests)

### Build

```bash
# Build the program
anchor build

# Build with verifiable flag (for mainnet)
anchor build --verifiable
```

### Test

```bash
# Run all tests
anchor test

# Run a specific test file
anchor test --skip-build tests/ghost.test.ts
```

### Deploy

```bash
# Devnet
anchor deploy --provider.cluster devnet

# Mainnet (requires confirmation)
anchor deploy --provider.cluster mainnet
```

## Authority Model

| Authority          | Permissions                                                     |
|--------------------|-----------------------------------------------------------------|
| **Admin**          | Mint tokens, update config, pause program, initiate rotations   |
| **Metadata**       | Update on-chain token metadata (name, symbol, URI)             |
| **Emergency**      | Emergency-pause and emergency-resume the program               |

- Authority rotations require a **24-hour timelock** before they can be accepted.
- The emergency authority is intentionally separated from the admin to allow
  a distinct key to respond to security incidents.
- Metadata can be permanently **frozen** to guarantee immutability.

## Security Features

- **Authority-gated minting** — Only the admin authority can mint new tokens.
- **Timelocked authority rotation** — 24-hour delay prevents hostile takeover.
- **Dual pause system** — Standard pause (admin) + emergency pause (emergency authority).
- **Metadata freeze** — Permanent immutability option for token metadata.
- **Checked arithmetic** — All math uses `checked_*` operations to prevent overflow.
- **PDA enforcement** — All state accounts are derived via PDAs with canonical seeds.
- **Comprehensive validation** — Every input is validated before state mutation.
- **Invariant assertions** — Supply and state invariants are checked on every operation.
- **No hidden taxes** — Standard SPL Token-2022 transfers with no transfer fees or hooks.

## License

MIT — see [LICENSE](./LICENSE) for details.