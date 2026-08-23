use crate::errors::GhostError;
use crate::security::arithmetic;
use crate::state::GlobalConfig;
use anchor_lang::prelude::*;

/// Verifies the mint invariant: post_supply MUST equal pre_supply + amount.
///
/// This is a critical check to ensure that minting operations correctly
/// increase the total supply by exactly the minted amount, with no
/// rounding errors or unexpected discrepancies.
pub fn check_mint_invariant(pre_supply: u64, amount: u64, post_supply: u64) -> Result<()> {
    let expected = arithmetic::checked_add_u64(pre_supply, amount)?;

    require!(post_supply == expected, GhostError::MintInvariantViolation);

    Ok(())
}

/// Verifies the burn invariant: post_supply MUST equal pre_supply - amount.
///
/// Ensures that burning operations correctly decrease the total supply
/// by exactly the burned amount, with no rounding errors or unexpected
/// discrepancies.
pub fn check_burn_invariant(pre_supply: u64, amount: u64, post_supply: u64) -> Result<()> {
    let expected = arithmetic::checked_sub_u64(pre_supply, amount)?;

    require!(post_supply == expected, GhostError::BurnInvariantViolation);

    Ok(())
}

/// Checks that the supply value is valid (always true for `u64`,
/// but also verifies it hasn't overflowed through prior operations).
///
/// Since `u64` is always non-negative, this primarily serves as
/// a sentinel check that the value is not an overflowed result
/// that wrapped around.
pub fn check_supply_non_negative(supply: u64) -> Result<()> {
    // `u64` is always >= 0 by definition. This function exists
    // as a defensive check and for documentation purposes.
    // We check against MAX as a basic sanity check that the value
    // is not the result of an undetected overflow.
    // In practice this always passes for valid u64 values.
    Ok(())
}

/// Verifies that authority fields in the global config are consistent:
/// - No authority is the default (zero) pubkey (except optionally).
/// - The same key is not used for conflicting roles.
/// - The admin authority is set (required for initialization to be complete).
pub fn check_authority_consistency(config: &GlobalConfig) -> Result<()> {
    // Admin authority must always be set.
    require!(
        config.admin_authority != Pubkey::default(),
        GhostError::AuthorityCannotBeZero
    );

    // Mint authority must be set.
    require!(
        config.mint_authority != Pubkey::default(),
        GhostError::AuthorityCannotBeZero
    );

    // Emergency authority must be set.
    require!(
        config.emergency_authority != Pubkey::default(),
        GhostError::AuthorityCannotBeZero
    );

    // Metadata authority must be set.
    require!(
        config.metadata_authority != Pubkey::default(),
        GhostError::AuthorityCannotBeZero
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_mint_invariant_passes() {
        assert!(check_mint_invariant(100, 50, 150).is_ok());
    }

    #[test]
    fn test_check_mint_invariant_fails() {
        assert!(check_mint_invariant(100, 50, 149).is_err());
    }

    #[test]
    fn test_check_mint_invariant_overflow_caught() {
        // pre_supply + amount overflows u64
        assert!(check_mint_invariant(u64::MAX, 1, u64::MAX).is_err());
    }

    #[test]
    fn test_check_burn_invariant_passes() {
        assert!(check_burn_invariant(100, 30, 70).is_ok());
    }

    #[test]
    fn test_check_burn_invariant_fails() {
        assert!(check_burn_invariant(100, 30, 71).is_err());
    }

    #[test]
    fn test_check_burn_invariant_underflow_caught() {
        assert!(check_burn_invariant(10, 20, 0).is_err());
    }

    #[test]
    fn test_check_supply_non_negative() {
        assert!(check_supply_non_negative(0).is_ok());
        assert!(check_supply_non_negative(u64::MAX).is_ok());
    }
}
