use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeConfigAccounts<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,
}

#[event]
pub struct ConfigurationUpdated {
    pub global_config: Pubkey,
    pub admin: Pubkey,
    pub initialization_slot: u64,
    pub initialization_timestamp: i64,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<InitializeConfigAccounts>) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    let config = &mut accounts.global_config;

    // Verify admin is the configured admin authority
    require!(
        config.admin_authority != Pubkey::default(),
        GhostError::NotInitialized
    );
    require!(
        accounts.admin.key() == config.admin_authority,
        GhostError::Unauthorized
    );

    let current_slot = clock.slot;
    let timestamp = clock.unix_timestamp;

    config.initialization_slot = current_slot;
    config.initialization_timestamp = timestamp;

    emit!(ConfigurationUpdated {
        global_config: accounts.global_config.key(),
        admin: accounts.admin.key(),
        initialization_slot: current_slot,
        initialization_timestamp: timestamp,
        timestamp,
    });

    Ok(())
}
