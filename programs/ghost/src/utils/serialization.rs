use anchor_lang::prelude::*;

use crate::errors::GhostError;
use crate::state::GlobalConfig;

/// Serializes a GlobalConfig into a byte vector using Borsh encoding.
pub fn serialize_config(config: &GlobalConfig) -> Result<Vec<u8>> {
    let mut writer = Vec::with_capacity(GlobalConfig::LEN);
    config
        .serialize(&mut writer)
        .map_err(|_| GhostError::InvalidConfiguration)?;
    Ok(writer)
}

/// Deserializes a byte slice into a GlobalConfig using Borsh decoding.
pub fn deserialize_config(data: &[u8]) -> Result<GlobalConfig> {
    GlobalConfig::try_from_slice(data).map_err(|_| GhostError::InvalidConfiguration.into())
}

/// Serializes any Borsh-serializable type into a byte vector.
pub fn serialize_borsh<T: BorshSerialize>(value: &T) -> Result<Vec<u8>> {
    let mut writer = Vec::new();
    value
        .serialize(&mut writer)
        .map_err(|_| GhostError::InvalidConfiguration)?;
    Ok(writer)
}

/// Deserializes a byte slice into a Borsh-deserializable type.
pub fn deserialize_borsh<T: BorshDeserialize>(data: &[u8]) -> Result<T> {
    T::try_from_slice(data).map_err(|_| GhostError::InvalidConfiguration.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_roundtrip_config() {
        let config = GlobalConfig {
            version: 1,
            decimals: 9,
            initialized: true,
            paused: false,
            emergency_paused: false,
            mint: Pubkey::new_unique(),
            mint_authority: Pubkey::new_unique(),
            update_authority: Pubkey::new_unique(),
            emergency_authority: Pubkey::new_unique(),
            metadata_state: Pubkey::new_unique(),
            total_minted: 10_000_000_000_000_000_000,
            total_burned: 0,
            mint_count: 1,
            burn_count: 0,
            created_at: 1234567890,
            paused_at: 0,
            ..Default::default()
        };

        let bytes = serialize_config(&config).unwrap();
        let decoded = deserialize_config(&bytes).unwrap();

        assert_eq!(config.version, decoded.version);
        assert_eq!(config.decimals, decoded.decimals);
        assert_eq!(config.initialized, decoded.initialized);
        assert_eq!(config.mint, decoded.mint);
        assert_eq!(config.total_minted, decoded.total_minted);
    }

    #[test]
    fn test_deserialize_garbage_fails() {
        let garbage = vec![0u8; 10];
        assert!(deserialize_config(&garbage).is_err());
    }

    #[test]
    fn test_serialize_empty_slice_fails() {
        let empty: &[u8] = &[];
        assert!(deserialize_config(empty).is_err());
    }
}
