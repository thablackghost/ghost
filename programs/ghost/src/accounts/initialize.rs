//! GHOST Token — Initialization account context.
//!
//! The `Initialize` struct defines all accounts required for
//! one-time program initialization: creating the mint, all PDAs,
//! and minting the initial supply.

use crate::constants::*;
use crate::state::{
    AuthorityState, GlobalConfig, MetadataState, MintConfig, PauseState, VersionState,
};
use anchor_lang::prelude::*;
use anchor_spl::token_2022;

/// Accounts required for program initialization.
///
/// One-time setup that creates the SPL Token-2022 mint, initializes
/// all program PDAs, and mints the initial supply to the destination.
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// The admin/payer authority. Must sign and covers rent for all PDAs.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The GHOST token mint account. Must be a signer (created via CPI
    /// in the handler or pre-created with the correct keypair).
    ///
    /// CHECK: Validated in the instruction handler against Token-2022
    ///        mint constraints.
    #[account(mut)]
    pub mint: Signer<'info>,

    /// Global config PDA — stores admin authorities and program flags.
    #[account(
        init,
        payer = payer,
        space = GlobalConfig::LEN,
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    /// Mint config PDA — tracks minting statistics and configuration.
    #[account(
        init,
        payer = payer,
        space = MintConfig::LEN,
        seeds = [b"ghost_mint_config"],
        bump,
    )]
    pub mint_config: Account<'info, MintConfig>,

    /// Authority state PDA — tracks authority rotation proposals.
    #[account(
        init,
        payer = payer,
        space = AuthorityState::LEN,
        seeds = [AUTHORITY_SEED],
        bump,
    )]
    pub authority_state: Account<'info, AuthorityState>,

    /// Metadata state PDA — stores on-chain token metadata.
    #[account(
        init,
        payer = payer,
        space = MetadataState::LEN,
        seeds = [METADATA_SEED],
        bump,
    )]
    pub metadata_state: Account<'info, MetadataState>,

    /// Pause state PDA — tracks pause/emergency-pause flags.
    #[account(
        init,
        payer = payer,
        space = PauseState::LEN,
        seeds = [PAUSE_SEED],
        bump,
    )]
    pub pause_state: Account<'info, PauseState>,

    /// Version state PDA — stores program version info.
    #[account(
        init,
        payer = payer,
        space = VersionState::LEN,
        seeds = [VERSION_SEED],
        bump,
    )]
    pub version_state: Account<'info, VersionState>,

    /// SPL Token-2022 program (validated against canonical ID).
    pub token_program: Program<'info, token_2022::Token2022>,

    /// Solana system program (for PDA creation).
    pub system_program: Program<'info, System>,

    /// Rent sysvar (for rent-exemption calculations).
    pub rent: Sysvar<'info, Rent>,
}
