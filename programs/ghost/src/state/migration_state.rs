//! GHOST Token — Migration state account.

use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct MigrationState {
    pub from_version: u8,
    pub to_version: u8,
    pub migrated: bool,
    pub migration_timestamp: i64,
    pub migration_authority: Pubkey,
    pub bump: u8,
    pub _reserved: [u8; 95],
}

impl Default for MigrationState {
    fn default() -> Self {
        Self {
            from_version: 0,
            to_version: 0,
            migrated: false,
            migration_timestamp: 0,
            migration_authority: Pubkey::default(),
            bump: 0,
            _reserved: [0u8; 95],
        }
    }
}

impl MigrationState {
    pub const LEN: usize = 8 + 1 + 1 + 1 + 8 + 32 + 1 + 95;
    pub const INIT_SPACE: usize = Self::LEN;
}
