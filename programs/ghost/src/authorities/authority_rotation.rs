use crate::errors::GhostError;
use crate::state::AuthorityState;
use anchor_lang::prelude::*;

/// The status of an authority rotation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RotationStatus {
    /// No rotation is currently proposed.
    Idle,
    /// A rotation has been proposed and is within the timelock period.
    /// Contains the proposed authority and the timestamp when it can be accepted.
    Pending { proposed: Pubkey, accept_after: i64 },
    /// The timelock has elapsed and the rotation can be accepted.
    Ready { proposed: Pubkey },
}

/// Proposes a new authority rotation with a timelock.
///
/// Records the proposed new authority and the current timestamp.
/// The rotation can only be accepted after `state.timelock_duration` seconds
/// have elapsed since the proposal.
pub fn propose_rotation(state: &mut AuthorityState, new: Pubkey, now: i64) -> Result<()> {
    require!(
        state.proposed_authority == Pubkey::default(),
        GhostError::RotationAlreadyProposed
    );
    require!(new != Pubkey::default(), GhostError::AuthorityCannotBeZero);
    require!(
        new != state.current_authority,
        GhostError::AuthorityRotationNoChange
    );

    state.proposed_authority = new;
    state.proposed_at = now;

    msg!(
        "Authority rotation proposed: current={}, proposed={}, proposed_at={}",
        state.current_authority,
        new,
        now
    );

    Ok(())
}

/// Accepts a pending authority rotation after the timelock has elapsed.
///
/// Swaps the current authority with the proposed one and resets the proposal state.
pub fn accept_rotation(state: &mut AuthorityState, now: i64) -> Result<()> {
    require!(
        state.proposed_authority != Pubkey::default(),
        GhostError::NoRotationProposed
    );

    let elapsed = now
        .checked_sub(state.proposed_at)
        .ok_or(GhostError::InvalidTimestamp)?;

    require!(
        elapsed >= state.timelock_duration,
        GhostError::RotationTimelockNotElapsed
    );

    msg!(
        "Authority rotation accepted: old={}, new={}",
        state.current_authority,
        state.proposed_authority
    );

    state.current_authority = state.proposed_authority;
    state.proposed_authority = Pubkey::default();
    state.proposed_at = 0;

    Ok(())
}

/// Cancels a pending authority rotation before it is accepted.
pub fn cancel_rotation(state: &mut AuthorityState) -> Result<()> {
    require!(
        state.proposed_authority != Pubkey::default(),
        GhostError::NoRotationProposed
    );

    msg!(
        "Authority rotation cancelled: proposed={}",
        state.proposed_authority
    );

    state.proposed_authority = Pubkey::default();
    state.proposed_at = 0;

    Ok(())
}

/// Returns the current status of the authority rotation.
pub fn get_rotation_status(state: &AuthorityState, now: i64) -> RotationStatus {
    if state.proposed_authority == Pubkey::default() {
        return RotationStatus::Idle;
    }

    let accept_after = match state.proposed_at.checked_add(state.timelock_duration) {
        Some(t) => t,
        None => {
            // Overflow protection: if timelock_duration is unreasonably large,
            // treat as never ready. Return pending with a far-future timestamp.
            return RotationStatus::Pending {
                proposed: state.proposed_authority,
                accept_after: i64::MAX,
            };
        }
    };

    if now >= accept_after {
        RotationStatus::Ready {
            proposed: state.proposed_authority,
        }
    } else {
        RotationStatus::Pending {
            proposed: state.proposed_authority,
            accept_after,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    fn new_state(current: Pubkey, timelock: i64) -> AuthorityState {
        AuthorityState {
            current_authority: current,
            proposed_authority: Pubkey::default(),
            proposed_at: 0,
            timelock_duration: timelock,
        }
    }

    #[test]
    fn test_propose_and_accept_rotation() {
        let current = Pubkey::new_unique();
        let new = Pubkey::new_unique();
        let mut state = new_state(current, 86_400); // 24 hours

        assert!(propose_rotation(&mut state, new, 1_000_000).is_ok());
        assert_eq!(state.proposed_authority, new);
        assert_eq!(state.proposed_at, 1_000_000);

        // Accept before timelock should fail.
        assert!(accept_rotation(&mut state, 1_000_000 + 100).is_err());

        // Accept after timelock should succeed.
        assert!(accept_rotation(&mut state, 1_000_000 + 86_400).is_ok());
        assert_eq!(state.current_authority, new);
        assert_eq!(state.proposed_authority, Pubkey::default());
    }

    #[test]
    fn test_cancel_rotation() {
        let current = Pubkey::new_unique();
        let new = Pubkey::new_unique();
        let mut state = new_state(current, 86_400);

        propose_rotation(&mut state, new, 1_000_000).unwrap();
        assert!(cancel_rotation(&mut state).is_ok());
        assert_eq!(state.proposed_authority, Pubkey::default());
    }

    #[test]
    fn test_cancel_without_proposal_fails() {
        let current = Pubkey::new_unique();
        let mut state = new_state(current, 86_400);
        assert!(cancel_rotation(&mut state).is_err());
    }

    #[test]
    fn test_rotation_status() {
        let current = Pubkey::new_unique();
        let new = Pubkey::new_unique();
        let mut state = new_state(current, 100);

        // Idle
        assert_eq!(get_rotation_status(&state, 0), RotationStatus::Idle);

        // Propose
        propose_rotation(&mut state, new, 1_000).unwrap();

        // Pending
        match get_rotation_status(&state, 1_050) {
            RotationStatus::Pending {
                proposed,
                accept_after,
            } => {
                assert_eq!(proposed, new);
                assert_eq!(accept_after, 1_100);
            }
            _ => panic!("Expected Pending"),
        }

        // Ready
        match get_rotation_status(&state, 1_100) {
            RotationStatus::Ready { proposed } => {
                assert_eq!(proposed, new);
            }
            _ => panic!("Expected Ready"),
        }
    }

    #[test]
    fn test_propose_same_authority_fails() {
        let current = Pubkey::new_unique();
        let mut state = new_state(current, 86_400);
        assert!(propose_rotation(&mut state, current, 0).is_err());
    }

    #[test]
    fn test_propose_zero_authority_fails() {
        let current = Pubkey::new_unique();
        let mut state = new_state(current, 86_400);
        assert!(propose_rotation(&mut state, Pubkey::default(), 0).is_err());
    }
}
