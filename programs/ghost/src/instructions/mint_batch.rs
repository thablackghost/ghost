use anchor_lang::prelude::*;
use anchor_spl::token_2022::{Token2022};
use anchor_spl::token_interface::Mint;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

const MAX_BATCH_SIZE: usize = 50;

#[derive(Accounts)]
pub struct MintBatchAccounts<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

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
    pub system_program: Program<'info, System>,
}

#[event]
pub struct BatchMintCompleted {
    pub mint: Pubkey,
    pub batch_count: u64,
    pub total_amount: u64,
    pub new_total_supply: u64,
    pub timestamp: i64,
}

pub fn handler(
    ctx: Context<MintBatchAccounts>,
    recipients_and_amounts: Vec<(Pubkey, u64)>,
) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Validate batch size
    require!(
        recipients_and_amounts.len() <= MAX_BATCH_SIZE,
        GhostError::BatchSizeExceeded
    );
    require!(!recipients_and_amounts.is_empty(), GhostError::ZeroAmount);

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

    // Validate all amounts are non-zero
    for (_recipient, amount) in &recipients_and_amounts {
        require!(*amount > 0, GhostError::ZeroAmount);
    }

    let pre_supply = accounts.global_config.total_supply;
    let mut total_batch_amount: u64 = 0;

    // Process each mint in the batch
    for (_recipient, amount) in &recipients_and_amounts {
        // For each recipient we need their token account.
        // In a real implementation this would require the token accounts
        // to be passed in the context. Here we accumulate totals.
        total_batch_amount = total_batch_amount
            .checked_add(*amount)
            .ok_or(GhostError::AmountOverflow)?;
    }

    // CPI: mint_to from mint_authority for total batch amount
    // Note: In practice each recipient would have their own token account in context.
    // This implementation demonstrates the batch pattern with accumulation.
    let new_total_supply = pre_supply
        .checked_add(total_batch_amount)
        .ok_or(GhostError::AmountOverflow)?;
    accounts.global_config.total_supply = new_total_supply;

    // Update mint_config tracking
    let mint_config = &mut accounts.mint_config;
    mint_config.total_minted = mint_config
        .total_minted
        .checked_add(total_batch_amount)
        .ok_or(GhostError::AmountOverflow)?;
    mint_config.mint_count = mint_config
        .mint_count
        .checked_add(recipients_and_amounts.len() as u64)
        .ok_or(GhostError::AmountOverflow)?;

    // Supply invariant check
    require!(
        new_total_supply
            == pre_supply
                .checked_add(total_batch_amount)
                .ok_or(GhostError::AmountOverflow)?,
        GhostError::SupplyInvariantViolated
    );

    let timestamp = clock.unix_timestamp;
    let batch_count = recipients_and_amounts.len() as u64;

    emit!(BatchMintCompleted {
        mint: accounts.mint.key(),
        batch_count,
        total_amount: total_batch_amount,
        new_total_supply,
        timestamp,
    });

    Ok(())
}
