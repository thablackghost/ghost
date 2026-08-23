use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateMetadataAccounts<'info> {
    #[account(mut)]
    pub metadata_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [METADATA_SEED],
        bump = metadata_state.bump,
    )]
    pub metadata_state: Account<'info, MetadataState>,

    #[account(
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,
}

#[event]
pub struct MetadataUpdated {
    pub metadata_state: Pubkey,
    pub old_name: String,
    pub new_name: String,
    pub old_symbol: String,
    pub new_symbol: String,
    pub old_uri: String,
    pub new_uri: String,
    pub update_authority: Pubkey,
    pub timestamp: i64,
}

pub fn handler(
    ctx: Context<UpdateMetadataAccounts>,
    new_name: String,
    new_symbol: String,
    new_uri: String,
) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Verify metadata authority
    require!(
        accounts.metadata_authority.key() == accounts.global_config.metadata_authority,
        GhostError::Unauthorized
    );

    // Check metadata is not frozen
    require!(!accounts.metadata_state.frozen, GhostError::MetadataFrozen);

    // Validate lengths against MAX constants
    require!(
        new_name.len() <= METADATA_MAX_NAME_LEN as usize,
        GhostError::MetadataTooLong
    );
    require!(
        new_symbol.len() <= METADATA_MAX_SYMBOL_LEN as usize,
        GhostError::MetadataTooLong
    );
    require!(
        new_uri.len() <= METADATA_MAX_URI_LEN as usize,
        GhostError::MetadataTooLong
    );

    let metadata = &mut accounts.metadata_state;
    let old_name = metadata.name.clone();
    let old_symbol = metadata.symbol.clone();
    let old_uri = metadata.uri.clone();

    metadata.name = new_name.clone();
    metadata.symbol = new_symbol.clone();
    metadata.uri = new_uri.clone();

    let timestamp = clock.unix_timestamp;

    emit!(MetadataUpdated {
        metadata_state: accounts.metadata_state.key(),
        old_name,
        new_name,
        old_symbol,
        new_symbol,
        old_uri,
        new_uri,
        update_authority: accounts.metadata_authority.key(),
        timestamp,
    });

    Ok(())
}
