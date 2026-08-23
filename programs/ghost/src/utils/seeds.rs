use anchor_lang::prelude::*;

use crate::state::seeds;

/// Derives the PDA for the global configuration account.
///
/// Seed: b"config"
pub fn get_config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[seeds::CONFIG_SEED], &crate::id())
}

/// Derives the PDA for the authority state account.
///
/// Seed: b"authority"
pub fn get_authority_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[seeds::AUTHORITY_SEED], &crate::id())
}

/// Derives the PDA for the metadata state account.
///
/// Seed: b"metadata"
pub fn get_metadata_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[seeds::METADATA_SEED], &crate::id())
}

/// Derives the PDA for the pause state account.
///
/// Seed: b"pause"
pub fn get_pause_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[seeds::PAUSE_SEED], &crate::id())
}

/// Derives the PDA for the version state account.
///
/// Seed: b"version"
pub fn get_version_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[seeds::VERSION_SEED], &crate::id())
}

/// Derives the PDA for the mint config account.
///
/// Seed: b"mint-config"
pub fn get_mint_config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[seeds::MINT_CONFIG_SEED], &crate::id())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pda_derivation_is_deterministic() {
        let (pda1, bump1) = get_config_pda();
        let (pda2, bump2) = get_config_pda();
        assert_eq!(pda1, pda2);
        assert_eq!(bump1, bump2);
    }

    #[test]
    fn test_all_pdas_are_unique() {
        let pdas = vec![
            get_config_pda().0,
            get_authority_pda().0,
            get_metadata_pda().0,
            get_pause_pda().0,
            get_version_pda().0,
            get_mint_config_pda().0,
        ];

        for i in 0..pdas.len() {
            for j in (i + 1)..pdas.len() {
                assert_ne!(pdas[i], pdas[j], "PDA {} and {} should differ", i, j);
            }
        }
    }

    #[test]
    fn test_bumps_in_valid_range() {
        let bumps = vec![
            get_config_pda().1,
            get_authority_pda().1,
            get_metadata_pda().1,
            get_pause_pda().1,
            get_version_pda().1,
            get_mint_config_pda().1,
        ];

        for bump in bumps {
            assert!((bump as u32) < 256, "Bump must be valid u8");
        }
    }
}
