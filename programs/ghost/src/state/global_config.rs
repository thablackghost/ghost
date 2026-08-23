//! GHOST Token — Global configuration account.

use crate::errors::GhostError;
use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct GlobalConfig {
    pub version: u8,
    pub bump: u8,
    pub initialized: bool,
    pub paused: bool,
    pub emergency_paused: bool,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub admin_authority: Pubkey,
    pub metadata_authority: Pubkey,
    pub emergency_authority: Pubkey,
    pub update_authority: Pubkey,
    pub metadata_uri: String,
    pub metadata_frozen: bool,
    pub initialization_slot: u64,
    pub initialization_timestamp: i64,
    pub total_supply: u64,
    pub total_minted: u64,
    pub total_burned: u64,
    pub metadata_state: Pubkey,
    pub pause_timestamp: i64,
    pub emergency_pause_timestamp: i64,
    pub _reserved: [u8; 64],
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            version: 0,
            bump: 0,
            initialized: false,
            paused: false,
            emergency_paused: false,
            name: String::new(),
            symbol: String::new(),
            decimals: 0,
            mint: Pubkey::default(),
            mint_authority: Pubkey::default(),
            admin_authority: Pubkey::default(),
            metadata_authority: Pubkey::default(),
            emergency_authority: Pubkey::default(),
            update_authority: Pubkey::default(),
            metadata_uri: String::new(),
            metadata_frozen: false,
            initialization_slot: 0,
            initialization_timestamp: 0,
            total_supply: 0,
            total_minted: 0,
            total_burned: 0,
            metadata_state: Pubkey::default(),
            pause_timestamp: 0,
            emergency_pause_timestamp: 0,
            _reserved: [0u8; 64],
        }
    }
}

impl GlobalConfig {
    pub const LEN: usize = 8
        + 1
        + 1
        + 1
        + 1
        + 1
        + (4 + 32)
        + (4 + 10)
        + 1
        + 32
        + 32
        + 32
        + 32
        + 32
        + 32
        + (4 + 200)
        + 1
        + 8
        + 8
        + 8
        + 8
        + 8
        + 32
        + 8
        + 8
        + 64;

    pub const INIT_SPACE: usize = Self::LEN;

    pub fn check_not_paused(&self) -> Result<()> {
        require!(!self.emergency_paused, GhostError::EmergencyPaused);
        require!(!self.paused, GhostError::ProgramPaused);
        Ok(())
    }
}
