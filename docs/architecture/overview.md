# Architecture Overview

## Project Structure

```
ghost-token/
├── programs/
│   └── ghost-token/           # Solana program (on-chain)
│       ├── src/
│       │   ├── lib.rs         # Program entry point & instruction dispatch
│       │   ├── state.rs       # On-chain state structures (Config)
│       │   ├── instructions/  # Instruction handlers
│       │   │   ├── mod.rs
│       │   │   ├── initialize.rs
│       │   │   ├── mint.rs
│       │   │   ├── burn.rs
│       │   │   ├── update_authority.rs
│       │   │   ├── pause.rs
│       │   │   └── resume.rs
│       │   ├── errors.rs      # Custom error types
│       │   ├── constants.rs   # Magic numbers, seeds
│       │   └── utils.rs       # Validation helpers
│       └── Cargo.toml
├── sdk/
│   ├── rust/                  # Rust SDK for program interaction
│   └── typescript/            # TypeScript SDK (npm package)
├── tests/                     # Integration & security tests
├── metadata/                  # Token metadata JSON files
├── migrations/                # Migration guides & version history
├── audits/                    # Security checklists & audit reports
├── docs/                      # Comprehensive documentation
├── devnet/                    # Devnet deployment configs & fixtures
└── .github/                   # CI/CD & repo configuration
```

## Module Responsibilities

### On-Chain Program (`programs/ghost-token/`)

| Module | Responsibility |
|--------|---------------|
| `lib.rs` | Program entry point. Routes incoming instructions to the appropriate handler via `process_instruction`. |
| `state.rs` | Defines the `Config` account — stores mint address, admin authority, emergency pause flag, and authority rotation state. |
| `instructions/initialize.rs` | Creates the SPL Token-2022 mint with 9 decimals, mints initial 10B supply. Sets up the on-chain Config PDA. |
| `instructions/mint.rs` | Allows the designated mint authority to mint additional tokens on demand. No supply cap. |
| `instructions/burn.rs` | Allows any token holder to burn their own tokens, reducing total supply. |
| `instructions/update_authority.rs` | Initiates or completes a two-step authority rotation with timelock. |
| `instructions/pause.rs` | Sets the emergency pause flag, halting all mint and authority rotation operations. |
| `instructions/resume.rs` | Clears the emergency pause flag, restoring normal operations. |
| `errors.rs` | Custom error codes for all failure modes. |
| `constants.rs` | PDA seeds (`b"config"`), program-derived address constants. |
| `utils.rs` | Shared validation functions (signer checks, PDA verification, owner validation). |

### SDKs

| SDK | Purpose |
|-----|---------|
| Rust SDK (`sdk/rust/`) | Typed client for Rust applications. Provides instruction builders, account deserialization, and CPI helpers. |
| TypeScript SDK (`sdk/typescript/`) | Typed client for browser/Node.js. Provides IDL types, instruction builders, and utility functions. |

## Data Flow

```
Client (dApp / CLI)
    │
    ▼
SDK (Rust or TypeScript)
    │  Constructs instruction + accounts
    ▼
Solana Transaction
    │
    ▼
On-Chain Program (ghost-token)
    │
    ├──► SPL Token-2022 Program (CPI)
    │       Mint tokens, burn tokens
    │
    ├──► Config PDA (read/write)
    │       Authority validation, pause state
    │
    └──► Returns success/error
```

### Instruction Flow (Mint Example)

1. **Client** calls `mintTokens(amount)` via SDK
2. **SDK** constructs `MintTokens` instruction with accounts:
   - Mint account (Token-2022)
   - Token account (destination)
   - Config PDA
   - Mint authority (signer)
   - Token-2022 program
3. **Transaction** submitted to cluster
4. **Program** processes instruction:
   - Deserializes instruction data
   - Loads Config PDA → checks pause state
   - Validates signer is current mint authority
   - Invokes CPI to Token-2022: `mint_to`
   - Returns success

## Authority Model

```
                    ┌─────────────────────┐
                    │   Upgrade Authority  │
                    │  (off-chain keypair) │
                    └──────────┬──────────┘
                               │ controls program upgrades
                               ▼
┌──────────────────────────────────────────────────────────────┐
│                      On-Chain Config PDA                      │
│                                                               │
│   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│   │ Mint         │  │ Admin        │  │ Emergency    │       │
│   │ Authority    │  │ Authority    │  │ Authority    │       │
│   └──────┬───────┘  └──────┬───────┘  └──────┬───────┘       │
│          │                 │                  │               │
└──────────┼─────────────────┼──────────────────┼───────────────┘
           │                 │                  │
           ▼                 ▼                  ▼
    Mint new tokens    Rotate authorities    Pause / Resume
    (unlimited)        (two-step + lock)     operations
```

### Separation of Concerns

| Authority | Scope | Risk if Compromised |
|-----------|-------|---------------------|
| **Mint Authority** | Can mint new tokens (no cap) | Infinite token inflation |
| **Admin Authority** | Can rotate mint/emergency authorities | Indirect control via rotation |
| **Emergency Authority** | Can pause all minting and rotation | Temporary DOS (recoverable) |
| **Upgrade Authority** | Can deploy new program code | Full program takeover |
| **Metadata Authority** | Can update token URI/name/symbol | Misleading metadata |

## Token Lifecycle

```
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  Deploy  │───►│Initialize│───►│  Active  │───►│  Frozen  │
│ Program  │    │  Token   │    │  State   │    │ (paused) │
└──────────┘    └──────────┘    └──────────┘    └──────────┘
                                     │                │
                                     │                │
                                     ▼                ▼
                              ┌──────────┐    ┌──────────┐
                              │   Mint   │    │  Resume  │
                              │   Burn   │    │ (unpause)│
                              │  Rotate  │    └──────────┘
                              └──────────┘

1. DEPLOY: Program binary deployed to Solana via `solana program deploy`
2. INITIALIZE: Config PDA created, Token-2022 mint created, 10B tokens minted to specified account
3. ACTIVE: Normal operations — minting, burning, authority rotation
4. FROZEN: Emergency pause active — minting and rotation blocked; burning still allowed
```

## Key Design Decisions

1. **Token-2022 over Original SPL Token**: Enables metadata extensions, transfer hooks (future), and confidential transfers (future).
2. **Unlimited Minting**: By design. GHOST is an inflationary meme token. Trust is placed in the mint authority.
3. **Two-Step Authority Rotation**: Prevents accidental authority loss. Requires both initiate + confirm steps with a timelock.
4. **PDA for Config**: Derives from program ID, ensuring only this program can modify its config.
5. **No Supply Cap**: Allows flexible tokenomics. The mint authority is the sole gate.
6. **Emergency Pause**: Defense-in-depth. Stops minting if authority is compromised, giving time for recovery.