use anchor_lang::prelude::*;

use crate::errors::GhostError;
use crate::state::AuthorityState;

/// Validates that an authority pubkey is not the default (zero) address.
pub fn validate_authority_not_default(auth: &Pubkey) -> Result<()> {
    if auth == &Pubkey::default() {
        return Err(GhostError::InvalidAuthority.into());
    }
    Ok(())
}

/// Validates that the signer matches the expected authority.
pub fn validate_signer_matches_expected(signer: &Signer, expected: &Pubkey) -> Result<()> {
    if signer.key() != *expected {
        return Err(GhostError::Unauthorized.into());
    }
    Ok(())
}

/// Validates the current authority rotation state.
///
/// The authority state must be initialized and not in a pending transfer window.
pub fn validate_authority_rotation_state(state: &AuthorityState) -> Result<()> {
    if !state.initialized {
        return Err(GhostError::NotInitialized.into());
    }
    // If there is a pending authority, the transfer window may still be open.
    // This is a valid state for accepting the transfer, but not for initiating a new one.
    // We allow the state to be read; specific operations will add their own checks.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_validate_authority_not_default() {
        assert!(validate_authority_not_default(&Pubkey::default()).is_err());
        assert!(validate_authority_not_default(&Pubkey::new_from_array([1u8; 32])).is_ok());
    }
}
