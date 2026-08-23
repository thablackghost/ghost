use crate::errors::GhostError;
use crate::state::GlobalConfig;
use anchor_lang::prelude::*;

/// Authority responsible for administrative operations on the GHOST token program.
/// The admin can update configuration parameters, manage other authorities,
/// and perform protocol-level changes.
pub struct AdminAuthority;

impl AdminAuthority {
    /// Checks whether the given pubkey is registered as an admin in the global config.
    pub fn is_admin(authority: &Pubkey, config: &GlobalConfig) -> bool {
        authority == &config.admin_authority
    }

    /// Validates that the signer is the configured admin authority.
    /// Returns `UnauthorizedAdmin` if the signer's key does not match.
    pub fn validate_admin<'info>(authority: &Signer<'info>, config: &GlobalConfig) -> Result<()> {
        require!(
            Self::is_admin(&authority.key(), config),
            GhostError::UnauthorizedAdmin
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_is_admin_match() {
        let key = Pubkey::new_unique();
        let config = GlobalConfig {
            admin_authority: key,
            ..Default::default()
        };
        assert!(AdminAuthority::is_admin(&key, &config));
    }

    #[test]
    fn test_is_admin_mismatch() {
        let config = GlobalConfig {
            admin_authority: Pubkey::new_unique(),
            ..Default::default()
        };
        assert!(!AdminAuthority::is_admin(&Pubkey::new_unique(), &config));
    }
}
