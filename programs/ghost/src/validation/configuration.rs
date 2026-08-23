use anchor_lang::prelude::*;

use crate::errors::GhostError;
use crate::math::decimals::validate_decimals;
use crate::state::GlobalConfig;
use crate::validation::authorities::validate_authority_not_default;

/// Validates that the global configuration is internally consistent.
///
/// Checks:
/// - Decimals are within range
/// - All authority pubkeys are not default
/// - Mint pubkey is not default
pub fn validate_config_consistency(config: &GlobalConfig) -> Result<()> {
    if !config.initialized {
        return Err(GhostError::NotInitialized.into());
    }
    validate_decimals_config(config.decimals)?;
    validate_authorities_configured(config)?;

    if config.mint == Pubkey::default() {
        return Err(GhostError::InvalidMint.into());
    }

    Ok(())
}

/// Validates that decimals are within the allowed range (0-18).
pub fn validate_decimals_config(decimals: u8) -> Result<()> {
    validate_decimals(decimals)
}

/// Validates that all required authorities in the config are set (non-default).
pub fn validate_authorities_configured(config: &GlobalConfig) -> Result<()> {
    validate_authority_not_default(&config.mint_authority)?;
    validate_authority_not_default(&config.update_authority)?;
    validate_authority_not_default(&config.emergency_authority)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    fn make_config() -> GlobalConfig {
        GlobalConfig {
            initialized: true,
            decimals: 9,
            mint: Pubkey::new_unique(),
            mint_authority: Pubkey::new_unique(),
            update_authority: Pubkey::new_unique(),
            emergency_authority: Pubkey::new_unique(),
            ..Default::default()
        }
    }

    #[test]
    fn test_config_consistency_ok() {
        assert!(validate_config_consistency(&make_config()).is_ok());
    }

    #[test]
    fn test_config_not_initialized() {
        let mut c = make_config();
        c.initialized = false;
        assert!(validate_config_consistency(&c).is_err());
    }

    #[test]
    fn test_config_bad_decimals() {
        let mut c = make_config();
        c.decimals = 19;
        assert!(validate_config_consistency(&c).is_err());
    }

    #[test]
    fn test_config_default_mint() {
        let mut c = make_config();
        c.mint = Pubkey::default();
        assert!(validate_config_consistency(&c).is_err());
    }
}
