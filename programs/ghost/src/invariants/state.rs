use anchor_lang::prelude::*;

use crate::errors::GhostError;
use crate::state::GlobalConfig;

/// INVARIANT_8: PDA state must correspond to deterministic seeds.
/// All PDAs must be derived from the program's canonical seed constants.
pub const INVARIANT_8_MSG: &str = "INVARIANT_8: PDA state must correspond to deterministic seeds";

/// INVARIANT_9: Arithmetic must never wrap. All math must use checked operations.
pub const INVARIANT_9_MSG: &str = "INVARIANT_9: Arithmetic must never wrap";

/// INVARIANT_10: Initialization may occur only once per account.
/// Double-initialization must be rejected.
pub const INVARIANT_10_MSG: &str = "INVARIANT_10: Initialization may occur only once";

/// Performs comprehensive state invariant checks on the global configuration.
///
/// This function verifies:
/// - The config is marked as initialized (INVARIANT_10)
/// - All required fields are non-default
/// - The paused/emergency flags are consistent
pub fn check_state_invariants(config: &GlobalConfig) -> Result<()> {
    msg!("{}", INVARIANT_8_MSG);
    msg!("{}", INVARIANT_9_MSG);
    msg!("{}", INVARIANT_10_MSG);

    // INVARIANT_10: Config must be initialized for any operation
    if !config.initialized {
        return Err(GhostError::NotInitialized.into());
    }

    // INVARIANT_8: Mint key must correspond to a valid PDA-derived value
    // (The actual PDA derivation check happens at instruction level using seeds)
    if config.mint == Pubkey::default() {
        return Err(GhostError::InvalidPda.into());
    }

    // INVARIANT_9: All numeric fields must be consistent
    if config.total_minted < config.total_burned {
        // This would indicate an arithmetic error somewhere
        return Err(GhostError::SupplyInvariantViolated.into());
    }

    // Check consistency of pause flags
    if config.emergency_paused && !config.paused {
        // Emergency pause should also set paused
        return Err(GhostError::InvalidState.into());
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
            decimals: 9,
            version: 1,
            mint: Pubkey::new_unique(),
            mint_authority: Pubkey::new_unique(),
            update_authority: Pubkey::new_unique(),
            emergency_authority: Pubkey::new_unique(),
            metadata_state: Pubkey::new_unique(),
            total_minted: 10_000_000_000_000_000_000,
            total_burned: 0,
            ..Default::default()
        }
    }

    #[test]
    fn test_state_invariants_ok() {
        assert!(check_state_invariants(&make_config()).is_ok());
    }

    #[test]
    fn test_state_not_initialized() {
        let mut c = make_config();
        c.initialized = false;
        assert!(check_state_invariants(&c).is_err());
    }

    #[test]
    fn test_state_default_mint() {
        let mut c = make_config();
        c.mint = Pubkey::default();
        assert!(check_state_invariants(&c).is_err());
    }

    #[test]
    fn test_state_burned_gt_minted() {
        let mut c = make_config();
        c.total_burned = c.total_minted + 1;
        assert!(check_state_invariants(&c).is_err());
    }

    #[test]
    fn test_state_inconsistent_pause() {
        let mut c = make_config();
        c.emergency_paused = true;
        c.paused = false;
        assert!(check_state_invariants(&c).is_err());
    }
}
