use anchor_lang::prelude::*;

use crate::errors::GhostError;

/// Validates that an account is marked as initialized.
pub fn validate_account_initialized(initialized: bool) -> Result<()> {
    if !initialized {
        return Err(GhostError::NotInitialized.into());
    }
    Ok(())
}

/// Validates that an account is NOT yet initialized (prevents double-init).
pub fn validate_account_not_initialized(initialized: bool) -> Result<()> {
    if initialized {
        return Err(GhostError::AlreadyInitialized.into());
    }
    Ok(())
}

/// Validates that a PDA derived from seeds matches the expected pubkey.
pub fn validate_pda_matches(expected: &Pubkey, actual: &Pubkey) -> Result<()> {
    if expected != actual {
        return Err(GhostError::InvalidPda.into());
    }
    Ok(())
}

/// Validates that the token program account matches the expected program ID.
///
/// For GHOST, this must always be the SPL Token-2022 program.
pub fn validate_token_program(program: &Pubkey, expected: &Pubkey) -> Result<()> {
    if program != expected {
        return Err(GhostError::TokenProgramMismatch.into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_validate_initialized() {
        assert!(validate_account_initialized(true).is_ok());
        assert!(validate_account_initialized(false).is_err());
    }

    #[test]
    fn test_validate_not_initialized() {
        assert!(validate_account_not_initialized(false).is_ok());
        assert!(validate_account_not_initialized(true).is_err());
    }

    #[test]
    fn test_validate_pda() {
        let p1 = Pubkey::new_unique();
        let p2 = Pubkey::new_unique();
        assert!(validate_pda_matches(&p1, &p1).is_ok());
        assert!(validate_pda_matches(&p1, &p2).is_err());
    }
}
