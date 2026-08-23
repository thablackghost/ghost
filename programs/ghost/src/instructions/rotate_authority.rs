use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::GhostError;
use crate::state::*;

const ACTION_PROPOSE: u8 = 0;
const ACTION_ACCEPT: u8 = 1;
const ACTION_CANCEL: u8 = 2;

// Minimum delay for authority rotation acceptance (in seconds)
const MIN_PROPOSAL_DELAY: i64 = 86400; // 24 hours

#[derive(Accounts)]
pub struct RotateAuthorityAccounts<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [AUTHORITY_SEED],
        bump = authority_state.bump,
    )]
    pub authority_state: Account<'info, AuthorityState>,

    #[account(
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,
}

#[event]
pub struct AuthorityRotationProposed {
    pub authority_state: Pubkey,
    pub authority_type: u8,
    pub proposed_authority: Pubkey,
    pub proposal_timestamp: i64,
    pub delay: i64,
    pub timestamp: i64,
}

#[event]
pub struct AuthorityRotationAccepted {
    pub authority_state: Pubkey,
    pub authority_type: u8,
    pub new_authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AuthorityRotationCancelled {
    pub authority_state: Pubkey,
    pub authority_type: u8,
    pub cancelled_authority: Pubkey,
    pub timestamp: i64,
}

pub fn handler(
    ctx: Context<RotateAuthorityAccounts>,
    action: u8,
    new_authority: Pubkey,
) -> Result<()> {
    let accounts = ctx.accounts;
    let clock = Clock::get()?;

    // Verify admin authority
    require!(
        accounts.admin.key() == accounts.global_config.admin_authority,
        GhostError::Unauthorized
    );

    let authority_state = &mut accounts.authority_state;
    let timestamp = clock.unix_timestamp;

    match action {
        ACTION_PROPOSE => {
            // Propose a new authority
            require!(
                new_authority != Pubkey::default(),
                GhostError::InvalidAuthority
            );

            let _previous_proposed = authority_state.proposed_authority;
            let authority_type = authority_state.proposed_authority_type;

            authority_state.proposed_authority = new_authority;
            authority_state.proposal_timestamp = timestamp;
            authority_state.proposed_delay = MIN_PROPOSAL_DELAY;

            emit!(AuthorityRotationProposed {
                authority_state: accounts.authority_state.key(),
                authority_type,
                proposed_authority: new_authority,
                proposal_timestamp: timestamp,
                delay: MIN_PROPOSAL_DELAY,
                timestamp,
            });
        }
        ACTION_ACCEPT => {
            // Accept the proposed authority after delay
            let proposal_ts = authority_state.proposal_timestamp;
            let delay = authority_state.proposed_delay;
            let proposed = authority_state.proposed_authority;
            let authority_type = authority_state.proposed_authority_type;

            require!(proposed != Pubkey::default(), GhostError::InvalidAuthority);

            // Check that the proposal delay has passed
            let elapsed = timestamp
                .checked_sub(proposal_ts)
                .ok_or(GhostError::ArithmeticError)?;
            require!(elapsed >= delay, GhostError::InvalidState);

            // Apply the rotation: set current = proposed, clear proposed
            let global_config = &mut accounts.global_config;

            match authority_type {
                0 => {
                    global_config.mint_authority = proposed;
                    authority_state.mint_authority = proposed;
                }
                1 => {
                    global_config.admin_authority = proposed;
                    authority_state.admin_authority = proposed;
                }
                2 => {
                    global_config.metadata_authority = proposed;
                    authority_state.metadata_authority = proposed;
                }
                3 => {
                    global_config.emergency_authority = proposed;
                    authority_state.emergency_authority = proposed;
                }
                _ => return Err(GhostError::InvalidConfiguration.into()),
            }

            // Clear the proposal
            authority_state.proposed_authority = Pubkey::default();
            authority_state.proposal_timestamp = 0;
            authority_state.proposed_delay = 0;

            emit!(AuthorityRotationAccepted {
                authority_state: accounts.authority_state.key(),
                authority_type,
                new_authority: proposed,
                timestamp,
            });
        }
        ACTION_CANCEL => {
            // Cancel the pending proposal
            let cancelled = authority_state.proposed_authority;
            let authority_type = authority_state.proposed_authority_type;

            require!(cancelled != Pubkey::default(), GhostError::InvalidState);

            authority_state.proposed_authority = Pubkey::default();
            authority_state.proposal_timestamp = 0;
            authority_state.proposed_delay = 0;

            emit!(AuthorityRotationCancelled {
                authority_state: accounts.authority_state.key(),
                authority_type,
                cancelled_authority: cancelled,
                timestamp,
            });
        }
        _ => return Err(GhostError::InvalidConfiguration.into()),
    }

    Ok(())
}
