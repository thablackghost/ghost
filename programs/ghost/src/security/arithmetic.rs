use crate::errors::GhostError;
use anchor_lang::prelude::*;

/// Performs checked addition for two `u64` values.
/// Returns `GhostError::ArithmeticError` on overflow instead of panicking.
pub fn checked_add_u64(a: u64, b: u64) -> Result<u64> {
    a.checked_add(b).ok_or_else(|| {
        msg!("Arithmetic overflow: {} + {}", a, b);
        GhostError::ArithmeticError.into()
    })
}

/// Performs checked subtraction for two `u64` values.
/// Returns `GhostError::ArithmeticError` on underflow instead of panicking.
pub fn checked_sub_u64(a: u64, b: u64) -> Result<u64> {
    a.checked_sub(b).ok_or_else(|| {
        msg!("Arithmetic underflow: {} - {}", a, b);
        GhostError::ArithmeticError.into()
    })
}

/// Performs checked multiplication for two `u64` values.
/// Returns `GhostError::ArithmeticError` on overflow instead of panicking.
pub fn checked_mul_u64(a: u64, b: u64) -> Result<u64> {
    a.checked_mul(b).ok_or_else(|| {
        msg!("Arithmetic overflow: {} * {}", a, b);
        GhostError::ArithmeticError.into()
    })
}

/// Computes `base.pow(exp)` with overflow checking.
/// Returns `GhostError::ArithmeticError` on overflow instead of panicking.
pub fn checked_pow_u64(base: u64, exp: u32) -> Result<u64> {
    base.checked_pow(exp).ok_or_else(|| {
        msg!("Arithmetic overflow: {} ^ {}", base, exp);
        GhostError::ArithmeticError.into()
    })
}

/// Adds an amount to the current supply with overflow checking.
/// Specifically uses `GhostError::AmountOverflow` to distinguish supply
/// overflow from generic arithmetic errors.
pub fn checked_supply_add(current: u64, amount: u64) -> Result<u64> {
    current.checked_add(amount).ok_or_else(|| {
        msg!(
            "Supply overflow: current_supply={}, mint_amount={}",
            current,
            amount
        );
        GhostError::AmountOverflow.into()
    })
}

/// Subtracts an amount from the current supply with overflow (underflow) checking.
/// Specifically uses `GhostError::AmountOverflow` to distinguish supply
/// underflow from generic arithmetic errors.
pub fn checked_supply_sub(current: u64, amount: u64) -> Result<u64> {
    current.checked_sub(amount).ok_or_else(|| {
        msg!(
            "Supply underflow: current_supply={}, burn_amount={}",
            current,
            amount
        );
        GhostError::AmountOverflow.into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_add_u64_ok() {
        assert_eq!(checked_add_u64(1, 2).unwrap(), 3);
    }

    #[test]
    fn test_checked_add_u64_overflow() {
        assert!(checked_add_u64(u64::MAX, 1).is_err());
    }

    #[test]
    fn test_checked_sub_u64_ok() {
        assert_eq!(checked_sub_u64(5, 3).unwrap(), 2);
    }

    #[test]
    fn test_checked_sub_u64_underflow() {
        assert!(checked_sub_u64(0, 1).is_err());
    }

    #[test]
    fn test_checked_mul_u64_ok() {
        assert_eq!(checked_mul_u64(3, 4).unwrap(), 12);
    }

    #[test]
    fn test_checked_mul_u64_overflow() {
        assert!(checked_mul_u64(u64::MAX, 2).is_err());
    }

    #[test]
    fn test_checked_pow_u64_ok() {
        assert_eq!(checked_pow_u64(2, 10).unwrap(), 1024);
    }

    #[test]
    fn test_checked_pow_u64_overflow() {
        assert!(checked_pow_u64(2, 128).is_err());
    }

    #[test]
    fn test_checked_supply_add_overflow() {
        assert!(checked_supply_add(u64::MAX, 1).is_err());
    }

    #[test]
    fn test_checked_supply_sub_underflow() {
        assert!(checked_supply_sub(0, 1).is_err());
    }
}
