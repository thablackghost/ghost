use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct BurnAccounts<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [b"ghost_mint_config"],
        bump = mint_config.bump,
    )]
    pub mint_config: Account<'info, MintConfig>,

    pub token_program: Program<'info, Token2022>,
}

#[event]
pub struct TokensBurned {
    pub mint: Pubkey,
    pub amount: u64,
    pub burner: Pubkey,
    pub token_account: Pubkey,
    pub new_total_supply: u64,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<BurnAccounts>, amount: u64) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Validate amount
    require!(amount > 0, GhostError::ZeroAmount);

    // Verify mint matches config
    require!(
        accounts.mint.key() == accounts.global_config.mint,
        GhostError::MintMismatch
    );

    // Save pre-supply for invariant check
    let pre_supply = accounts.global_config.total_supply;

    // CPI: burn from token_account
    token_2022::burn(
        CpiContext::new(
            accounts.token_program.to_account_info(),
            token_2022::Burn {
                mint: accounts.mint.to_account_info(),
                from: accounts.token_account.to_account_info(),
                authority: accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;

    // Update config total_supply with checked arithmetic
    let new_total_supply = pre_supply
        .checked_sub(amount)
        .ok_or(GhostError::ArithmeticError)?;
    accounts.global_config.total_supply = new_total_supply;

    // Update mint_config tracking
    let mint_config = &mut accounts.mint_config;
    mint_config.total_burned = mint_config
        .total_burned
        .checked_add(amount)
        .ok_or(GhostError::AmountOverflow)?;
    mint_config.burn_count = mint_config
        .burn_count
        .checked_add(1)
        .ok_or(GhostError::AmountOverflow)?;

    // Supply invariant check
    require!(
        new_total_supply
            == pre_supply
                .checked_sub(amount)
                .ok_or(GhostError::ArithmeticError)?,
        GhostError::SupplyInvariantViolated
    );

    let timestamp = clock.unix_timestamp;

    emit!(TokensBurned {
        mint: accounts.mint.key(),
        amount,
        burner: accounts.authority.key(),
        token_account: accounts.token_account.key(),
        new_total_supply,
        timestamp,
    });

    Ok(())
}
