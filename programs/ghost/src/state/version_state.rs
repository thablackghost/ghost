//! GHOST Token — Version state account.

use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct VersionState {
    pub current_version: u8,
    pub previous_version: u8,
    pub min_compatible_version: u8,
    pub upgrade_authority: Pubkey,
    pub last_migration_timestamp: i64,
    pub bump: u8,
    pub _reserved: [u8; 63],
}

impl Default for VersionState {
    fn default() -> Self {
        Self {
            current_version: 0,
            previous_version: 0,
            min_compatible_version: 0,
            upgrade_authority: Pubkey::default(),
            last_migration_timestamp: 0,
            bump: 0,
            _reserved: [0u8; 63],
        }
    }
}

impl VersionState {
    pub const LEN: usize = 8 + 1 + 1 + 1 + 32 + 8 + 1 + 63;
    pub const INIT_SPACE: usize = Self::LEN;
}
