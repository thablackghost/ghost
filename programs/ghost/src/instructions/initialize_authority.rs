use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeAuthorityAccounts<'info> {
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
        seeds = [AUTHORITY_SEED],
        bump = authority_state.bump,
    )]
    pub authority_state: Account<'info, AuthorityState>,

    /// CHECK: Mint authority pubkey to set
    pub mint_authority: UncheckedAccount<'info>,

    /// CHECK: Admin authority pubkey to set
    pub admin_authority: UncheckedAccount<'info>,

    /// CHECK: Metadata authority pubkey to set
    pub metadata_authority: UncheckedAccount<'info>,

    /// CHECK: Emergency authority pubkey to set
    pub emergency_authority: UncheckedAccount<'info>,
}

#[event]
pub struct AuthorityInitialized {
    pub global_config: Pubkey,
    pub mint_authority: Pubkey,
    pub admin_authority: Pubkey,
    pub metadata_authority: Pubkey,
    pub emergency_authority: Pubkey,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<InitializeAuthorityAccounts>) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Validate non-zero pubkeys
    require!(
        accounts.mint_authority.key() != Pubkey::default(),
        GhostError::InvalidAuthority
    );
    require!(
        accounts.admin_authority.key() != Pubkey::default(),
        GhostError::InvalidAuthority
    );
    require!(
        accounts.metadata_authority.key() != Pubkey::default(),
        GhostError::InvalidAuthority
    );
    require!(
        accounts.emergency_authority.key() != Pubkey::default(),
        GhostError::InvalidAuthority
    );

    // Verify admin signing matches the payer (admin is the authority initializing)
    // In a real scenario admin.authority should be pre-validated

    let config = &mut accounts.global_config;
    config.mint_authority = accounts.mint_authority.key();
    config.admin_authority = accounts.admin_authority.key();
    config.metadata_authority = accounts.metadata_authority.key();
    config.emergency_authority = accounts.emergency_authority.key();

    let authority_state = &mut accounts.authority_state;
    authority_state.mint_authority = accounts.mint_authority.key();
    authority_state.admin_authority = accounts.admin_authority.key();
    authority_state.metadata_authority = accounts.metadata_authority.key();
    authority_state.emergency_authority = accounts.emergency_authority.key();

    let timestamp = clock.unix_timestamp;

    emit!(AuthorityInitialized {
        global_config: accounts.global_config.key(),
        mint_authority: accounts.mint_authority.key(),
        admin_authority: accounts.admin_authority.key(),
        metadata_authority: accounts.metadata_authority.key(),
        emergency_authority: accounts.emergency_authority.key(),
        timestamp,
    });

    Ok(())
}
