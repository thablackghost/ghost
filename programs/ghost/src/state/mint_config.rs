//! GHOST Token — Mint configuration account.

use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct MintConfig {
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub token_program: Pubkey,
    pub decimals: u8,
    pub last_mint_slot: u64,
    pub last_mint_timestamp: i64,
    pub total_minted: u64,
    pub total_burned: u64,
    pub mint_count: u64,
    pub burn_count: u64,
    pub bump: u8,
    pub _reserved: [u8; 47],
}

impl Default for MintConfig {
    fn default() -> Self {
        Self {
            mint: Pubkey::default(),
            mint_authority: Pubkey::default(),
            token_program: Pubkey::default(),
            decimals: 0,
            last_mint_slot: 0,
            last_mint_timestamp: 0,
            total_minted: 0,
            total_burned: 0,
            mint_count: 0,
            burn_count: 0,
            bump: 0,
            _reserved: [0u8; 47],
        }
    }
}

impl MintConfig {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 47;
    pub const INIT_SPACE: usize = Self::LEN;
}
