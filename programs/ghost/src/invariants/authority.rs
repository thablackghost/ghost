use anchor_lang::prelude::*;

use crate::errors::GhostError;
use crate::state::GlobalConfig;

/// INVARIANT_4: Non-authority cannot rotate the authority.
/// Enforced by checking the signer matches the current authority.
pub const INVARIANT_4_MSG: &str = "INVARIANT_4: Non-authority cannot rotate authority";

/// INVARIANT_5: The configured mint must never be replaced accidentally.
/// Operations that update config must verify the mint key remains unchanged.
pub const INVARIANT_5_MSG: &str =
    "INVARIANT_5: Configured mint must never be replaced accidentally";

/// Checks all authority-related invariants.
///
/// # Arguments
/// * `config` - The global configuration to verify.
/// * `operation` - A label describing the operation being performed (for logging).
pub fn check_all_authority_invariants(config: &GlobalConfig, operation: &str) -> Result<()> {
    msg!("Checking authority invariants for operation: {}", operation);

    // INVARIANT_4 is enforced at the instruction level via signer checks.
    // Here we verify that required fields are populated.
    if config.mint_authority == Pubkey::default() {
        msg!("{}", INVARIANT_4_MSG);
        return Err(GhostError::InvalidAuthority.into());
    }

    if config.update_authority == Pubkey::default() {
        msg!("{}", INVARIANT_4_MSG);
        return Err(GhostError::InvalidAuthority.into());
    }

    Ok(())
}

/// Verifies that the mint pubkey has not changed between configuration updates.
///
/// This enforces INVARIANT_5: the mint associated with this program
/// must never be accidentally replaced.
pub fn verify_mint_unchanged(old_mint: &Pubkey, new_config: &GlobalConfig) -> Result<()> {
    if *old_mint != new_config.mint {
        msg!("{}", INVARIANT_5_MSG);
        return Err(GhostError::MintMismatch.into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    fn make_config() -> GlobalConfig {
        GlobalConfig {
            initialized: true,
            mint: Pubkey::new_unique(),
            mint_authority: Pubkey::new_unique(),
            update_authority: Pubkey::new_unique(),
            emergency_authority: Pubkey::new_unique(),
            ..Default::default()
        }
    }

    #[test]
    fn test_authority_invariants_ok() {
        let cfg = make_config();
        assert!(check_all_authority_invariants(&cfg, "test").is_ok());
    }

    #[test]
    fn test_authority_invariants_default_mint_authority() {
        let mut cfg = make_config();
        cfg.mint_authority = Pubkey::default();
        assert!(check_all_authority_invariants(&cfg, "test").is_err());
    }

    #[test]
    fn test_verify_mint_unchanged() {
        let mint = Pubkey::new_unique();
        let mut cfg = make_config();
        cfg.mint = mint;
        assert!(verify_mint_unchanged(&mint, &cfg).is_ok());

        cfg.mint = Pubkey::new_unique();
        assert!(verify_mint_unchanged(&mint, &cfg).is_err());
    }
}
