use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::TokenAccount;

#[derive(Accounts)]
pub struct RevokeAccounts<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
}

#[event]
pub struct TokensRevoked {
    pub token_account: Pubkey,
    pub owner: Pubkey,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<RevokeAccounts>) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // CPI: revoke delegate from token_account
    token_2022::revoke(CpiContext::new(
        accounts.token_program.to_account_info(),
        token_2022::Revoke {
            source: accounts.token_account.to_account_info(),
            authority: accounts.owner.to_account_info(),
        },
    ))?;

    let timestamp = clock.unix_timestamp;

    emit!(TokensRevoked {
        token_account: accounts.token_account.key(),
        owner: accounts.owner.key(),
        timestamp,
    });

    Ok(())
}
