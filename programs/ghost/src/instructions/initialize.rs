use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount},
};

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = global_config.key(),
        mint::token_program = token_program,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        space = GlobalConfig::LEN,
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[event]
pub struct GhostInitialized {
    pub mint: Pubkey,
    pub global_config: Pubkey,
    pub payer: Pubkey,
    pub version: u8,
    pub decimals: u8,
    pub timestamp: i64,
}

#[event]
pub struct InitialSupplyMinted {
    pub mint: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub total_supply: u64,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<InitializeAccounts>) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;
    let bump = ctx.bumps.global_config;

    // Compute raw supply using checked arithmetic
    let initial_supply_raw: u64 = INITIAL_SUPPLY_HUMAN
        .checked_mul(10u64.pow(TOKEN_DECIMALS as u32))
        .ok_or(GhostError::AmountOverflow)?;

    // Initialize GlobalConfig
    let config = &mut accounts.global_config;
    config.version = 1;
    config.bump = bump;
    config.initialized = true;
    config.paused = false;
    config.emergency_paused = false;
    config.name = TOKEN_NAME.to_string();
    config.symbol = TOKEN_SYMBOL.to_string();
    config.decimals = TOKEN_DECIMALS;
    config.mint = accounts.mint.key();
    config.mint_authority = Pubkey::default();
    config.admin_authority = Pubkey::default();
    config.metadata_authority = Pubkey::default();
    config.emergency_authority = Pubkey::default();
    config.metadata_uri = String::new();
    config.metadata_frozen = false;
    config.initialization_slot = clock.slot;
    config.initialization_timestamp = clock.unix_timestamp;
    config.total_supply = 0;
    config.pause_timestamp = 0;
    config.emergency_pause_timestamp = 0;

    // CPI: Mint initial supply from global_config PDA to payer's token account
    let seeds: &[&[u8]] = &[CONFIG_SEED, &[bump]];
    let signer_seeds: &[&[&[u8]]] = &[seeds];

    let global_config_info = config.to_account_info();
    let mint_info = accounts.mint.to_account_info();
    let token_account_info = accounts.token_account.to_account_info();
    let token_program_info = accounts.token_program.to_account_info();

    token_2022::mint_to(
        CpiContext::new_with_signer(
            token_program_info,
            token_2022::MintTo {
                mint: mint_info,
                to: token_account_info,
                authority: global_config_info,
            },
            signer_seeds,
        ),
        initial_supply_raw,
    )?;

    // Update config with initial supply
    config.total_supply = initial_supply_raw;

    let timestamp = clock.unix_timestamp;

    emit!(GhostInitialized {
        mint: accounts.mint.key(),
        global_config: accounts.global_config.key(),
        payer: accounts.payer.key(),
        version: 1,
        decimals: TOKEN_DECIMALS,
        timestamp,
    });

    emit!(InitialSupplyMinted {
        mint: accounts.mint.key(),
        recipient: accounts.payer.key(),
        amount: initial_supply_raw,
        total_supply: initial_supply_raw,
        timestamp,
    });

    Ok(())
}
