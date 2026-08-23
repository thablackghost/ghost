use anchor_lang::prelude::*;

use crate::errors::GhostError;

/// Safe addition. Returns ArithmeticError on overflow.
pub fn safe_add(a: u64, b: u64) -> Result<u64> {
    a.checked_add(b)
        .ok_or_else(|| GhostError::ArithmeticError.into())
}

/// Safe subtraction. Returns ArithmeticError on underflow.
pub fn safe_sub(a: u64, b: u64) -> Result<u64> {
    a.checked_sub(b)
        .ok_or_else(|| GhostError::ArithmeticError.into())
}

/// Safe multiplication. Returns AmountOverflow on overflow.
pub fn safe_mul(a: u64, b: u64) -> Result<u64> {
    a.checked_mul(b)
        .ok_or_else(|| GhostError::AmountOverflow.into())
}

/// Safe division. Returns ArithmeticError on division by zero.
pub fn safe_div(a: u64, b: u64) -> Result<u64> {
    if b == 0 {
        return Err(GhostError::ArithmeticError.into());
    }
    a.checked_div(b)
        .ok_or_else(|| GhostError::ArithmeticError.into())
}

/// Safe exponentiation. base^exp using checked multiplication.
/// Returns ArithmeticError on overflow.
pub fn safe_pow(base: u64, exp: u32) -> Result<u64> {
    let mut result: u64 = 1;
    for _ in 0..exp {
        result = safe_mul(result, base)?;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_add() {
        assert_eq!(safe_add(5, 3).unwrap(), 8);
        assert!(safe_add(u64::MAX, 1).is_err());
    }

    #[test]
    fn test_safe_sub() {
        assert_eq!(safe_sub(5, 3).unwrap(), 2);
        assert!(safe_sub(1, 5).is_err());
    }

    #[test]
    fn test_safe_mul() {
        assert_eq!(safe_mul(5, 3).unwrap(), 15);
        assert!(safe_mul(u64::MAX, 2).is_err());
    }

    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(10, 2).unwrap(), 5);
        assert!(safe_div(10, 0).is_err());
    }

    #[test]
    fn test_safe_pow() {
        assert_eq!(safe_pow(2, 10).unwrap(), 1024);
        assert_eq!(safe_pow(10, 9).unwrap(), 1_000_000_000);
        assert!(safe_pow(2, 64).is_err());
    }
}
