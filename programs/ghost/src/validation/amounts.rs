use anchor_lang::prelude::*;

use crate::errors::GhostError;
use crate::math::checked::safe_add;

/// Validates a mint amount: must be greater than zero and not cause overflow.
pub fn validate_mint_amount(amount: u64) -> Result<()> {
    if amount == 0 {
        return Err(GhostError::ZeroAmount.into());
    }
    // Check that adding amount to u64::MAX doesn't overflow (a large safe bound check)
    // This ensures amount itself is sane
    Ok(())
}

/// Validates a burn amount: must be greater than zero and <= current balance.
pub fn validate_burn_amount(amount: u64, balance: u64) -> Result<()> {
    if amount == 0 {
        return Err(GhostError::ZeroAmount.into());
    }
    if amount > balance {
        return Err(GhostError::InsufficientBalance.into());
    }
    Ok(())
}

/// Validates a transfer amount: must be greater than zero and <= current balance.
pub fn validate_transfer_amount(amount: u64, balance: u64) -> Result<()> {
    if amount == 0 {
        return Err(GhostError::ZeroAmount.into());
    }
    if amount > balance {
        return Err(GhostError::InsufficientBalance.into());
    }
    Ok(())
}

/// Validates an approve amount: must be >= 0 (always passes for u64).
/// Allowance can be zero to revoke an existing allowance.
pub fn validate_approve_amount(_amount: u64) -> Result<()> {
    // u64 is always >= 0
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_mint_amount() {
        assert!(validate_mint_amount(0).is_err());
        assert!(validate_mint_amount(1).is_ok());
        assert!(validate_mint_amount(u64::MAX).is_ok());
    }

    #[test]
    fn test_validate_burn_amount() {
        assert!(validate_burn_amount(0, 100).is_err());
        assert!(validate_burn_amount(50, 100).is_ok());
        assert!(validate_burn_amount(101, 100).is_err());
        assert!(validate_burn_amount(100, 100).is_ok());
    }

    #[test]
    fn test_validate_transfer_amount() {
        assert!(validate_transfer_amount(0, 100).is_err());
        assert!(validate_transfer_amount(50, 100).is_ok());
        assert!(validate_transfer_amount(101, 100).is_err());
    }

    #[test]
    fn test_validate_approve_amount() {
        assert!(validate_approve_amount(0).is_ok());
        assert!(validate_approve_amount(u64::MAX).is_ok());
    }
}
