use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct UnpauseAccounts<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

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
pub struct ProgramResumed {
    pub global_config: Pubkey,
    pub admin: Pubkey,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<UnpauseAccounts>) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Verify admin authority
    require!(
        accounts.admin.key() == accounts.global_config.admin_authority,
        GhostError::Unauthorized
    );

    let config = &mut accounts.global_config;

    // Check that program is actually paused
    require!(config.paused, GhostError::InvalidState);

    let timestamp = clock.unix_timestamp;

    config.paused = false;
    config.pause_timestamp = 0;

    let pause_state = &mut accounts.pause_state;
    pause_state.paused = false;
    pause_state.pause_timestamp = 0;

    emit!(ProgramResumed {
        global_config: accounts.global_config.key(),
        admin: accounts.admin.key(),
        timestamp,
    });

    Ok(())
}
