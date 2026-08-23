use anchor_lang::prelude::*;

use super::checked::*;
use crate::errors::GhostError;

/// Converts a human-readable amount to raw token amount using the given decimals.
///
/// Example: human_to_raw(100, 9) => 100_000_000_000
pub fn human_to_raw(amount: u64, decimals: u8) -> Result<u64> {
    let multiplier = 10u64
        .checked_pow(decimals as u32)
        .ok_or::<Error>(GhostError::AmountOverflow.into())?;
    safe_mul(amount, multiplier)
}

/// Converts a raw token amount to a human-readable amount with remainder.
///
/// Returns (human_amount, remainder_raw).
/// The human_amount is the integer part, remainder_raw is the fractional part in raw units.
pub fn raw_to_human(raw: u64, decimals: u8) -> Result<(u64, u64)> {
    let multiplier = 10u64
        .checked_pow(decimals as u32)
        .ok_or::<Error>(GhostError::AmountOverflow.into())?;
    if multiplier == 0 {
        return Err(GhostError::ArithmeticError.into());
    }
    let human = safe_div(raw, multiplier)?;
    let remainder = raw.wrapping_sub(safe_mul(human, multiplier)?);
    Ok((human, remainder))
}

/// Validates that an amount is positive (>= 0). Always passes for u64 since it's unsigned.
pub fn validate_amount_positive(_amount: u64) -> Result<()> {
    // u64 is always >= 0
    Ok(())
}

/// Validates that an amount is not zero.
pub fn validate_amount_not_zero(amount: u64) -> Result<()> {
    if amount == 0 {
        return Err(GhostError::ZeroAmount.into());
    }
    Ok(())
}

/// Calculates the new total supply after a mint, checking for overflow.
pub fn calculate_total_supply_after_mint(current: u64, amount: u64) -> Result<u64> {
    safe_add(current, amount)
}

/// Calculates the new total supply after a burn, checking for underflow.
pub fn calculate_total_supply_after_burn(current: u64, amount: u64) -> Result<u64> {
    safe_sub(current, amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_to_raw() {
        assert_eq!(human_to_raw(100, 9).unwrap(), 100_000_000_000);
        assert_eq!(human_to_raw(1, 0).unwrap(), 1);
        assert_eq!(
            human_to_raw(10_000_000_000, 9).unwrap(),
            10_000_000_000_000_000_000
        );
    }

    #[test]
    fn test_raw_to_human() {
        let (human, rem) = raw_to_human(100_000_000_000, 9).unwrap();
        assert_eq!(human, 100);
        assert_eq!(rem, 0);

        let (human, rem) = raw_to_human(100_000_000_001, 9).unwrap();
        assert_eq!(human, 100);
        assert_eq!(rem, 1);
    }

    #[test]
    fn test_calculate_supply_after_mint() {
        assert_eq!(
            calculate_total_supply_after_mint(1_000_000, 500_000).unwrap(),
            1_500_000
        );
    }

    #[test]
    fn test_calculate_supply_after_burn() {
        assert_eq!(
            calculate_total_supply_after_burn(1_000_000, 500_000).unwrap(),
            500_000
        );
        assert!(calculate_total_supply_after_burn(100, 200).is_err());
    }
}
