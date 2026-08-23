use anchor_lang::prelude::*;

use crate::errors::GhostError;
use crate::state::MetadataState;

/// INVARIANT_6: The metadata update authority must be explicitly controlled.
/// Only the configured update_authority can modify metadata.
pub const INVARIANT_6_MSG: &str = "INVARIANT_6: Metadata authority must be explicitly controlled";

/// INVARIANT_7: Frozen metadata cannot be modified under any circumstances.
/// Once frozen, no update operation is allowed.
pub const INVARIANT_7_MSG: &str = "INVARIANT_7: Frozen metadata cannot be modified";

/// Checks all metadata invariants for a given operation.
///
/// # Arguments
/// * `metadata` - The current metadata state.
/// * `operation` - A label describing the operation (for logging purposes).
pub fn check_metadata_invariants(metadata: &MetadataState, operation: &str) -> Result<()> {
    msg!("Checking metadata invariants for operation: {}", operation);

    // INVARIANT_6: Update authority must be set
    if metadata.update_authority == Pubkey::default() {
        msg!("{}", INVARIANT_6_MSG);
        return Err(GhostError::InvalidAuthority.into());
    }

    // INVARIANT_7: If frozen, no modifications allowed
    // This is a pre-emptive check; the update functions themselves also check this.
    // Having it here provides defense-in-depth.
    if metadata.frozen && operation != "read" && operation != "freeze" {
        msg!("{}", INVARIANT_7_MSG);
        return Err(GhostError::MetadataFrozen.into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    fn make_metadata() -> MetadataState {
        MetadataState {
            initialized: true,
            frozen: false,
            name: "GHOST".to_string(),
            symbol: "GHOST".to_string(),
            uri: "https://ghost.io".to_string(),
            description: "A meme token.".to_string(),
            update_authority: Pubkey::new_unique(),
            ..Default::default()
        }
    }

    #[test]
    fn test_metadata_invariants_ok() {
        let m = make_metadata();
        assert!(check_metadata_invariants(&m, "update").is_ok());
    }

    #[test]
    fn test_metadata_invariants_default_authority() {
        let mut m = make_metadata();
        m.update_authority = Pubkey::default();
        assert!(check_metadata_invariants(&m, "update").is_err());
    }

    #[test]
    fn test_metadata_invariants_frozen() {
        let mut m = make_metadata();
        m.frozen = true;
        assert!(check_metadata_invariants(&m, "update").is_err());
        // Read operation should still pass
        assert!(check_metadata_invariants(&m, "read").is_ok());
        // freeze operation should also still pass (it's what sets the flag)
        assert!(check_metadata_invariants(&m, "freeze").is_ok());
    }
}
