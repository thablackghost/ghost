use anchor_lang::prelude::*;

use super::checked::*;
use crate::errors::GhostError;

/// Default number of decimals for the GHOST token.
pub const DEFAULT_DECIMALS: u8 = 9;

/// Maximum allowed decimals for any token.
pub const MAX_DECIMALS: u8 = 18;

/// Returns 10^decimals as a u64. Returns ArithmeticError on overflow.
pub fn get_decimals_multiplier(decimals: u8) -> Result<u64> {
    validate_decimals(decimals)?;
    10u64
        .checked_pow(decimals as u32)
        .ok_or_else(|| GhostError::ArithmeticError.into())
}

/// Validates that decimal count is within the allowed range (0-18).
pub fn validate_decimals(decimals: u8) -> Result<()> {
    if decimals > MAX_DECIMALS {
        return Err(GhostError::InvalidDecimals.into());
    }
    Ok(())
}

/// Converts a raw amount from one decimal precision to another.
///
/// For example, converting 18-decimal USDC amount to 9-decimal GHOST:
///   convert_decimals(1_000_000_000_000_000_000, 18, 9) => 1_000_000_000
pub fn convert_decimals(amount: u64, from_decimals: u8, to_decimals: u8) -> Result<u64> {
    validate_decimals(from_decimals)?;
    validate_decimals(to_decimals)?;

    if from_decimals == to_decimals {
        return Ok(amount);
    }

    if from_decimals > to_decimals {
        let diff = from_decimals - to_decimals;
        let divisor = get_decimals_multiplier(diff)?;
        safe_div(amount, divisor)
    } else {
        let diff = to_decimals - from_decimals;
        let multiplier = get_decimals_multiplier(diff)?;
        safe_mul(amount, multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_decimals() {
        assert_eq!(DEFAULT_DECIMALS, 9);
    }

    #[test]
    fn test_validate_decimals() {
        assert!(validate_decimals(0).is_ok());
        assert!(validate_decimals(9).is_ok());
        assert!(validate_decimals(18).is_ok());
        assert!(validate_decimals(19).is_err());
        assert!(validate_decimals(255).is_err());
    }

    #[test]
    fn test_get_decimals_multiplier() {
        assert_eq!(get_decimals_multiplier(0).unwrap(), 1);
        assert_eq!(get_decimals_multiplier(1).unwrap(), 10);
        assert_eq!(get_decimals_multiplier(9).unwrap(), 1_000_000_000);
        assert_eq!(get_decimals_multiplier(18).unwrap(), 1_000_000_000_000_000_000);
    }

    #[test]
    fn test_convert_decimals_same() {
        assert_eq!(convert_decimals(100, 9, 9).unwrap(), 100);
    }

    #[test]
    fn test_convert_decimals_down() {
        // 1000 raw with 6 decimals => 1 raw with 3 decimals
        assert_eq!(convert_decimals(1000, 6, 3).unwrap(), 1);
    }

    #[test]
    fn test_convert_decimals_up() {
        // 1 raw with 0 decimals => 1_000_000_000 raw with 9 decimals
        assert_eq!(convert_decimals(1, 0, 9).unwrap(), 1_000_000_000);
    }
}
