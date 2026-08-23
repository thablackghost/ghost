use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct TransferAccounts<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub from: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub to: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    pub token_program: Program<'info, Token2022>,
}

#[event]
pub struct TokensTransferred {
    pub mint: Pubkey,
    pub amount: u64,
    pub from: Pubkey,
    pub to: Pubkey,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<TransferAccounts>, amount: u64) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Validate amount
    require!(amount > 0, GhostError::ZeroAmount);

    // Check not paused
    require!(!accounts.global_config.paused, GhostError::ProgramPaused);
    require!(
        !accounts.global_config.emergency_paused,
        GhostError::EmergencyPaused
    );

    // CPI: transfer_checked with decimals=9
    token_2022::transfer_checked(
        CpiContext::new(
            accounts.token_program.to_account_info(),
            token_2022::TransferChecked {
                from: accounts.from.to_account_info(),
                mint: accounts.mint.to_account_info(),
                to: accounts.to.to_account_info(),
                authority: accounts.authority.to_account_info(),
            },
        ),
        amount,
        TOKEN_DECIMALS,
    )?;

    let timestamp = clock.unix_timestamp;

    emit!(TokensTransferred {
        mint: accounts.mint.key(),
        amount,
        from: accounts.from.key(),
        to: accounts.to.key(),
        timestamp,
    });

    Ok(())
}
