# Program Design Document

## Program ID

```
Gh0st111111111111111111111111111111111111111
```

This is a vanity program ID containing the prefix "Gh0st" followed by ones. The address was generated using a GPU-accelerated vanity key generator.

## Instruction List

| # | Instruction | Description | Required Signers |
|---|------------|-------------|------------------|
| 1 | `Initialize` | Creates Token-2022 mint (9 decimals), mints 10B tokens, creates Config PDA | Admin, Funding Account |
| 2 | `MintTokens` | Mints additional tokens to a specified token account | Mint Authority |
| 3 | `BurnTokens` | Burns tokens from the caller's token account | Token Owner |
| 4 | `InitiateAuthorityRotation` | Starts two-step authority rotation with timelock | Admin Authority |
| 5 | `ConfirmAuthorityRotation` | Completes pending authority rotation after timelock | New Authority (optional) |
| 6 | `CancelAuthorityRotation` | Cancels a pending authority rotation | Admin Authority |
| 7 | `Pause` | Sets emergency pause flag, halts minting and rotation | Emergency Authority |
| 8 | `Resume` | Clears emergency pause flag | Emergency Authority |

## Instruction Data Structures

### Initialize

```rust
pub struct Initialize {
    /// Initial token supply (10_000_000_000 * 10^9 raw units)
    pub initial_supply: u64,
}
```

**Accounts:**
- `[writable, signer]` admin: The admin authority
- `[writable]` mint: Token-2022 mint account (created via CPI)
- `[writable]` token_account: Destination for initial supply
- `[writable]` config: Config PDA
- `[]` token_program: SPL Token-2022 program ID
- `[]` system_program: System program
- `[]` rent: Rent sysvar

### MintTokens

```rust
pub struct MintTokens {
    pub amount: u64,
}
```

**Accounts:**
- `[writable]` mint: The Token-2022 mint
- `[writable]` token_account: Destination token account
- `[signer]` authority: Must match config.mint_authority
- `[]` config: Config PDA

### BurnTokens

```rust
pub struct BurnTokens {
    pub amount: u64,
}
```

**Accounts:**
- `[writable]` mint: The Token-2022 mint
- `[writable]` token_account: Source token account (tokens to burn)
- `[signer]` owner: Token account owner
- `[]` config: Config PDA (for pause check)

### InitiateAuthorityRotation

```rust
pub struct InitiateAuthorityRotation {
    /// Which authority to rotate (0=mint, 1=emergency, 2=admin)
    pub authority_type: u8,
    /// New authority public key
    pub new_authority: Pubkey,
}
```

**Accounts:**
- `[signer]` admin: Current admin authority
- `[writable]` config: Config PDA
- `[]` clock: Clock sysvar (for timelock)

### ConfirmAuthorityRotation

```rust
pub struct ConfirmAuthorityRotation {}
```

**Accounts:**
- `[writable]` config: Config PDA
- `[]` clock: Clock sysvar

### CancelAuthorityRotation

```rust
pub struct CancelAuthorityRotation {}

```

**Accounts:**
- `[signer]` admin: Current admin authority
- `[writable]` config: Config PDA

### Pause

```rust
pub struct Pause {}
```

**Accounts:**
- `[signer]` emergency_authority: Must match config.emergency_authority
- `[writable]` config: Config PDA

### Resume

```rust
pub struct Resume {}
```

**Accounts:**
- `[signer]` emergency_authority: Must match config.emergency_authority
- `[writable]` config: Config PDA

## Account Structures

### Config

```rust
/// On-chain configuration account stored at a PDA
pub struct Config {
    /// Bump seed for PDA derivation
    pub bump: u8,
    /// Program version (for future upgrades)
    pub version: u8,
    /// Emergency pause flag
    pub paused: bool,
    /// Padding for alignment
    pub _padding: [u8; 5],
    /// The Token-2022 mint address
    pub mint: Pubkey,
    /// Current mint authority (can mint tokens)
    pub mint_authority: Pubkey,
    /// Admin authority (can rotate other authorities)
    pub admin_authority: Pubkey,
    /// Emergency authority (can pause/resume)
    pub emergency_authority: Pubkey,
    /// Pending authority rotation state
    pub pending_rotation: Option<PendingRotation>,
}

pub struct PendingRotation {
    /// Type of authority being rotated
    pub authority_type: u8,
    /// Proposed new authority
    pub new_authority: Pubkey,
    /// Timestamp when rotation can be confirmed
    pub effective_at: i64,
}
```

