use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct EmergencyResumeAccounts<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [PAUSE_SEED],
        bump = pause_state.bump,
    )]
    pub pause_state: Account<'info, PauseState>,
}

#[event]
pub struct EmergencyResumed {
    pub global_config: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<EmergencyResumeAccounts>) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Verify signer is either admin or emergency authority
    let is_admin = accounts.authority.key() == accounts.global_config.admin_authority;
    let is_emergency = accounts.authority.key() == accounts.global_config.emergency_authority;
    require!(is_admin || is_emergency, GhostError::Unauthorized);

    let config = &mut accounts.global_config;

    // Check that emergency pause is actually active
    require!(config.emergency_paused, GhostError::InvalidState);

    let timestamp = clock.unix_timestamp;

    config.emergency_paused = false;
    config.emergency_pause_timestamp = 0;

    let pause_state = &mut accounts.pause_state;
    pause_state.emergency_paused = false;
    pause_state.emergency_pause_timestamp = 0;

    emit!(EmergencyResumed {
        global_config: accounts.global_config.key(),
        authority: accounts.authority.key(),
        timestamp,
    });

    Ok(())
}
