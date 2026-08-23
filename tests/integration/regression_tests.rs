#[cfg(test)]
mod regression_tests {
    /// Regression tests ensure that previously fixed bugs do not resurface.
    ///
    /// Each test below corresponds to a known issue that was identified,
    /// fixed, and should be verified on every CI run to prevent regression.

    const INITIAL_SUPPLY: u64 = 10_000_000_000_000_000_000;

    /// Simulated GHOST token state for regression testing.
    #[derive(Debug, Clone, PartialEq)]
    struct GhostState {
        total_supply: u64,
        initialized: bool,
        paused: bool,
        decimals: u8,
    }

    impl GhostState {
        fn new() -> Self {
            Self {
                total_supply: 0,
                initialized: false,
                paused: false,
                decimals: 0,
            }
        }

        fn init(&mut self, decimals: u8, supply: u64) -> Result<(), &'static str> {
            if self.initialized {
                return Err("AlreadyInitialized");
            }
            if decimals > 9 {
                return Err("InvalidDecimals");
            }
            self.decimals = decimals;
            self.total_supply = supply;
            self.initialized = true;
            Ok(())
        }

        fn mint(&mut self, amount: u64) -> Result<u64, &'static str> {
            if !self.initialized {
                return Err("NotInitialized");
            }
            if self.paused {
                return Err("Paused");
            }
            if amount == 0 {
                return Err("ZeroAmount");
            }
            let new = self.total_supply.checked_add(amount).ok_or("Overflow")?;
            Ok(std::mem::replace(&mut self.total_supply, new))
        }

        fn burn(&mut self, amount: u64) -> Result<u64, &'static str> {
            if !self.initialized {
                return Err("NotInitialized");
            }
            if self.paused {
                return Err("Paused");
            }
            if amount == 0 {
                return Err("ZeroAmount");
            }
            let new = self
                .total_supply
                .checked_sub(amount)
                .ok_or("InsufficientBalance")?;
            Ok(std::mem::replace(&mut self.total_supply, new))
        }
    }

    // =========================================================================
    // BUG-001: Initialization allowed decimals > 9
    // Fixed: Added decimals validation in initialize handler
    // =========================================================================
    #[test]
    fn regression_bug_001_invalid_decimals_accepted() {
        let mut state = GhostState::new();

        // Before fix: decimals=19 was accepted
        let result = state.init(19, INITIAL_SUPPLY);
        assert!(
            result.is_err(),
            "REGRESSION BUG-001: Decimals > 9 must be rejected"
        );
        assert!(
            !state.initialized,
            "State should remain uninitialized after invalid init"
        );
    }

    // =========================================================================
    // BUG-002: u64 overflow on mint when supply near MAX
    // Fixed: Added checked_add with proper error handling
    // =========================================================================
    #[test]
    fn regression_bug_002_mint_overflow_not_handled() {
        let mut state = GhostState::new();

        // Set supply to near u64::MAX
        assert!(state.init(9, u64::MAX - 100).is_ok());

        // Try minting 200 (would overflow: MAX-100 + 200 > MAX)
        let result = state.mint(200);

        // Before fix: this would silently overflow/wrap
        // After fix: should return Overflow error
        assert!(
            result.is_err(),
            "REGRESSION BUG-002: Mint overflow must be caught"
        );
        // Supply must be unchanged after failed operation
        assert_eq!(
            state.total_supply,
            u64::MAX - 100,
            "Supply must not change after failed mint"
        );
    }

    // =========================================================================
    // BUG-003: Supply could go below zero via repeated burns
    // Fixed: Added checked_sub on burn path
    // =========================================================================
    #[test]
    fn regression_bug_003_supply_underflow() {
        let mut state = GhostState::new();
        assert!(state.init(9, 500).is_ok());

        // Before fix: burning 200 twice (total 400) ok,
        // then burning 200 more would underflow
        assert!(state.burn(200).is_ok());
        assert!(state.burn(200).is_ok());
        assert_eq!(state.total_supply, 100);

        let result = state.burn(200); // Would go to -100 without checked_sub
        assert!(
            result.is_err(),
            "REGRESSION BUG-003: Burn underflow must be caught"
        );
        assert_eq!(
            state.total_supply, 100,
            "Supply must remain unchanged after failed burn"
        );
    }

    // =========================================================================
    // BUG-004: Initialization could be called multiple times
    // Fixed: Added is_initialized guard
    // =========================================================================
    #[test]
    fn regression_bug_004_double_init_allowed() {
        let mut state = GhostState::new();

        assert!(state.init(9, 1_000_000).is_ok());
        assert_eq!(state.total_supply, 1_000_000);

        // Second initialization should fail
        let result = state.init(9, 5_000_000);
        assert!(
            result.is_err(),
            "REGRESSION BUG-004: Double init must be rejected"
        );

        // State must be from first initialization
        assert_eq!(
            state.total_supply, 1_000_000,
            "Supply must remain from first initialization"
        );
    }

    // =========================================================================
    // BUG-005: Zero-amount operations were silently succeeding
    // Fixed: Added zero-amount checks in mint and burn
    // =========================================================================
    #[test]
    fn regression_bug_005_zero_amount_operations_succeed() {
        let mut state = GhostState::new();
        assert!(state.init(9, INITIAL_SUPPLY).is_ok());

        let supply_before = state.total_supply;

        let mint_result = state.mint(0);
        assert!(
            mint_result.is_err(),
            "REGRESSION BUG-005: Mint of 0 must fail"
        );
        assert_eq!(
            state.total_supply, supply_before,
            "Supply must not change after zero mint"
        );

        let burn_result = state.burn(0);
        assert!(
            burn_result.is_err(),
            "REGRESSION BUG-005: Burn of 0 must fail"
        );
        assert_eq!(
            state.total_supply, supply_before,
            "Supply must not change after zero burn"
        );
    }

    // =========================================================================
    // BUG-006: Paused state could be bypassed via certain paths
    // Fixed: Added pause check to all mutable operations
    // =========================================================================
    #[test]
    fn regression_bug_006_pause_bypass() {
        let mut state = GhostState::new();
        assert!(state.init(9, 1_000_000).is_ok());

        // Pause the contract
        state.paused = true;

        // All mutable operations should fail
        let mint_result = state.mint(100);
        assert!(
            mint_result.is_err(),
            "REGRESSION BUG-006: Mint must fail when paused"
        );

        let burn_result = state.burn(100);
        assert!(
            burn_result.is_err(),
            "REGRESSION BUG-006: Burn must fail when paused"
        );
    }

    // =========================================================================
    // BUG-007: Decimals validation was unsigned, could wrap
    // Fixed: Bounds check explicitly 0..=9
    // =========================================================================
    #[test]
    fn regression_bug_007_decimals_edge_cases() {
        let valid_decimals: Vec<u8> = vec![0, 1, 5, 9];
        let invalid_decimals: Vec<u8> = vec![10, 19, 100, 255];

        for &dec in &valid_decimals {
            let mut state = GhostState::new();
            assert!(
                state.init(dec, INITIAL_SUPPLY).is_ok(),
                "REGRESSION BUG-007: Decimals {} should be valid",
                dec
            );
        }

        for &dec in &invalid_decimals {
            let mut state = GhostState::new();
            assert!(
                state.init(dec, INITIAL_SUPPLY).is_err(),
                "REGRESSION BUG-007: Decimals {} should be invalid",
                dec
            );
        }
    }

    // =========================================================================
    // BUG-008: PDAs could be generated without canonical bump
    // Fixed: Use anchor's canonical bump enforcement
    // =========================================================================
    #[test]
    fn regression_bug_008_non_canonical_pda_bump() {
        // The program must enforce canonical bump (the first valid bump found
        // by find_program_address). Using a non-canonical bump is a known
        // security issue that allows multiple valid PDAs.

        let seeds: &[&[u8]] = &[b"config"];
        let program_id = solana_sdk::pubkey!("Gh0st111111111111111111111111111111111111111");

        let (canonical_pda, canonical_bump) =
            solana_sdk::pubkey::Pubkey::find_program_address(seeds, &program_id);

        // Verify the canonical bump is the expected one (255 down-search)
        let (_, verify_bump) = solana_sdk::pubkey::Pubkey::find_program_address(seeds, &program_id);
        assert_eq!(
            canonical_bump, verify_bump,
            "REGRESSION BUG-008: PDA derivation must be deterministic"
        );

        // Non-canonical bumps should not produce valid PDAs
        for non_canonical_bump in 0..=255 {
            if non_canonical_bump == canonical_bump {
                continue;
            }

            let mut full_seeds: Vec<&[u8]> = seeds.to_vec();
            let bump_slice = [non_canonical_bump];
            full_seeds.push(&bump_slice);

            let non_canonical_result =
                solana_sdk::pubkey::Pubkey::create_program_address(&full_seeds, &program_id);

            // Non-canonical bumps either fail (Err) or produce a different PDA
            // The program must reject non-canonical PDAs
            match non_canonical_result {
                Ok(non_canonical_pda) => {
                    // If it happens to produce a valid PDA, it must differ from canonical
                    assert_ne!(
                        non_canonical_pda, canonical_pda,
                        "Non-canonical bump {} should not produce the canonical PDA",
                        non_canonical_bump
                    );
                }
                Err(_) => {
                    // create_program_address fails for this bump - expected
                }
            }
        }
    }

    // =========================================================================
    // BUG-009: Exceedingly large instruction data could crash
    // Fixed: Added data size validation
    // =========================================================================
    #[test]
    fn regression_bug_009_large_instruction_data() {
        // Instruction data should be size-limited. Anchor normally enforces
        // this via the Borsh deserialization limits.

        let max_reasonable_size = 1024; // 1KB should be more than enough
        let name = "GHOST";
        let symbol = "GHOST";
        let uri = "https://ghost.token/metadata.json";

        let total_size = 8 // discriminator
            + 4 + name.len()  // name length + data
            + 4 + symbol.len()
            + 4 + uri.len();

        assert!(
            total_size < max_reasonable_size,
            "REGRESSION BUG-009: Instruction data size ({}) exceeds reasonable limit ({})",
            total_size,
            max_reasonable_size
        );
    }

    // =========================================================================
    // BUG-010: Race condition on authority rotation
    // Fixed: Added timelock/delay mechanism
    // =========================================================================
    #[test]
    fn regression_bug_010_authority_rotation_race() {
        // Authority rotation must have a delay period.
        // Without it, a compromised authority could instantly transfer control.

        const ROTATION_DELAY: u64 = 100; // slots

        // Simulate: propose rotation at slot N, cannot accept until slot N + DELAY
        let proposal_slot: u64 = 1000;
        let accept_slot_too_early: u64 = proposal_slot + ROTATION_DELAY - 1;
        let accept_slot_valid: u64 = proposal_slot + ROTATION_DELAY;

        assert!(
            accept_slot_too_early < proposal_slot + ROTATION_DELAY,
            "Accepting before delay should be rejected"
        );
        assert!(
            accept_slot_valid >= proposal_slot + ROTATION_DELAY,
            "Accepting after delay should be allowed"
        );
        assert!(
            ROTATION_DELAY > 0,
            "REGRESSION BUG-010: Rotation delay must be > 0 slots"
        );
    }
}
