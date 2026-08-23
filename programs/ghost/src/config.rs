//! GHOST Token — Runtime configuration utilities.
//! Provides helpers for working with GlobalConfig state.

use crate::errors::GhostError;
use crate::state::GlobalConfig;
use anchor_lang::prelude::*;

/// Validates that the global config is in a consistent state.
pub fn validate_config(config: &GlobalConfig) -> Result<()> {
    require!(config.initialized, GhostError::NotInitialized);
    Ok(())
}

/// Checks that the program is not paused (standard or emergency).
pub fn check_not_paused(config: &GlobalConfig) -> Result<()> {
    require!(!config.emergency_paused, GhostError::EmergencyPaused);
    require!(!config.paused, GhostError::ProgramPaused);
    Ok(())
}