**Space Calculation:**
- bump: 1 byte
- version: 1 byte
- paused: 1 byte
- padding: 5 bytes
- mint: 32 bytes
- mint_authority: 32 bytes
- admin_authority: 32 bytes
- emergency_authority: 32 bytes
- pending_rotation (Option): 1 + (1 + 32 + 8) = 42 bytes
- **Total: ~178 bytes** (rounded up to include account discriminator)

## PDA Derivation

### Config PDA

```rust
let (config_pda, config_bump) = Pubkey::find_program_address(
    &[b"config"],
    &program_id,
);
```

- **Seeds**: `[b"config"]`
- **Derived from**: Program ID
- **Guarantee**: Only this program can sign for the Config PDA

### Token Mint PDA (for CPI signing)

When the program itself needs to act as mint authority for CPI calls:

```rust
let (mint_authority_pda, bump) = Pubkey::find_program_address(
    &[b"mint-authority"],
    &program_id,
);
```

## CPI Interactions

The program performs Cross-Program Invocation (CPI) to SPL Token-2022:

### Mint CPI
```
Program Account → SPL Token-2022 → mint_to(...)
                              → Destination: token_account
                              → Amount: instruction_data.amount
```

### Burn CPI
```
Program Account → SPL Token-2022 → burn(...)
                              → Source: token_account (delegated)
                              → Amount: instruction_data.amount
```

### Initialize CPI
```
Program Account → SPL Token-2022 → initialize_mint(...)
                              → Decimals: 9
                              → Mint Authority: mint_authority_pda

Program Account → SPL Token-2022 → mint_to(...)
                              → Amount: 10_000_000_000 * 10^9

Program Account → System Program → create_account(...)
                              → Config PDA created
```

## State Machine Transitions

```
                    ┌──────────┐
                    │ UNINIT   │  (config PDA does not exist)
                    └────┬─────┘
                         │ Initialize
                         ▼
                    ┌──────────┐
          ┌────────►│  ACTIVE  │◄─────────┐
          │         └────┬─────┘          │
          │              │                │
          │         Pause│           Resume
          │              ▼                │
          │         ┌──────────┐          │
          │         │  PAUSED  │──────────┘
          │         └──────────┘
          │
          │  Authority Rotation (in any non-paused state):
          │
          │  ┌──────────┐   Initiate   ┌──────────┐
          │  │ NO_PEND  │─────────────►│ PENDING  │
          │  └──────────┘              └────┬─────┘
          │       ▲                         │
          │       │      Confirm/Cancel     │
          │       └─────────────────────────┘
          │
          └── Burn always allowed (even when paused)
```

### Valid Transitions

| From | To | Condition |
|------|----|-----------|
| UNINIT | ACTIVE | Initialize instruction |
| ACTIVE | PAUSED | Pause by emergency authority |
| PAUSED | ACTIVE | Resume by emergency authority |
| ACTIVE | ACTIVE | Mint, Burn, Authority Rotations |
| PAUSED | PAUSED | Burn only |
| N/A | N/A | Initiate rotate when no pending rotation |
| PENDING | NO_PEND | Confirm after timelock, or Cancel |

### Blocked During Pause

- `MintTokens` — returns error `Paused`
- `InitiateAuthorityRotation` — returns error `Paused`
- `ConfirmAuthorityRotation` — returns error `Paused`
- `CancelAuthorityRotation` — allowed (admin can always cancel)
- `BurnTokens` — always allowed

## Version Compatibility

The `Config.version` field allows future program upgrades to identify the state format:

| Version | Description |
|---------|-------------|
| 0 | Initial release |
| 1+ | Reserved for future upgrades |

Upgrades must handle backward compatibility or include a migration instruction.