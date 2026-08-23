#[cfg(test)]
pub mod test_fixtures {
    /// Test fixtures for the GHOST token program.
    ///
    /// Provides pre-built account states, keypairs, and configuration
    /// objects used across multiple test modules.
    /// These fixtures ensure consistency and reduce boilerplate in tests.
    use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

    // =========================================================================
    // Constants
    // =========================================================================

    pub const GHOST_PROGRAM_ID: Pubkey =
        solana_sdk::pubkey!("Gh0st111111111111111111111111111111111111111");

    pub const TOKEN_NAME: &str = "GHOST";
    pub const TOKEN_SYMBOL: &str = "GHOST";
    pub const TOKEN_DECIMALS: u8 = 9;
    pub const INITIAL_SUPPLY_RAW: u64 = 10_000_000_000_000_000_000; // 10B * 10^9
    pub const INITIAL_SUPPLY_HUMAN: u64 = 10_000_000_000; // 10 billion

    pub const CONFIG_SEED: &[u8] = b"config";
    pub const MINT_SEED: &[u8] = b"ghost-mint";
    pub const METADATA_SEED: &[u8] = b"metadata";
    pub const AUTHORITY_SEED: &[u8] = b"authority";

    pub const MAX_NAME_LENGTH: usize = 32;
    pub const MAX_SYMBOL_LENGTH: usize = 10;
    pub const MAX_URI_LENGTH: usize = 200;
    pub const AUTHORITY_ROTATION_DELAY: u64 = 100; // slots

    // =========================================================================
    // PDA Derivation Helpers
    // =========================================================================

