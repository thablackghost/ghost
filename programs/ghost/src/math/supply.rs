use anchor_lang::prelude::*;

use super::checked::*;
use crate::constants::{INITIAL_SUPPLY_HUMAN, TOKEN_DECIMALS};
use crate::errors::GhostError;
use crate::state::GlobalConfig;
use crate::state::SupplyOperation;

/// Computes the initial supply in raw token units.
///
/// Formula: INITIAL_SUPPLY (10B) * 10^TOKEN_DECIMALS (9) = 10_000_000_000_000_000_000
pub fn get_initial_supply_raw() -> Result<u64> {
    let multiplier = 10u64
        .checked_pow(TOKEN_DECIMALS as u32)
        .ok_or::<Error>(GhostError::AmountOverflow.into())?;
    safe_mul(INITIAL_SUPPLY_HUMAN, multiplier)
}

/// Verifies that a mint operation preserves the supply invariant:
///   new_supply == old_supply + amount
pub fn verify_supply_invariant_mint(old: u64, amount: u64, new: u64) -> Result<()> {
    let expected = safe_add(old, amount)?;
    if expected != new {
        return Err(GhostError::SupplyInvariantViolated.into());
    }
    Ok(())
}

/// Verifies that a burn operation preserves the supply invariant:
///   new_supply == old_supply - amount
pub fn verify_supply_invariant_burn(old: u64, amount: u64, new: u64) -> Result<()> {
    let expected = safe_sub(old, amount)?;
    if expected != new {
        return Err(GhostError::SupplyInvariantViolated.into());
    }
    Ok(())
}

/// Checks whether supply arithmetic is consistent.
///
/// pre + change == post must hold for the invariant to be satisfied.
/// change is i128 to allow representing negative deltas (burns).
pub fn is_supply_consistent(pre: u64, change: i128, post: u64) -> bool {
    let pre_i128 = pre as i128;
    let post_i128 = post as i128;
    pre_i128
        .checked_add(change)
        .map_or(false, |expected| expected == post_i128)
}

/// Checks all supply invariants for a given operation.
///
/// Validates:
/// - INVARIANT_1: Only configured mint authority can mint (must be checked upstream)
/// - INVARIANT_2: No burn can increase supply
/// - INVARIANT_3: No mint can decrease supply
pub fn check_all_supply_invariants(
    _config: &GlobalConfig,
    operation: SupplyOperation,
    pre: u64,
    post: u64,
    amount: u64,
) -> Result<()> {
    match operation {
        SupplyOperation::Mint => {
            // INVARIANT_3: No mint can decrease supply
            if post < pre {
                return Err(GhostError::SupplyInvariantViolated.into());
            }
            verify_supply_invariant_mint(pre, amount, post)?;
        }
        SupplyOperation::Burn => {
            // INVARIANT_2: No burn can increase supply
            if post > pre {
                return Err(GhostError::SupplyInvariantViolated.into());
            }
            verify_supply_invariant_burn(pre, amount, post)?;
        }
        SupplyOperation::Transfer => {
            // Transfers do not change total supply
            if pre != post {
                return Err(GhostError::SupplyInvariantViolated.into());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_supply() {
        let raw = get_initial_supply_raw().unwrap();
        assert_eq!(raw, 10_000_000_000_000_000_000);
    }

    #[test]
    fn test_supply_invariant_mint() {
        assert!(verify_supply_invariant_mint(100, 50, 150).is_ok());
        assert!(verify_supply_invariant_mint(100, 50, 200).is_err());
    }

    #[test]
    fn test_supply_invariant_burn() {
        assert!(verify_supply_invariant_burn(100, 50, 50).is_ok());
        assert!(verify_supply_invariant_burn(100, 50, 80).is_err());
    }

    #[test]
    fn test_is_supply_consistent() {
        assert!(is_supply_consistent(100, 50, 150));
        assert!(is_supply_consistent(100, -50, 50));
        assert!(!is_supply_consistent(100, 50, 200));
        assert!(!is_supply_consistent(100, -200, 50));
    }
}
