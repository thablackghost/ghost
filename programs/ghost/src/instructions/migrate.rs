use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct MigrateAccounts<'info> {
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
        seeds = [VERSION_SEED],
        bump = version_state.bump,
    )]
    pub version_state: Account<'info, VersionState>,
}

#[event]
pub struct MigrationCompleted {
    pub global_config: Pubkey,
    pub admin: Pubkey,
    pub previous_version: u8,
    pub new_version: u8,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<MigrateAccounts>, target_version: u8) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Verify admin authority
    require!(
        accounts.admin.key() == accounts.global_config.admin_authority,
        GhostError::Unauthorized
    );

    let version_state = &mut accounts.version_state;
    let current = version_state.current_version;

    // Validate target version > current
    require!(target_version > current, GhostError::InvalidConfiguration);

    let timestamp = clock.unix_timestamp;

    version_state.previous_version = current;
    version_state.current_version = target_version;
    version_state.last_migration_timestamp = timestamp;

    // Update global config version as well
    accounts.global_config.version = target_version;

    emit!(MigrationCompleted {
        global_config: accounts.global_config.key(),
        admin: accounts.admin.key(),
        previous_version: current,
        new_version: target_version,
        timestamp,
    });

    Ok(())
}
