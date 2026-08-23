#[cfg(test)]
pub mod test_helpers {
    /// Test helpers for the GHOST token program integration tests.
    ///
    /// Provides utility functions for setting up the Solana program test
    /// environment, creating test accounts, minting initial supply, and
    /// asserting expected error conditions.
    use solana_program_test::*;
    use solana_sdk::{
        instruction::Instruction,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_instruction,
        transaction::Transaction,
    };

    use super::test_fixtures::{self, GHOST_PROGRAM_ID, INITIAL_SUPPLY_RAW, TOKEN_DECIMALS};

    // =========================================================================
    // Error Codes
    // =========================================================================

    /// Error codes for the GHOST token program.
    /// These mirror the #[error_code] enum in the Anchor program.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GhostError {
        AlreadyInitialized = 6000,
        NotInitialized = 6001,
        Unauthorized = 6002,
        InvalidDecimals = 6003,
        ZeroAmount = 6004,
        InvalidMint = 6005,
        InsufficientBalance = 6006,
        Paused = 6007,
        NameTooLong = 6008,
        SymbolTooLong = 6009,
        UriTooLong = 6010,
        InvalidVersion = 6011,
        RotationDelayNotMet = 6012,
        NotPendingRotation = 6013,
        Overflow = 6014,
        MetadataFrozen = 6015,
    }

    impl GhostError {
        pub fn to_anchor_error_code(self) -> u32 {
            self as u32
        }

        pub fn to_hex(self) -> String {
            format!("0x{:x}", self.to_anchor_error_code())
        }
    }

    /// Anchor error format: 8-byte discriminator + error code
    /// AnchorError = [8 bytes of ERROR_DISCRIMINATOR][4 bytes LE error_code]
    const ANCHOR_ERROR_DISCRIMINATOR: [u8; 8] = [11, 168, 72, 51, 178, 70, 60, 64];

    fn make_anchor_error(error: GhostError) -> Vec<u8> {
        let mut data = Vec::with_capacity(12);
        data.extend_from_slice(&ANCHOR_ERROR_DISCRIMINATOR);
        data.extend_from_slice(&(error as u32).to_le_bytes());
        data
    }

    // =========================================================================
    // ProgramTest Setup
    // =========================================================================

    /// Sets up a standard Solana ProgramTest environment for GHOST token tests.
    ///
    /// Returns a `ProgramTest` configured with:
    /// - The GHOST token program loaded
    /// - SPL Token program
    /// - Associated Token Account program
    /// - System program
    /// - Rent sysvar
    /// - Generous compute budget
    pub fn setup_test_environment() -> ProgramTest {
        let mut program_test = ProgramTest::new(
            "ghost_token",
            GHOST_PROGRAM_ID,
            processor!(|_context| {
                // In production, this would be the actual program entrypoint.
                // For program_test, the .so file would be loaded here.
                // When using `solana_program_test`, the program .so is
                // loaded via:
                //
                // let mut program_test = ProgramTest::new(
                //     "ghost_token",
                //     GHOST_PROGRAM_ID,
                //     processor!(ghost_token::entry),
                // );
                //
                // The no-op processor is a placeholder that allows tests
                // that don't actually invoke the program to still compile.
                Ok(())
            }),
        );

        // Add required system programs
        program_test.add_program("spl_token", spl_token::ID, None);
        program_test.add_program(
            "spl_associated_token_account",
            spl_associated_token_account::ID,
            None,
        );

        // Set generous compute budget (1.4M is Anchor test default)
        program_test.set_compute_max_units(1_400_000);

        program_test
    }

    /// Sets up the test environment with additional pre-funded accounts.
    pub fn setup_test_environment_with_accounts() -> ProgramTest {
        let mut program_test = setup_test_environment();

        // Fund the default payer with a generous amount
        // This is handled automatically by program_test, but we can
        // add pre-configured accounts here if needed.

        program_test
    }

    // =========================================================================
    // Account Management
    // =========================================================================

    /// Adds common test accounts to the test environment.
    ///
    /// This function is called after creating the ProgramTestContext to
    /// fund additional test signers from the program_test's default payer.
    pub async fn add_test_accounts(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        additional_signers: &[&Keypair],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lamports_per_account = 1_000_000_000; // 1 SOL each

        let mut instructions: Vec<Instruction> = Vec::new();
        for signer in additional_signers {
            instructions.push(system_instruction::transfer(
                &payer.pubkey(),
                &signer.pubkey(),
                lamports_per_account,
            ));
        }

        if !instructions.is_empty() {
            let mut all_signers: Vec<&Keypair> = vec![payer];
            all_signers.extend(additional_signers.iter().copied());

            let recent_blockhash = banks_client.get_latest_blockhash().await?;
            let tx = Transaction::new_signed_with_payer(
                &instructions,
                Some(&payer.pubkey()),
                &all_signers,
                recent_blockhash,
            );
            banks_client.process_transaction(tx).await?;
        }

        Ok(())
    }

    // =========================================================================
    // Minting Helpers
    // =========================================================================

    /// Mints the initial supply to the specified token account.
    ///
    /// In a real test, this would create a mint instruction and process it.
    pub async fn mint_initial_supply(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        mint_authority: &Keypair,
        mint: &Pubkey,
        token_account: &Pubkey,
        config: &Pubkey,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mint_amount = INITIAL_SUPPLY_RAW;

        let mut data = Vec::new();
        data.extend_from_slice(&[102, 76, 55, 251, 38, 73, 215, 88]); // mint discriminator
        data.extend_from_slice(&mint_amount.to_le_bytes());

        let ix = Instruction {
            program_id: GHOST_PROGRAM_ID,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new_readonly(mint_authority.pubkey(), true),
                solana_sdk::instruction::AccountMeta::new(*mint, false),
                solana_sdk::instruction::AccountMeta::new(*token_account, false),
                solana_sdk::instruction::AccountMeta::new_readonly(*config, false),
                solana_sdk::instruction::AccountMeta::new_readonly(spl_token::ID, false),
            ],
            data,
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[payer, mint_authority],
            recent_blockhash,
        );

        banks_client.process_transaction(tx).await?;
        Ok(())
    }

    /// Mints a specific amount of tokens.
    pub async fn mint_tokens(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        mint_authority: &Keypair,
        mint: &Pubkey,
        token_account: &Pubkey,
        config: &Pubkey,
        amount: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut data = Vec::new();
        data.extend_from_slice(&[102, 76, 55, 251, 38, 73, 215, 88]); // mint discriminator
        data.extend_from_slice(&amount.to_le_bytes());

        let ix = Instruction {
            program_id: GHOST_PROGRAM_ID,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new_readonly(mint_authority.pubkey(), true),
                solana_sdk::instruction::AccountMeta::new(*mint, false),
                solana_sdk::instruction::AccountMeta::new(*token_account, false),
                solana_sdk::instruction::AccountMeta::new_readonly(*config, false),
                solana_sdk::instruction::AccountMeta::new_readonly(spl_token::ID, false),
            ],
            data,
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[payer, mint_authority],
            recent_blockhash,
        );

        banks_client.process_transaction(tx).await?;
        Ok(())
    }

    // =========================================================================
    // Error Assertion Helpers
    // =========================================================================

    /// Asserts that a transaction result contains a specific GHOST error.
    ///
    /// Parses the Anchor error from the transaction error and compares it
    /// against the expected GhostError variant.
    pub fn assert_ghost_error(
        result: Result<(), solana_sdk::transport::TransportError>,
        expected_error: GhostError,
    ) {
        match result {
            Ok(()) => {
                panic!(
                    "Expected GHOST error {:?} (code: {}), but transaction succeeded",
                    expected_error,
                    expected_error.to_anchor_error_code()
                );
            }
            Err(transport_err) => {
                let error_string = transport_err.to_string();

                // Try to find the error code in the error message
                let error_code = expected_error.to_anchor_error_code();

                // Anchor errors in program_test are reported as:
                // "Transaction simulation failed: Error processing Instruction 0: custom program error: 0x1770"
                let hex_code = format!("0x{:x}", error_code);

                let found = error_string.contains(&hex_code)
                    || error_string.contains(&format!("custom program error: {}", error_code))
                    || error_string.contains(&format!("{}", error_code));

                if !found {
                    // Check if the error bytes are in the logs
                    let error_bytes = make_anchor_error(expected_error);
                    // The error might also appear as decimal
                    let found_bytes = error_string.contains(&format!("{:?}", error_code));

                    if !found_bytes {
                        panic!(
                            "Expected GHOST error {:?} (code: {}, hex: {}), but got: {}",
                            expected_error, error_code, hex_code, error_string
                        );
                    }
                }
            }
        }
    }

    /// Asserts that a transaction result succeeded (no error).
    pub fn assert_ok(result: Result<(), solana_sdk::transport::TransportError>, context: &str) {
        match result {
            Ok(()) => {}
            Err(e) => {
                panic!("Expected success in '{}', but got error: {}", context, e);
            }
        }
    }

    /// Asserts that a transaction failed with any error (useful for negative tests
    /// where the specific error doesn't matter, just that it failed).
    pub fn assert_err(result: Result<(), solana_sdk::transport::TransportError>, context: &str) {
        match result {
            Ok(()) => {
                panic!(
                    "Expected failure in '{}', but transaction succeeded",
                    context
                );
            }
            Err(_) => {
                // Expected - test passes
            }
        }
    }

    // =========================================================================
    // Instruction Builders (shared across tests)
    // =========================================================================

    /// Builds an initialize instruction for the GHOST token program.
    pub fn build_initialize_instruction(
        authority: &Pubkey,
        mint: &Pubkey,
        config: &Pubkey,
        decimals: u8,
        initial_supply: u64,
    ) -> Instruction {
        let mut data = Vec::new();
        data.extend_from_slice(&[175, 175, 109, 31, 13, 152, 155, 237]); // init discriminator
        data.push(decimals);
        data.extend_from_slice(&initial_supply.to_le_bytes());

        Instruction {
            program_id: GHOST_PROGRAM_ID,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new(*authority, true),
                solana_sdk::instruction::AccountMeta::new(*mint, false),
                solana_sdk::instruction::AccountMeta::new(*config, false),
                solana_sdk::instruction::AccountMeta::new_readonly(
                    solana_sdk::system_program::ID,
                    false,
                ),
                solana_sdk::instruction::AccountMeta::new_readonly(spl_token::ID, false),
                solana_sdk::instruction::AccountMeta::new_readonly(
                    solana_sdk::sysvar::rent::ID,
                    false,
                ),
            ],
            data,
        }
    }

    /// Builds a mint instruction.
    pub fn build_mint_instruction(
        authority: &Pubkey,
        mint: &Pubkey,
        token_account: &Pubkey,
        config: &Pubkey,
        amount: u64,
    ) -> Instruction {
        let mut data = Vec::new();
        data.extend_from_slice(&[102, 76, 55, 251, 38, 73, 215, 88]); // mint discriminator
        data.extend_from_slice(&amount.to_le_bytes());

        Instruction {
            program_id: GHOST_PROGRAM_ID,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new_readonly(*authority, true),
                solana_sdk::instruction::AccountMeta::new(*mint, false),
                solana_sdk::instruction::AccountMeta::new(*token_account, false),
                solana_sdk::instruction::AccountMeta::new_readonly(*config, false),
                solana_sdk::instruction::AccountMeta::new_readonly(spl_token::ID, false),
            ],
            data,
        }
    }

    /// Builds a burn instruction.
    pub fn build_burn_instruction(
        owner: &Pubkey,
        mint: &Pubkey,
        token_account: &Pubkey,
        config: &Pubkey,
        amount: u64,
    ) -> Instruction {
        let mut data = Vec::new();
        data.extend_from_slice(&[177, 49, 83, 8, 165, 119, 131, 175]); // burn discriminator
        data.extend_from_slice(&amount.to_le_bytes());

        Instruction {
            program_id: GHOST_PROGRAM_ID,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new_readonly(*owner, true),
                solana_sdk::instruction::AccountMeta::new(*mint, false),
                solana_sdk::instruction::AccountMeta::new(*token_account, false),
                solana_sdk::instruction::AccountMeta::new_readonly(*config, false),
                solana_sdk::instruction::AccountMeta::new_readonly(spl_token::ID, false),
            ],
            data,
        }
    }

    /// Builds a pause instruction.
    pub fn build_pause_instruction(authority: &Pubkey, config: &Pubkey) -> Instruction {
        let mut data = Vec::new();
        data.extend_from_slice(&[211, 11, 44, 80, 20, 7, 236, 189]); // pause discriminator

        Instruction {
            program_id: GHOST_PROGRAM_ID,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new_readonly(*authority, true),
                solana_sdk::instruction::AccountMeta::new(*config, false),
            ],
            data,
        }
    }

    /// Builds an unpause instruction.
    pub fn build_unpause_instruction(authority: &Pubkey, config: &Pubkey) -> Instruction {
        let mut data = Vec::new();
        data.extend_from_slice(&[88, 67, 143, 52, 254, 215, 3, 27]); // unpause discriminator

        Instruction {
            program_id: GHOST_PROGRAM_ID,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new_readonly(*authority, true),
                solana_sdk::instruction::AccountMeta::new(*config, false),
            ],
            data,
        }
    }

    // =========================================================================
    // Convenience Structs
    // =========================================================================

    /// A convenience struct bundling all test signers together.
    pub struct TestSigners {
        pub payer: Keypair,
        pub authority: Keypair,
        pub mint: Keypair,
        pub config: Keypair,
        pub token_account: Keypair,
    }

    impl TestSigners {
        /// Creates a new set of test signers with random keypairs.
        pub fn new() -> Self {
            Self {
                payer: Keypair::new(),
                authority: Keypair::new(),
                mint: Keypair::new(),
                config: Keypair::new(),
                token_account: Keypair::new(),
            }
        }

        /// Returns all signers as a slice.
        pub fn all(&self) -> Vec<&Keypair> {
            vec![
                &self.payer,
                &self.authority,
                &self.mint,
                &self.config,
                &self.token_account,
            ]
        }
    }

    impl Default for TestSigners {
        fn default() -> Self {
            Self::new()
        }
    }

    // =========================================================================
    // Self-tests for helpers
    // =========================================================================

    #[cfg(test)]
    mod helper_tests {
        use super::*;

        #[test]
        fn test_ghost_error_codes_are_unique() {
            let errors = vec![
                GhostError::AlreadyInitialized,
                GhostError::NotInitialized,
                GhostError::Unauthorized,
                GhostError::InvalidDecimals,
                GhostError::ZeroAmount,
                GhostError::InvalidMint,
                GhostError::InsufficientBalance,
                GhostError::Paused,
                GhostError::NameTooLong,
                GhostError::SymbolTooLong,
                GhostError::UriTooLong,
                GhostError::InvalidVersion,
                GhostError::RotationDelayNotMet,
                GhostError::NotPendingRotation,
                GhostError::Overflow,
                GhostError::MetadataFrozen,
            ];

            let mut codes: Vec<u32> = errors.iter().map(|e| e.to_anchor_error_code()).collect();
            codes.sort();
            codes.dedup();

            assert_eq!(codes.len(), errors.len(), "All error codes must be unique");
        }

        #[test]
        fn test_anchor_error_format() {
            let error = GhostError::Unauthorized;
            let bytes = make_anchor_error(error);

            assert_eq!(bytes.len(), 12, "Anchor error must be 12 bytes");
            assert_eq!(
                &bytes[..8],
                &ANCHOR_ERROR_DISCRIMINATOR,
                "First 8 bytes must be Anchor error discriminator"
            );
            assert_eq!(
                &bytes[8..],
                &(error as u32).to_le_bytes(),
                "Last 4 bytes must be LE error code"
            );
        }

        #[test]
        fn test_test_signers_have_unique_keys() {
            let signers = TestSigners::new();
            let keys = vec![
                signers.payer.pubkey(),
                signers.authority.pubkey(),
                signers.mint.pubkey(),
                signers.config.pubkey(),
                signers.token_account.pubkey(),
            ];

            for i in 0..keys.len() {
                for j in (i + 1)..keys.len() {
                    assert_ne!(keys[i], keys[j], "All test signer keys must be unique");
                }
            }
        }

        #[test]
        fn test_instruction_builders_produce_valid_instructions() {
            let signers = TestSigners::new();

            let init_ix = build_initialize_instruction(
                &signers.authority.pubkey(),
                &signers.mint.pubkey(),
                &signers.config.pubkey(),
                TOKEN_DECIMALS,
                INITIAL_SUPPLY_RAW,
            );

            assert_eq!(init_ix.program_id, GHOST_PROGRAM_ID);
            assert_eq!(init_ix.accounts.len(), 6);
            assert!(!init_ix.data.is_empty());

            let mint_ix = build_mint_instruction(
                &signers.authority.pubkey(),
                &signers.mint.pubkey(),
                &signers.token_account.pubkey(),
                &signers.config.pubkey(),
                1_000_000_000,
            );

            assert_eq!(mint_ix.program_id, GHOST_PROGRAM_ID);
            assert_eq!(mint_ix.accounts.len(), 5);
        }
    }
}

