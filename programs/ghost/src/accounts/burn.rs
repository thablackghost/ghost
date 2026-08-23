//! GHOST Token — Burn tokens account context.
//!
//! The `Burn` struct defines accounts required for the
//! burn_tokens instruction. Any token holder can burn their
//! own tokens.

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::GlobalConfig;
use anchor_lang::prelude::*;
use anchor_spl::token_2022;

/// Accounts required for burning GHOST tokens.
///
/// # Authorization
/// The authority must be the owner of the source token account.
#[derive(Accounts)]
pub struct Burn<'info> {
    /// The token holder authorizing the burn. Must sign.
    pub authority: Signer<'info>,

    /// The GHOST token mint. Must be mutable (supply decreases).
    #[account(
        mut,
        address = global_config.mint @ GhostError::InvalidMint,
    )]
    pub mint: Box<InterfaceAccount<'info, token_2022::Mint>>,

    /// The source token account from which tokens will be burned.
    #[account(
        mut,
        constraint = token_account.mint == global_config.mint @ GhostError::MintMismatch,
        constraint = token_account.owner == authority.key() @ GhostError::InvalidOwner,
    )]
    pub token_account: Box<InterfaceAccount<'info, token_2022::TokenAccount>>,

    /// Global config PDA (read-only for validation and pause check).
    #[account(
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    /// SPL Token-2022 program.
    pub token_program: Program<'info, token_2022::Token2022>,
}
