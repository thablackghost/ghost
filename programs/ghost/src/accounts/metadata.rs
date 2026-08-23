//! GHOST Token — Metadata update account context.
//!
//! The `MetadataUpdate` struct defines accounts required for
//! updating on-chain token metadata (name, symbol, URI).

use crate::constants::*;
use crate::state::{GlobalConfig, MetadataState};
use anchor_lang::prelude::*;

/// Accounts required for updating on-chain metadata.
///
/// # Authorization
/// Only the metadata authority can update metadata.
/// Updates are rejected if the metadata is frozen.
#[derive(Accounts)]
pub struct MetadataUpdate<'info> {
    /// The metadata authority. Must sign.
    pub metadata_authority: Signer<'info>,

    /// The metadata state PDA (mutable — fields will be updated).
    #[account(
        mut,
        seeds = [METADATA_SEED],
        bump = metadata_state.bump,
    )]
    pub metadata_state: Account<'info, MetadataState>,

    /// Global config PDA (read-only for authority and freeze checks).
    #[account(
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    /// System program (for potential rent reallocation on resize).
    pub system_program: Program<'info, System>,
}
