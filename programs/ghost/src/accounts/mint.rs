//! GHOST Token — Mint tokens account context.
//!
//! The `Mint` struct defines accounts required for the
//! admin-only mint_tokens instruction.

use anchor_lang::prelude::*;
use anchor_spl::token_2022;
use crate::constants::*;
use crate::errors::GhostError;
use crate::state::{GlobalConfig, MintConfig, AuthorityState};

/// Accounts required for minting new GHOST tokens.
///
/// # Authorization
/// Only the mint authority (admin) can call this instruction.
/// The destination token account must belong to the canonical
/// GHOST mint.
#[derive(Accounts)]
pub struct Mint<'info> {
    /// The mint authority (admin). Must sign.
    pub mint_authority: Signer<'info>,

    /// The GHOST token mint. Must be mutable (supply changes).
    #[account(
        mut,
        address = global_config.mint @ GhostError::InvalidMint,
    )]
    pub mint: Box<InterfaceAccount<'info, token_2022::Mint>>,

    /// The destination token account to receive minted tokens.
    #[account(
        mut,
        constraint = token_account.mint == global_config.mint @ GhostError::MintMismatch,
    )]
    pub token_account: Box<InterfaceAccount<'info, token_2022::TokenAccount>>,

    /// Mint config PDA — tracks mint statistics.
    #[account(mut)]
    pub mint_config: Account<'info, MintConfig>,

    /// Global config PDA (read-only for authority and pause checks).
    #[account(
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    /// Authority state PDA (read-only for rotation validation).
    #[account(
        seeds = [AUTHORITY_SEED],
        bump = authority_state.bump,
    )]
    pub authority_state: Account<'info, AuthorityState>,

    /// SPL Token-2022 program.
    pub token_program: Program<'info, token_2022::Token2022>,
}
