use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct EmergencyPauseAccounts<'info> {
    #[account(mut)]
    pub emergency_authority: Signer<'info>,

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
pub struct EmergencyPaused {
    pub global_config: Pubkey,
    pub emergency_authority: Pubkey,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<EmergencyPauseAccounts>) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Verify emergency authority
    require!(
        accounts.emergency_authority.key() == accounts.global_config.emergency_authority,
        GhostError::Unauthorized
    );

    let config = &mut accounts.global_config;

    let timestamp = clock.unix_timestamp;

    config.emergency_paused = true;
    config.emergency_pause_timestamp = timestamp;

    let pause_state = &mut accounts.pause_state;
    pause_state.emergency_paused = true;
    pause_state.emergency_pause_timestamp = timestamp;

    emit!(EmergencyPaused {
        global_config: accounts.global_config.key(),
        emergency_authority: accounts.emergency_authority.key(),
        timestamp,
    });

    Ok(())
}