    /// Derives the config PDA for the GHOST token program.
    pub fn derive_config_pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[CONFIG_SEED], &GHOST_PROGRAM_ID)
    }

    /// Derives the mint PDA for the GHOST token.
    pub fn derive_mint_pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[MINT_SEED], &GHOST_PROGRAM_ID)
    }

    /// Derives the metadata PDA.
    pub fn derive_metadata_pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[METADATA_SEED], &GHOST_PROGRAM_ID)
    }

    /// Derives the authority PDA.
    pub fn derive_authority_pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[AUTHORITY_SEED], &GHOST_PROGRAM_ID)
    }

    // =========================================================================
    // Keypair Generation
    // =========================================================================

    /// Creates a new random mint keypair (not a PDA - for test purposes).
    pub fn create_test_mint() -> Pubkey {
        Keypair::new().pubkey()
    }

    /// Creates a new random token account pubkey (not a PDA - for test purposes).
    pub fn create_test_token_account(_mint: &Pubkey, _owner: &Pubkey) -> Pubkey {
        // In a real test with program_test, this would derive from
        // the associated token account program. For standalone use,
        // we return a deterministic but unique pubkey.
        let mut seed = [0u8; 32];
        seed[..8].copy_from_slice(b"ghost_ata");

        // Mix in mint and owner for uniqueness
        let mint_bytes = _mint.to_bytes();
        let owner_bytes = _owner.to_bytes();
        for i in 0..8 {
            seed[8 + i] = mint_bytes[i] ^ owner_bytes[i];
        }

        Pubkey::new_from_array(seed)
    }

    /// Creates a new random config keypair.
    pub fn create_test_config_keypair() -> Keypair {
        Keypair::new()
    }

    // =========================================================================
    // Global Config
    // =========================================================================

    /// Represents the on-chain global configuration account for GHOST token.
    #[derive(Debug, Clone, PartialEq)]
    pub struct GlobalConfig {
        pub authority: Pubkey,
        pub mint: Pubkey,
        pub decimals: u8,
        pub total_supply: u64,
        pub is_initialized: bool,
        pub is_paused: bool,
        pub version: u8,
        pub pending_authority: Option<Pubkey>,
        pub rotation_proposed_at: Option<u64>,
        pub bump: u8,
    }

    impl GlobalConfig {
        /// Creates a new uninitialized config.
        pub fn new() -> Self {
            Self {
                authority: Pubkey::default(),
                mint: Pubkey::default(),
                decimals: 0,
                total_supply: 0,
                is_initialized: false,
                is_paused: false,
                version: 0,
                pending_authority: None,
                rotation_proposed_at: None,
                bump: 0,
            }
        }
    }

    impl Default for GlobalConfig {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Creates the standard GHOST token configuration as it would exist
    /// on-chain after a successful initialization.
    pub fn create_test_config() -> GlobalConfig {
        let (config_pda, config_bump) = derive_config_pda();
        let (mint_pda, _) = derive_mint_pda();
        let authority = Pubkey::new_unique();

        GlobalConfig {
            authority,
            mint: mint_pda,
            decimals: TOKEN_DECIMALS,
            total_supply: INITIAL_SUPPLY_RAW,
            is_initialized: true,
            is_paused: false,
            version: 1,
            pending_authority: None,
            rotation_proposed_at: None,
            bump: config_bump,
        }
    }

    // =========================================================================
    // Authority Sets
    // =========================================================================

    /// Returns a set of default authority keypairs for testing.
    ///
    /// Returns: (owner, new_authority, pending_authority, unauthorized)
    pub fn default_authorities() -> (Keypair, Keypair, Keypair, Keypair) {
        let owner = Keypair::new();
        let new_authority = Keypair::new();
        let pending_authority = Keypair::new();
        let unauthorized = Keypair::new();

        // Ensure all are unique
        assert_ne!(owner.pubkey(), new_authority.pubkey());
        assert_ne!(owner.pubkey(), pending_authority.pubkey());
        assert_ne!(owner.pubkey(), unauthorized.pubkey());
        assert_ne!(new_authority.pubkey(), pending_authority.pubkey());
        assert_ne!(new_authority.pubkey(), unauthorized.pubkey());
        assert_ne!(pending_authority.pubkey(), unauthorized.pubkey());

        (owner, new_authority, pending_authority, unauthorized)
    }

    /// Returns a set of authority pubkeys (without keypairs) for tests
    /// that don't need signing.
    pub fn default_authority_pubkeys() -> (Pubkey, Pubkey, Pubkey, Pubkey) {
        let (owner, new_auth, pending, unauth) = default_authorities();
        (
            owner.pubkey(),
            new_auth.pubkey(),
            pending.pubkey(),
            unauth.pubkey(),
        )
    }

    // =========================================================================
    // Metadata Fixtures
    // =========================================================================

    /// Standard valid metadata values.
    pub fn valid_metadata() -> (&'static str, &'static str, &'static str) {
        (
            TOKEN_NAME,
            TOKEN_SYMBOL,
            "https://ghost.token/metadata.json",
        )
    }

    /// Metadata values at maximum length.
    pub fn max_length_metadata() -> (String, String, String) {
        (
            "G".repeat(MAX_NAME_LENGTH),
            "H".repeat(MAX_SYMBOL_LENGTH),
            format!("https://ghost.token/{}", "x".repeat(MAX_URI_LENGTH - 20)),
        )
    }

    /// Metadata values exceeding maximum lengths (for negative tests).
    pub fn oversized_metadata() -> (String, String, String) {
        (
            "X".repeat(MAX_NAME_LENGTH + 1),
            "Y".repeat(MAX_SYMBOL_LENGTH + 1),
            format!("https://ghost.token/{}", "z".repeat(MAX_URI_LENGTH + 1)),
        )
    }

    // =========================================================================
    // Test Amounts
    // =========================================================================

    /// Common token amounts for testing (all in raw units).
    pub mod amounts {
        use super::INITIAL_SUPPLY_RAW;

        pub const ZERO: u64 = 0;
        pub const ONE_TOKEN: u64 = 1_000_000_000; // 1 GHOST (9 decimals)
        pub const TEN_TOKENS: u64 = 10_000_000_000; // 10 GHOST
        pub const HUNDRED_TOKENS: u64 = 100_000_000_000; // 100 GHOST
        pub const THOUSAND_TOKENS: u64 = 1_000_000_000_000; // 1,000 GHOST
        pub const MILLION_TOKENS: u64 = 1_000_000_000_000_000; // 1M GHOST
        pub const BILLION_TOKENS: u64 = INITIAL_SUPPLY_RAW / 10; // 1B GHOST
        pub const TEN_BILLION_TOKENS: u64 = INITIAL_SUPPLY_RAW; // 10B GHOST
        pub const MAX_POSSIBLE: u64 = u64::MAX;
    }

    // =========================================================================
    // Validation Helpers
    // =========================================================================

    /// Validates that a GlobalConfig matches the expected post-initialization state.
    pub fn assert_valid_initialized_config(config: &GlobalConfig, expected_authority: &Pubkey) {
        assert!(config.is_initialized, "Config should be initialized");
        assert!(!config.is_paused, "Config should not be paused after init");
        assert_eq!(
            config.decimals, TOKEN_DECIMALS,
            "Decimals should be {}",
            TOKEN_DECIMALS
        );
        assert_eq!(
            config.total_supply, INITIAL_SUPPLY_RAW,
            "Total supply should be {} raw units",
            INITIAL_SUPPLY_RAW
        );
        assert_eq!(
            config.authority, *expected_authority,
            "Authority should match the initialization signer"
        );
        assert_eq!(config.version, 1, "Version should be 1 after init");
        assert!(
            config.pending_authority.is_none(),
            "No pending authority should exist after init"
        );
    }

    /// Generates a deterministic but unique Pubkey from a seed string.
    pub fn pubkey_from_seed(seed: &str) -> Pubkey {
        let mut bytes = [0u8; 32];
        let seed_bytes = seed.as_bytes();
        let copy_len = seed_bytes.len().min(32);
        bytes[..copy_len].copy_from_slice(&seed_bytes[..copy_len]);
        Pubkey::new_from_array(bytes)
    }

    #[cfg(test)]
    mod fixture_tests {
        use super::*;

        #[test]
        fn test_fixtures_are_consistent() {
            // Verify raw supply math
            let human_supply = INITIAL_SUPPLY_RAW / 10u64.pow(TOKEN_DECIMALS as u32);
            assert_eq!(
                human_supply, INITIAL_SUPPLY_HUMAN,
                "Human-readable supply should be 10 billion GHOST"
            );

            // Verify PDA derivation is deterministic
            let (pda1, bump1) = derive_config_pda();
            let (pda2, bump2) = derive_config_pda();
            assert_eq!(pda1, pda2, "Config PDA must be deterministic");
            assert_eq!(bump1, bump2, "Config PDA bump must be deterministic");

            // Verify all PDAs are different
            assert_ne!(
                derive_config_pda().0,
                derive_mint_pda().0,
                "Config and mint PDAs must differ"
            );
            assert_ne!(
                derive_mint_pda().0,
                derive_metadata_pda().0,
                "Mint and metadata PDAs must differ"
            );

            // Verify default authorities are unique
            let (a, b, c, d) = default_authority_pubkeys();
            assert_ne!(a, b);
            assert_ne!(a, c);
            assert_ne!(a, d);
            assert_ne!(b, c);
            assert_ne!(b, d);
            assert_ne!(c, d);
        }

        #[test]
        fn test_config_has_correct_defaults() {
            let config = create_test_config();
            assert_valid_initialized_config(&config, &config.authority);
        }

        #[test]
        fn test_metadata_lengths() {
            let (name, symbol, uri) = valid_metadata();
            assert!(name.len() <= MAX_NAME_LENGTH);
            assert!(symbol.len() <= MAX_SYMBOL_LENGTH);
            assert!(uri.len() <= MAX_URI_LENGTH);

            let (max_name, max_symbol, max_uri) = max_length_metadata();
            assert_eq!(max_name.len(), MAX_NAME_LENGTH);
            assert_eq!(max_symbol.len(), MAX_SYMBOL_LENGTH);
            assert!(max_uri.len() <= MAX_URI_LENGTH);

            let (over_name, over_symbol, over_uri) = oversized_metadata();
            assert!(over_name.len() > MAX_NAME_LENGTH);
            assert!(over_symbol.len() > MAX_SYMBOL_LENGTH);
            assert!(over_uri.len() > MAX_URI_LENGTH);
        }

        #[test]
        fn test_amounts_invariants() {
            use amounts::*;
            assert_eq!(ZERO, 0);
            assert!(ONE_TOKEN > 0);
            assert!(TEN_TOKENS > ONE_TOKEN);
            assert!(HUNDRED_TOKENS > TEN_TOKENS);
            assert_eq!(TEN_BILLION_TOKENS, INITIAL_SUPPLY_RAW);
        }
    }
}
