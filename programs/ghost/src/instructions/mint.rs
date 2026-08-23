use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct MintAccounts<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"ghost_mint_config"],
        bump = mint_config.bump,
    )]
    pub mint_config: Account<'info, MintConfig>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    pub token_program: Program<'info, Token2022>,
}

#[event]
pub struct TokensMinted {
    pub mint: Pubkey,
    pub amount: u64,
    pub recipient: Pubkey,
    pub new_total_supply: u64,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<MintAccounts>, amount: u64) -> Result<()> {
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

    // Verify mint authority
    require!(
        accounts.mint_authority.key() == accounts.global_config.mint_authority,
        GhostError::Unauthorized
    );

    // Verify mint matches config
    require!(
        accounts.mint.key() == accounts.global_config.mint,
        GhostError::MintMismatch
    );

    // Save pre-supply for invariant check
    let pre_supply = accounts.global_config.total_supply;

    // CPI: mint_to from mint_authority to token_account
    token_2022::mint_to(
        CpiContext::new(
            accounts.token_program.to_account_info(),
            token_2022::MintTo {
                mint: accounts.mint.to_account_info(),
                to: accounts.token_account.to_account_info(),
                authority: accounts.mint_authority.to_account_info(),
            },
        ),
        amount,
    )?;

    // Update config total_supply with checked arithmetic
    let new_total_supply = pre_supply
        .checked_add(amount)
        .ok_or(GhostError::AmountOverflow)?;
    accounts.global_config.total_supply = new_total_supply;

    // Update mint_config tracking
    let mint_config = &mut accounts.mint_config;
    mint_config.total_minted = mint_config
        .total_minted
        .checked_add(amount)
        .ok_or(GhostError::AmountOverflow)?;
    mint_config.mint_count = mint_config
        .mint_count
        .checked_add(1)
        .ok_or(GhostError::AmountOverflow)?;

    // Supply invariant check
    require!(
        new_total_supply
            == pre_supply
                .checked_add(amount)
                .ok_or(GhostError::AmountOverflow)?,
        GhostError::SupplyInvariantViolated
    );

    let timestamp = clock.unix_timestamp;

    emit!(TokensMinted {
        mint: accounts.mint.key(),
        amount,
        recipient: accounts.token_account.key(),
        new_total_supply,
        timestamp,
    });

    Ok(())
}
