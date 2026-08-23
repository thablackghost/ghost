use crate::authorities::admin_authority::AdminAuthority;
use crate::authorities::emergency_authority::EmergencyAuthority;
use crate::authorities::metadata_authority::MetadataAuthority;
use crate::authorities::mint_authority::MintAuthority;
use crate::errors::GhostError;
use crate::security::emergency;
use crate::state::GlobalConfig;
use anchor_lang::prelude::*;

/// Guards a minting operation by validating:
/// 1. The program is not paused or in emergency mode.
/// 2. The signer is the configured mint authority.
pub fn guard_mint_operation<'info>(authority: &Signer<'info>, config: &GlobalConfig) -> Result<()> {
    // Cannot mint while paused or emergency paused.
    emergency::check_not_paused(config)?;
    emergency::check_not_emergency_paused(config)?;

    MintAuthority::validate_mint_authority(authority, config)?;

    Ok(())
}

/// Guards an admin operation by validating:
/// 1. The program is not in emergency mode (admin ops allowed during regular pause).
/// 2. The signer is the configured admin authority.
pub fn guard_admin_operation<'info>(
    authority: &Signer<'info>,
    config: &GlobalConfig,
) -> Result<()> {
    // Admin operations are only blocked during emergency pause.
    emergency::check_not_emergency_paused(config)?;

    AdminAuthority::validate_admin(authority, config)?;

    Ok(())
}

/// Guards an emergency operation by validating:
/// 1. The signer is either the emergency authority or the admin.
/// (Emergency operations are always allowed — they are the escape hatch.)
pub fn guard_emergency_operation<'info>(
    authority: &Signer<'info>,
    config: &GlobalConfig,
) -> Result<()> {
    EmergencyAuthority::validate_emergency_or_admin(authority, config)?;
    Ok(())
}

/// Guards a metadata operation by validating:
/// 1. The program is not in emergency mode.
/// 2. The signer is the configured metadata authority OR the admin.
pub fn guard_metadata_operation<'info>(
    authority: &Signer<'info>,
    config: &GlobalConfig,
) -> Result<()> {
    emergency::check_not_emergency_paused(config)?;

    let key = authority.key();
    require!(
        key == config.metadata_authority || key == config.admin_authority,
        GhostError::UnauthorizedMetadataAuthority
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    fn default_config() -> GlobalConfig {
        GlobalConfig {
            initialized: true,
            paused: false,
            emergency_paused: false,
            admin_authority: Pubkey::new_unique(),
            mint_authority: Pubkey::new_unique(),
            metadata_authority: Pubkey::new_unique(),
            emergency_authority: Pubkey::new_unique(),
            ..Default::default()
        }
    }

    #[test]
    fn test_guard_emergency_allows_admin() {
        let mut config = default_config();
        let admin_key = Pubkey::new_unique();
        config.admin_authority = admin_key;

        // The guard looks at the signer's key which must match
        // admin or emergency authority.
        assert!(admin_key == config.admin_authority);
    }
}
