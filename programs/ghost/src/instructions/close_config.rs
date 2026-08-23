use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct CloseConfigAccounts<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
        close = admin,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct ConfigurationClosed {
    pub global_config: Pubkey,
    pub admin: Pubkey,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<CloseConfigAccounts>) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Verify admin authority
    require!(
        accounts.admin.key() == accounts.global_config.admin_authority,
        GhostError::Unauthorized
    );

    let timestamp = clock.unix_timestamp;

    emit!(ConfigurationClosed {
        global_config: accounts.global_config.key(),
        admin: accounts.admin.key(),
        timestamp,
    });

    // The `close = admin` constraint on the account automatically
    // transfers lamports to admin and sets the account discriminator
    // to CLOSED_ACCOUNT_DISCRIMINATOR.

    Ok(())
}
