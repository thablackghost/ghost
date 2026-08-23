/// Calculates the total account size for the GlobalConfig account.
///
/// Layout (all fields byte-aligned):
/// - discriminator:     8 bytes
/// - bump:              1 byte
/// - version:           1 byte
/// - decimals:          1 byte
/// - initialized:       1 byte
/// - paused:            1 byte
/// - emergency_paused:  1 byte
/// - mint:             32 bytes
/// - mint_authority:   32 bytes
/// - update_authority: 32 bytes
/// - emergency_authority: 32 bytes
/// - metadata_state:   32 bytes
/// - paused_at:         8 bytes
/// - created_at:        8 bytes
/// - total_minted:      8 bytes
/// - total_burned:      8 bytes
/// - mint_count:        8 bytes
/// - burn_count:        8 bytes
/// - reserved:         96 bytes
/// Total: 8 + 6 + (6 * 32) + 5*8 + 96 = 8 + 6 + 192 + 40 + 96 = 342
pub fn calculate_global_config_size() -> usize {
    // discriminator + all fields
    8 + 1 + 1 + 1 + 1 + 1 + 1 + 32 + 32 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 96
}

/// Calculates the total account size for the MintConfig account.
///
/// Layout:
/// - discriminator:     8 bytes
/// - bump:              1 byte
/// - initialized:       1 byte
/// - reserved:          6 bytes
/// - mint:             32 bytes
/// - authority:        32 bytes
/// - max_supply:        9 bytes (1 option tag + 8 value)
/// - created_at:        8 bytes
/// - reserved2:        64 bytes
/// Total: 8 + 8 + 64 + 9 + 8 + 64 = 161
pub fn calculate_mint_config_size() -> usize {
    8 + 1 + 1 + 6 + 32 + 32 + 9 + 8 + 64
}

/// Calculates the total account size for the AuthorityState account.
///
/// Layout:
/// - discriminator:          8 bytes
/// - bump:                   1 byte
/// - initialized:            1 byte
/// - rotation_count:         1 byte
/// - reserved:               5 bytes
/// - current_authority:     32 bytes
/// - pending_authority:     32 bytes
/// - original_authority:    32 bytes
/// - last_rotated_at:        8 bytes
/// - transfer_window_start:  8 bytes
/// - reserved2:             64 bytes
/// Total: 8 + 8 + 96 + 16 + 64 = 192
pub fn calculate_authority_state_size() -> usize {
    8 + 1 + 1 + 1 + 5 + 32 + 32 + 32 + 8 + 8 + 64
}

/// Calculates the total account size for the MetadataState account.
///
/// Layout:
/// - discriminator:      8 bytes
/// - bump:               1 byte
/// - initialized:        1 byte
/// - frozen:             1 byte
/// - reserved:           5 bytes
/// - name:        4 + 32 bytes (length prefix + data)
/// - symbol:      4 + 10 bytes
/// - uri:         4 + 200 bytes
/// - description: 4 + 500 bytes
/// - update_authority:  32 bytes
/// - created_at:         8 bytes
/// - updated_at:         8 bytes
/// Total: 8 + 8 + 766 + 32 + 16 = 830
pub fn calculate_metadata_state_size() -> usize {
    8 + 1 + 1 + 1 + 5 + (4 + 32) + (4 + 10) + (4 + 200) + (4 + 500) + 32 + 8 + 8
}

/// Calculates the total account size for the PauseState account.
///
/// Layout:
/// - discriminator:     8 bytes
/// - bump:              1 byte
/// - paused:            1 byte
/// - emergency_paused:  1 byte
/// - reserved:          5 bytes
/// - paused_by:        32 bytes
/// - paused_at:         8 bytes
/// - unpaused_at:       8 bytes
/// - reserved2:        64 bytes
/// Total: 8 + 8 + 32 + 16 + 64 = 128
pub fn calculate_pause_state_size() -> usize {
    8 + 1 + 1 + 1 + 5 + 32 + 8 + 8 + 64
}

/// Calculates the total account size for the VersionState account.
///
/// Layout:
/// - discriminator:  8 bytes
/// - bump:           1 byte
/// - initialized:    1 byte
/// - reserved:       6 bytes
/// - major:          1 byte
/// - minor:          1 byte
/// - patch:          2 bytes
/// - reserved2:     64 bytes
/// Total: 8 + 8 + 4 + 64 = 84
pub fn calculate_version_state_size() -> usize {
    8 + 1 + 1 + 6 + 1 + 1 + 2 + 64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_config_size() {
        assert_eq!(calculate_global_config_size(), 342);
    }

    #[test]
    fn test_mint_config_size() {
        assert_eq!(calculate_mint_config_size(), 161);
    }

    #[test]
    fn test_authority_state_size() {
        assert_eq!(calculate_authority_state_size(), 192);
    }

    #[test]
    fn test_metadata_state_size() {
        assert_eq!(calculate_metadata_state_size(), 830);
    }

    #[test]
    fn test_pause_state_size() {
        assert_eq!(calculate_pause_state_size(), 128);
    }

    #[test]
    fn test_version_state_size() {
        assert_eq!(calculate_version_state_size(), 84);
    }

    #[test]
    fn test_all_sizes_nonzero() {
        assert!(calculate_global_config_size() > 0);
        assert!(calculate_mint_config_size() > 0);
        assert!(calculate_authority_state_size() > 0);
        assert!(calculate_metadata_state_size() > 0);
        assert!(calculate_pause_state_size() > 0);
        assert!(calculate_version_state_size() > 0);
    }
}
