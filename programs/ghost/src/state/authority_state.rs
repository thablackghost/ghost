//! GHOST Token — Authority state account for rotation management.

use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct AuthorityState {
    pub initialized: bool,
    pub current_authority: Pubkey,
    pub mint_authority: Pubkey,
    pub admin_authority: Pubkey,
    pub metadata_authority: Pubkey,
    pub emergency_authority: Pubkey,
    pub proposed_authority: Pubkey,
    pub proposed_authority_type: u8,
    pub proposal_timestamp: i64,
    pub proposed_at: i64,
    pub proposed_delay: i64,
    pub timelock_duration: i64,
    pub bump: u8,
    pub _reserved: [u8; 63],
}

impl Default for AuthorityState {
    fn default() -> Self {
        Self {
            initialized: false,
            current_authority: Pubkey::default(),
            mint_authority: Pubkey::default(),
            admin_authority: Pubkey::default(),
            metadata_authority: Pubkey::default(),
            emergency_authority: Pubkey::default(),
            proposed_authority: Pubkey::default(),
            proposed_authority_type: 0,
            proposal_timestamp: 0,
            proposed_at: 0,
            proposed_delay: 0,
            timelock_duration: 0,
            bump: 0,
            _reserved: [0u8; 63],
        }
    }
}

impl AuthorityState {
    pub const LEN: usize = 8 + 1 + 32 + 32 + 32 + 32 + 32 + 32 + 1 + 8 + 8 + 8 + 8 + 1 + 63;
    pub const INIT_SPACE: usize = Self::LEN;
}
