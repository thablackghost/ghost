use anchor_lang::prelude::*;

use super::constants::*;
use super::validation;
use crate::errors::GhostError;
use crate::state::MetadataState;

/// Initializes metadata with custom values.
/// Validates all provided fields before setting them on the state.
pub fn initialize_metadata(
    state: &mut MetadataState,
    name: String,
    symbol: String,
    uri: String,
    update_authority: Pubkey,
) -> Result<()> {
    if state.initialized {
        return Err(GhostError::AlreadyInitialized.into());
    }

    validation::validate_metadata(&name, &symbol, &uri)?;

    state.name = name;
    state.symbol = symbol;
    state.uri = uri;
    state.description = GHOST_DESCRIPTION.to_string();
    state.update_authority = update_authority;
    state.frozen = false;

    let clock = Clock::get()?;
    state.created_at = clock.unix_timestamp;
    state.updated_at = clock.unix_timestamp;
    state.initialized = true;

    Ok(())
}

/// Initializes metadata with default GHOST values.
/// Uses the canonical GHOST_NAME, GHOST_SYMBOL, and an empty URI.
pub fn initialize_default_metadata(
    state: &mut MetadataState,
    update_authority: Pubkey,
) -> Result<()> {
    if state.initialized {
        return Err(GhostError::AlreadyInitialized.into());
    }

    state.name = GHOST_NAME.to_string();
    state.symbol = GHOST_SYMBOL.to_string();
    state.uri = String::new();
    state.description = GHOST_DESCRIPTION.to_string();
    state.update_authority = update_authority;
    state.frozen = false;

    let clock = Clock::get()?;
    state.created_at = clock.unix_timestamp;
    state.updated_at = clock.unix_timestamp;
    state.initialized = true;

    Ok(())
}
