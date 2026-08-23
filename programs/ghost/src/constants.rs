//! GHOST Token — Core constants and configuration parameters.

use solana_program::pubkey;
use solana_program::pubkey::Pubkey;

// ── Token Identity ────────────────────────────────────────────────────────────

pub const TOKEN_NAME: &str = "BLACK GHOST";
pub const TOKEN_SYMBOL: &str = "GHOST";
pub const TOKEN_DECIMALS: u8 = 9;

// ── Supply ────────────────────────────────────────────────────────────────────

/// Initial supply in human-readable units (1 billion GHOST).
pub const INITIAL_SUPPLY_HUMAN: u64 = 1_000_000_000;

/// Initial supply in raw atomic units (1 billion × 10^9).
pub const INITIAL_SUPPLY_RAW: u64 = 1_000_000_000_000_000_000;

/// There is no maximum supply cap.
pub const MAX_SUPPLY: u64 = u64::MAX;

// ── Metadata Bounds ───────────────────────────────────────────────────────────

pub const METADATA_MAX_NAME_LEN: usize = 32;
pub const METADATA_MAX_SYMBOL_LEN: usize = 10;
pub const METADATA_MAX_URI_LEN: usize = 200;

// ── Authority Rotation ────────────────────────────────────────────────────────

pub const AUTHORITY_ROTATION_DELAY_SECS: i64 = 86_400; // 24 hours

// ── Batch / Rate Limits ───────────────────────────────────────────────────────

pub const MAX_BATCH_SIZE: usize = 20;

// ── Program Version ───────────────────────────────────────────────────────────

pub const PROGRAM_VERSION: u8 = 1;

// ── PDA Seeds ─────────────────────────────────────────────────────────────────

pub const CONFIG_SEED: &[u8] = b"ghost_config";
pub const AUTHORITY_SEED: &[u8] = b"ghost_authority";
pub const METADATA_SEED: &[u8] = b"ghost_metadata";
pub const PAUSE_SEED: &[u8] = b"ghost_pause";
pub const VERSION_SEED: &[u8] = b"ghost_version";
pub const MIGRATION_SEED: &[u8] = b"ghost_migration";
pub const MINT_CONFIG_SEED: &[u8] = b"ghost_mint_config";

// ── Well-Known Pubkeys ────────────────────────────────────────────────────────

pub const TOKEN_2022_PROGRAM_ID: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
pub const TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
