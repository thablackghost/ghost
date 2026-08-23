use crate::errors::GhostError;
use anchor_lang::prelude::*;

/// Checks that the program state has NOT already been initialized.
/// This prevents reinitialization attacks where an attacker attempts
/// to re-run the initialization instruction to overwrite state.
pub fn check_not_reinitialized(initialized: bool) -> Result<()> {
    require!(!initialized, GhostError::AlreadyInitialized);
    Ok(())
}

/// Validates that a recorded slot number is not too old (stale).
///
/// If the difference between the current slot and the recorded slot
/// exceeds `max_slots`, the recorded slot is considered stale and
/// the operation is rejected. This is a form of replay protection
/// for operations that should be executed within a limited time window.
pub fn validate_slot_not_stale(
    current_slot: u64,
    recorded_slot: u64,
    max_slots: u64,
) -> Result<()> {
    let age = current_slot.checked_sub(recorded_slot).ok_or_else(|| {
        msg!(
            "Invalid slot ordering: current={}, recorded={}",
            current_slot,
            recorded_slot
        );
        GhostError::InvalidSlotOrdering
    })?;

    require!(age <= max_slots, GhostError::SlotTooStale);

    Ok(())
}

/// Validates that timestamps are in chronological order.
/// `before` must be strictly less than `after`.
pub fn validate_timestamp_order(before: i64, after: i64) -> Result<()> {
    require!(before < after, GhostError::InvalidTimestampOrder);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_not_reinitialized_fails_when_true() {
        assert!(check_not_reinitialized(true).is_err());
    }

    #[test]
    fn test_check_not_reinitialized_passes_when_false() {
        assert!(check_not_reinitialized(false).is_ok());
    }

    #[test]
    fn test_validate_slot_not_stale_passes() {
        assert!(validate_slot_not_stale(100, 90, 20).is_ok());
    }

    #[test]
    fn test_validate_slot_not_stale_fails_for_stale() {
        assert!(validate_slot_not_stale(100, 70, 20).is_err());
    }

    #[test]
    fn test_validate_slot_not_stale_fails_for_inverse() {
        // recorded_slot > current_slot is invalid.
        assert!(validate_slot_not_stale(90, 100, 20).is_err());
    }

    #[test]
    fn test_validate_timestamp_order_passes() {
        assert!(validate_timestamp_order(1, 2).is_ok());
    }

    #[test]
    fn test_validate_timestamp_order_fails_equal() {
        assert!(validate_timestamp_order(5, 5).is_err());
    }

    #[test]
    fn test_validate_timestamp_order_fails_inverse() {
        assert!(validate_timestamp_order(10, 5).is_err());
    }
}
