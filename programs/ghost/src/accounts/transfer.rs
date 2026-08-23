//! GHOST Token — Transfer tokens account context.
//!
//! The `Transfer` struct defines accounts required for the
//! standard transfer_tokens instruction.

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::GlobalConfig;
use anchor_lang::prelude::*;
use anchor_spl::token_2022;

/// Accounts required for transferring GHOST tokens.
///
/// Standard SPL Token-2022 transfer with mint validation and
/// pause gating.
#[derive(Accounts)]
pub struct Transfer<'info> {
    /// The token holder authorizing the transfer. Must sign.
    pub authority: Signer<'info>,

    /// The source token account (debited).
    #[account(
        mut,
        constraint = from.mint == global_config.mint @ GhostError::MintMismatch,
        constraint = from.owner == authority.key() @ GhostError::InvalidOwner,
    )]
    pub from: Box<InterfaceAccount<'info, token_2022::TokenAccount>>,

    /// The destination token account (credited).
    #[account(
        mut,
        constraint = to.mint == global_config.mint @ GhostError::MintMismatch,
    )]
    pub to: Box<InterfaceAccount<'info, token_2022::TokenAccount>>,

    /// The GHOST token mint (read-only for validation).
    #[account(
        address = global_config.mint @ GhostError::InvalidMint,
    )]
    pub mint: Box<InterfaceAccount<'info, token_2022::Mint>>,

    /// Global config PDA (read-only for pause check).
    #[account(
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    /// SPL Token-2022 program.
    pub token_program: Program<'info, token_2022::Token2022>,
}
