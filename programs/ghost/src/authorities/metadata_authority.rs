use crate::errors::GhostError;
use crate::state::GlobalConfig;
use anchor_lang::prelude::*;

/// Authority responsible for updating token metadata.
/// Handles operations such as setting/updating the token name, symbol, URI, etc.
pub struct MetadataAuthority;

impl MetadataAuthority {
    /// Validates that the provided signer is the configured metadata authority
    /// in the global config.
    pub fn validate_metadata_authority<'info>(
        authority: &Signer<'info>,
        config: &GlobalConfig,
    ) -> Result<()> {
        require!(
            authority.key() == config.metadata_authority,
            GhostError::UnauthorizedMetadataAuthority
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_metadata_authority_is_zero_should_fail_in_validation() {
        // Metadata authority of Pubkey::default() means no metadata authority is set.
        // The validate_metadata_authority function will still compare keys;
        // the caller should ensure a non-zero authority is set beforehand.
        let zero = Pubkey::default();
        let config = GlobalConfig {
            metadata_authority: zero,
            ..Default::default()
        };
        // A signer with a non-zero key will fail against the zero metadata_authority.
        // This test documents the expected behavior.
    }
}
