use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

const AUTHORITY_TYPE_MINT: u8 = 0;
const AUTHORITY_TYPE_ADMIN: u8 = 1;
const AUTHORITY_TYPE_METADATA: u8 = 2;
const AUTHORITY_TYPE_EMERGENCY: u8 = 3;

#[derive(Accounts)]
pub struct UpdateAuthorityAccounts<'info> {
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
}

#[event]
pub struct AuthorityUpdated {
    pub global_config: Pubkey,
    pub authority_type: u8,
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
    pub admin: Pubkey,
    pub timestamp: i64,
}

pub fn handler(
    ctx: Context<UpdateAuthorityAccounts>,
    authority_type: u8,
    new_authority: Pubkey,
) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Verify admin authority
    require!(
        accounts.admin.key() == accounts.global_config.admin_authority,
        GhostError::Unauthorized
    );

    // Validate new authority is non-zero
    require!(
        new_authority != Pubkey::default(),
        GhostError::InvalidAuthority
    );

    let config = &mut accounts.global_config;
    let authority_state = &mut accounts.authority_state;
    let old_authority: Pubkey;

    // Match authority type and update config
    match authority_type {
        AUTHORITY_TYPE_MINT => {
            old_authority = config.mint_authority;
            config.mint_authority = new_authority;
            authority_state.mint_authority = new_authority;
        }
        AUTHORITY_TYPE_ADMIN => {
            old_authority = config.admin_authority;
            config.admin_authority = new_authority;
            authority_state.admin_authority = new_authority;
        }
        AUTHORITY_TYPE_METADATA => {
            old_authority = config.metadata_authority;
            config.metadata_authority = new_authority;
            authority_state.metadata_authority = new_authority;
        }
        AUTHORITY_TYPE_EMERGENCY => {
            old_authority = config.emergency_authority;
            config.emergency_authority = new_authority;
            authority_state.emergency_authority = new_authority;
        }
        _ => return Err(GhostError::InvalidConfiguration.into()),
    }

    let timestamp = clock.unix_timestamp;

    emit!(AuthorityUpdated {
        global_config: accounts.global_config.key(),
        authority_type,
        old_authority,
        new_authority,
        admin: accounts.admin.key(),
        timestamp,
    });

    Ok(())
}
