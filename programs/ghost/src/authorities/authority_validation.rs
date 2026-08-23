use crate::errors::GhostError;
use crate::state::GlobalConfig;
use anchor_lang::prelude::*;

/// Validates that an authority pubkey is not the default (zero) pubkey,
/// which would indicate an unset or invalid authority.
pub fn validate_authority_not_zero(authority: &Pubkey) -> Result<()> {
    require!(
        *authority != Pubkey::default(),
        GhostError::AuthorityCannotBeZero
    );
    Ok(())
}

/// Validates the preconditions for an authority rotation.
/// Ensures:
/// - The old and new authorities are not the same (no no-op rotation).
/// - The new authority is not the zero pubkey.
/// - The old authority is not the zero pubkey (must already be set).
pub fn validate_authority_rotation(old: &Pubkey, new: &Pubkey) -> Result<()> {
    require!(old != new, GhostError::AuthorityRotationNoChange);
    validate_authority_not_zero(new)?;
    validate_authority_not_zero(old)?;
    Ok(())
}

/// Validates that authority changes are currently allowed based on the
/// global config state (e.g., the program is not in emergency pause).
pub fn validate_authority_change_allowed(config: &GlobalConfig) -> Result<()> {
    require!(!config.emergency_paused, GhostError::AuthorityChangesLocked);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_validate_not_zero_passes_for_nonzero() {
        let key = Pubkey::new_unique();
        assert!(validate_authority_not_zero(&key).is_ok());
    }

    #[test]
    fn test_validate_not_zero_fails_for_zero() {
        let key = Pubkey::default();
        assert!(validate_authority_not_zero(&key).is_err());
    }

    #[test]
    fn test_rotation_fails_if_same_key() {
        let key = Pubkey::new_unique();
        assert!(validate_authority_rotation(&key, &key).is_err());
    }

    #[test]
    fn test_rotation_fails_if_new_is_zero() {
        let old = Pubkey::new_unique();
        assert!(validate_authority_rotation(&old, &Pubkey::default()).is_err());
    }

    #[test]
    fn test_rotation_passes_for_valid_change() {
        let old = Pubkey::new_unique();
        let new = Pubkey::new_unique();
        assert!(validate_authority_rotation(&old, &new).is_ok());
    }
}
