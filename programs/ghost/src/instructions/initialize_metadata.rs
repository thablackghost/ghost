use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeMetadataAccounts<'info> {
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
pub struct MetadataInitialized {
    pub metadata_state: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub update_authority: Pubkey,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<InitializeMetadataAccounts>) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Validate lengths against MAX constants
    require!(
        TOKEN_NAME.len() <= METADATA_MAX_NAME_LEN as usize,
        GhostError::MetadataTooLong
    );
    require!(
        TOKEN_SYMBOL.len() <= METADATA_MAX_SYMBOL_LEN as usize,
        GhostError::MetadataTooLong
    );

    let metadata = &mut accounts.metadata_state;

    metadata.name = TOKEN_NAME.to_string();
    metadata.symbol = TOKEN_SYMBOL.to_string();
    metadata.uri = String::new();
    metadata.update_authority = accounts.metadata_authority.key();
    metadata.frozen = false;

    let timestamp = clock.unix_timestamp;

    emit!(MetadataInitialized {
        metadata_state: accounts.metadata_state.key(),
        name: TOKEN_NAME.to_string(),
        symbol: TOKEN_SYMBOL.to_string(),
        uri: String::new(),
        update_authority: accounts.metadata_authority.key(),
        timestamp,
    });

    Ok(())
}
