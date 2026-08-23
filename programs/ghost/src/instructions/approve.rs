use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::TokenAccount;


#[derive(Accounts)]
pub struct ApproveAccounts<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Delegate pubkey to approve for the token account
    pub delegate: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token2022>,
}

#[event]
pub struct TokensApproved {
    pub token_account: Pubkey,
    pub owner: Pubkey,
    pub delegate: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

pub fn handler(ctx: Context<ApproveAccounts>, amount: u64) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // CPI: approve for amount to delegate
    token_2022::approve(
        CpiContext::new(
            accounts.token_program.to_account_info(),
            token_2022::Approve {
                to: accounts.token_account.to_account_info(),
                delegate: accounts.delegate.to_account_info(),
                authority: accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    let timestamp = clock.unix_timestamp;

    emit!(TokensApproved {
        token_account: accounts.token_account.key(),
        owner: accounts.owner.key(),
        delegate: accounts.delegate.key(),
        amount,
        timestamp,
    });

    Ok(())
}
