use crate::errors::GhostError;
use crate::state::GlobalConfig;
use anchor_lang::prelude::*;

/// Authority responsible for minting new GHOST tokens.
pub struct MintAuthority;

impl MintAuthority {
    /// Checks whether the given pubkey matches the configured mint authority.
    pub fn is_authorized(authority: &Pubkey, config_mint_authority: &Pubkey) -> bool {
        authority == config_mint_authority
    }

    /// Validates that the signer is the current mint authority in the global config.
    /// Returns an error if the signer's key does not match the configured mint authority.
    pub fn validate_mint_authority<'info>(
        authority: &Signer<'info>,
        config: &GlobalConfig,
    ) -> Result<()> {
        require!(
            Self::is_authorized(&authority.key(), &config.mint_authority),
            GhostError::UnauthorizedMintAuthority
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_is_authorized_match() {
        let key = Pubkey::new_unique();
        assert!(MintAuthority::is_authorized(&key, &key));
    }

    #[test]
    fn test_is_authorized_mismatch() {
        let key1 = Pubkey::new_unique();
        let key2 = Pubkey::new_unique();
        assert!(!MintAuthority::is_authorized(&key1, &key2));
    }
}
