use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

const FIELD_VERSION: u8 = 0;
const FIELD_DECIMALS: u8 = 1;
const FIELD_NAME: u8 = 2;
const FIELD_SYMBOL: u8 = 3;
const FIELD_URI: u8 = 4;
const FIELD_PAUSED: u8 = 5;
const FIELD_PROPOSAL_DELAY: u8 = 6;
const FIELD_METADATA_FROZEN: u8 = 7;

#[derive(Accounts)]
pub struct ConfigureAccounts<'info> {
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
        seeds = [METADATA_SEED],
        bump = metadata_state.bump,
    )]
    pub metadata_state: Account<'info, MetadataState>,

    #[account(
        mut,
        seeds = [AUTHORITY_SEED],
        bump = authority_state.bump,
    )]
    pub authority_state: Account<'info, AuthorityState>,
}

#[event]
pub struct FieldConfigurationUpdated {
    pub global_config: Pubkey,
    pub admin: Pubkey,
    pub field: u8,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<ConfigureAccounts>, field: u8, value: Vec<u8>) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Verify admin authority
    require!(
        accounts.admin.key() == accounts.global_config.admin_authority,
        GhostError::Unauthorized
    );

    let config = &mut accounts.global_config;
    let metadata = &mut accounts.metadata_state;
    let authority_state = &mut accounts.authority_state;

    match field {
        FIELD_VERSION => {
            require!(value.len() == 1, GhostError::InvalidConfiguration);
            config.version = value[0];
        }
        FIELD_DECIMALS => {
            require!(value.len() == 1, GhostError::InvalidConfiguration);
            let new_decimals = value[0];
            require!(new_decimals <= 18, GhostError::InvalidDecimals);
            config.decimals = new_decimals;
        }
        FIELD_NAME => {
            let name = String::from_utf8(value).map_err(|_| GhostError::InvalidConfiguration)?;
            require!(
                name.len() <= METADATA_MAX_NAME_LEN as usize,
                GhostError::MetadataTooLong
            );
            config.name = name;
        }
        FIELD_SYMBOL => {
            let symbol = String::from_utf8(value).map_err(|_| GhostError::InvalidConfiguration)?;
            require!(
                symbol.len() <= METADATA_MAX_SYMBOL_LEN as usize,
                GhostError::MetadataTooLong
            );
            config.symbol = symbol;
        }
        FIELD_URI => {
            let uri = String::from_utf8(value).map_err(|_| GhostError::InvalidConfiguration)?;
            require!(
                uri.len() <= METADATA_MAX_URI_LEN as usize,
                GhostError::MetadataTooLong
            );
            metadata.uri = uri;
        }
        FIELD_PAUSED => {
            require!(value.len() == 1, GhostError::InvalidConfiguration);
            config.paused = value[0] != 0;
        }
        FIELD_PROPOSAL_DELAY => {
            require!(value.len() == 8, GhostError::InvalidConfiguration);
            let delay_bytes: [u8; 8] = value
                .try_into()
                .map_err(|_| GhostError::InvalidConfiguration)?;
            let delay = i64::from_le_bytes(delay_bytes);
            authority_state.proposed_delay = delay;
        }
        FIELD_METADATA_FROZEN => {
            require!(value.len() == 1, GhostError::InvalidConfiguration);
            metadata.frozen = value[0] != 0;
        }
        _ => return Err(GhostError::InvalidConfiguration.into()),
    }

    let timestamp = clock.unix_timestamp;

    emit!(FieldConfigurationUpdated {
        global_config: accounts.global_config.key(),
        admin: accounts.admin.key(),
        field,
        timestamp,
    });

    Ok(())
}
