use solana_program::hash::{hash, Hash};

/// Hashes the token name and symbol together using SHA-256 (via solana_program).
///
/// Produces a 32-byte hash digest useful for verifying metadata integrity.
/// The input is `name:symbol` concatenated with a colon separator.
pub fn hash_metadata(name: &str, symbol: &str) -> [u8; 32] {
    let mut data = Vec::with_capacity(name.len() + 1 + symbol.len());
    data.extend_from_slice(name.as_bytes());
    data.push(b':');
    data.extend_from_slice(symbol.as_bytes());
    hash(&data).to_bytes()
}

/// Hashes an authority public key using SHA-256 (via solana_program).
///
/// Produces a 32-byte hash that can be used as a commitment or
/// integrity check for authority state transitions.
pub fn hash_authority_state(authority: &solana_program::pubkey::Pubkey) -> [u8; 32] {
    hash(&authority.to_bytes()).to_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_hash_metadata_deterministic() {
        let h1 = hash_metadata("GHOST", "GHOST");
        let h2 = hash_metadata("GHOST", "GHOST");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_metadata_different() {
        let h1 = hash_metadata("GHOST", "GHOST");
        let h2 = hash_metadata("GHOST", "GST");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hash_metadata_not_empty() {
        let h = hash_metadata("GHOST", "GHOST");
        assert_ne!(h, [0u8; 32]);
    }

    #[test]
    fn test_hash_authority_deterministic() {
        let pk = Pubkey::new_unique();
        let h1 = hash_authority_state(&pk);
        let h2 = hash_authority_state(&pk);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_authority_different() {
        let h1 = hash_authority_state(&Pubkey::new_unique());
        let h2 = hash_authority_state(&Pubkey::new_unique());
        assert_ne!(h1, h2);
    }
}
