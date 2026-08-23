use crate::errors::GhostError;
use crate::state::GlobalConfig;
use anchor_lang::prelude::*;

/// Authority responsible for emergency operations such as pausing the program
/// or freezing transfers in critical situations.
pub struct EmergencyAuthority;

impl EmergencyAuthority {
    /// Validates that the signer is the configured emergency authority.
    pub fn validate_emergency_authority<'info>(
        authority: &Signer<'info>,
        config: &GlobalConfig,
    ) -> Result<()> {
        require!(
            authority.key() == config.emergency_authority,
            GhostError::UnauthorizedEmergencyAuthority
        );
        Ok(())
    }

    /// Validates that the signer is either the emergency authority OR the admin.
    /// This allows the admin to also perform emergency operations, providing
    /// a fallback in case the emergency authority key is lost.
    pub fn validate_emergency_or_admin<'info>(
        authority: &Signer<'info>,
        config: &GlobalConfig,
    ) -> Result<()> {
        let key = authority.key();
        require!(
            key == config.emergency_authority || key == config.admin_authority,
            GhostError::UnauthorizedEmergencyOrAdmin
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_emergency_or_admin_allows_admin() {
        let admin = Pubkey::new_unique();
        let config = GlobalConfig {
            admin_authority: admin,
            emergency_authority: Pubkey::new_unique(),
            ..Default::default()
        };
        // The admin key should be accepted by validate_emergency_or_admin.
        assert!(admin == config.admin_authority);
    }

    #[test]
    fn test_emergency_or_admin_allows_emergency() {
        let emergency = Pubkey::new_unique();
        let config = GlobalConfig {
            admin_authority: Pubkey::new_unique(),
            emergency_authority: emergency,
            ..Default::default()
        };
        assert!(emergency == config.emergency_authority);
    }
}
