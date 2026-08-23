use anchor_lang::prelude::*;

use super::validation;
use crate::errors::GhostError;
use crate::state::MetadataState;

/// Updates metadata fields. Only provided (Some) fields are updated.
/// All new values are validated before being applied.
/// Metadata must not be frozen.
pub fn update_metadata_fields(
    state: &mut MetadataState,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
) -> Result<()> {
    require!(!state.frozen, GhostError::MetadataFrozen);

    if let Some(ref new_name) = name {
        validation::validate_name(new_name)?;
        state.name = new_name.clone();
    }

    if let Some(ref new_symbol) = symbol {
        validation::validate_symbol(new_symbol)?;
        state.symbol = new_symbol.clone();
    }

    if let Some(ref new_uri) = uri {
        validation::validate_uri(new_uri)?;
        state.uri = new_uri.clone();
    }

    let clock = Clock::get()?;
    state.updated_at = clock.unix_timestamp;

    Ok(())
}

/// Freezes metadata so that no further updates are possible. This is irreversible.
pub fn freeze_metadata(state: &mut MetadataState) -> Result<()> {
    require!(!state.frozen, GhostError::MetadataFrozen);
    state.frozen = true;

    let clock = Clock::get()?;
    state.updated_at = clock.unix_timestamp;

    Ok(())
}

/// Returns whether the metadata is currently frozen.
pub fn is_metadata_frozen(state: &MetadataState) -> bool {
    state.frozen
}
