//! GHOST Token — Metadata state account.

use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct MetadataState {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub description: String,
    pub update_authority: Pubkey,
    pub frozen: bool,
    pub initialized: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
    pub _reserved: [u8; 47],
}

impl Default for MetadataState {
    fn default() -> Self {
        Self {
            name: String::new(),
            symbol: String::new(),
            uri: String::new(),
            description: String::new(),
            update_authority: Pubkey::default(),
            frozen: false,
            initialized: false,
            created_at: 0,
            updated_at: 0,
            bump: 0,
            _reserved: [0u8; 47],
        }
    }
}

impl MetadataState {
    pub const LEN: usize =
        8 + (4 + 32) + (4 + 10) + (4 + 200) + (4 + 256) + 32 + 1 + 1 + 8 + 8 + 1 + 47;
    pub const INIT_SPACE: usize = Self::LEN;
}
