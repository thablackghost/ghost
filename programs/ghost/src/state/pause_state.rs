//! GHOST Token — Pause state account.

use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct PauseState {
    pub paused: bool,
    pub emergency_paused: bool,
    pub pause_authority: Pubkey,
    pub emergency_authority: Pubkey,
    pub pause_timestamp: i64,
    pub emergency_pause_timestamp: i64,
    pub bump: u8,
    pub _reserved: [u8; 47],
}

impl Default for PauseState {
    fn default() -> Self {
        Self {
            paused: false,
            emergency_paused: false,
            pause_authority: Pubkey::default(),
            emergency_authority: Pubkey::default(),
            pause_timestamp: 0,
            emergency_pause_timestamp: 0,
            bump: 0,
            _reserved: [0u8; 47],
        }
    }
}

impl PauseState {
    pub const LEN: usize = 8 + 1 + 1 + 32 + 32 + 8 + 8 + 1 + 47;
    pub const INIT_SPACE: usize = Self::LEN;
}
