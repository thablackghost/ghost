use crate::errors::GhostError;
use crate::state::GlobalConfig;
use anchor_lang::prelude::*;

/// Checks that the program is NOT in emergency pause mode.
/// Emergency pause is a severe state that halts all operations including
/// admin operations, to prevent further damage during a critical incident.
pub fn check_not_emergency_paused(config: &GlobalConfig) -> Result<()> {
    require!(!config.emergency_paused, GhostError::ProgramEmergencyPaused);
    Ok(())
}

/// Checks that the program is NOT in regular pause mode.
/// Regular pause halts user-facing operations (transfers, minting, burning)
/// but may still allow admin operations.
pub fn check_not_paused(config: &GlobalConfig) -> Result<()> {
    require!(!config.paused, GhostError::ProgramPaused);
    Ok(())
}

/// Returns `true` if the program is fully operational:
/// - Not paused
/// - Not in emergency pause
/// - Has been initialized
pub fn is_program_active(config: &GlobalConfig) -> bool {
    config.initialized && !config.paused && !config.emergency_paused
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    fn default_config() -> GlobalConfig {
        GlobalConfig {
            is_initialized: true,
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
    fn test_check_not_emergency_paused_passes() {
        let config = default_config();
        assert!(check_not_emergency_paused(&config).is_ok());
    }

    #[test]
    fn test_check_not_emergency_paused_fails() {
        let mut config = default_config();
        config.emergency_paused = true;
        assert!(check_not_emergency_paused(&config).is_err());
    }

    #[test]
    fn test_check_not_paused_passes() {
        let config = default_config();
        assert!(check_not_paused(&config).is_ok());
    }

    #[test]
    fn test_check_not_paused_fails() {
        let mut config = default_config();
        config.paused = true;
        assert!(check_not_paused(&config).is_err());
    }

    #[test]
    fn test_is_program_active() {
        let mut config = default_config();
        assert!(is_program_active(&config));

        config.paused = true;
        assert!(!is_program_active(&config));

        config.paused = false;
        config.emergency_paused = true;
        assert!(!is_program_active(&config));

        config.emergency_paused = false;
        config.initialized = false;
        assert!(!is_program_active(&config));
    }
}
