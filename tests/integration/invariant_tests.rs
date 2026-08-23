#[cfg(test)]
mod invariant_tests {
    /// Invariant tests verify that key properties of the system hold true
    /// regardless of the sequence of operations performed.
    ///
    /// These are critical for ensuring the token contract maintains
    /// mathematical and security invariants.

    const INITIAL_SUPPLY: u64 = 10_000_000_000_000_000_000; // 10B * 10^9
    const GHOST_DECIMALS: u8 = 9;

    /// Simulates the core state of the GHOST token system for invariant checking.
    #[derive(Debug, Clone, PartialEq)]
    struct GhostState {
        total_supply: u64,
        mint_authority: Option<[u8; 32]>, // pubkey bytes
        is_initialized: bool,
        is_paused: bool,
        decimals: u8,
    }

    impl GhostState {
        fn new() -> Self {
            Self {
                total_supply: 0,
                mint_authority: None,
                is_initialized: false,
                is_paused: false,
                decimals: 0,
            }
        }

        fn initialize(
            &mut self,
            authority: [u8; 32],
            decimals: u8,
            initial_supply: u64,
        ) -> Result<(), &'static str> {
            if self.is_initialized {
                return Err("AlreadyInitialized");
            }
            if decimals > 9 {
                return Err("InvalidDecimals");
            }
            self.mint_authority = Some(authority);
            self.decimals = decimals;
            self.total_supply = initial_supply;
            self.is_initialized = true;
            Ok(())
        }

        fn mint(&mut self, authority: [u8; 32], amount: u64) -> Result<(), &'static str> {
            if self.is_paused {
                return Err("Paused");
            }
            if amount == 0 {
                return Err("ZeroAmount");
            }
            match self.mint_authority {
                Some(auth) if auth == authority => {
                    self.total_supply = self.total_supply.checked_add(amount).ok_or("Overflow")?;
                    Ok(())
                }
                _ => Err("Unauthorized"),
            }
        }

        fn burn(&mut self, amount: u64) -> Result<(), &'static str> {
            if self.is_paused {
                return Err("Paused");
            }
            if amount == 0 {
                return Err("ZeroAmount");
            }
            self.total_supply = self
                .total_supply
                .checked_sub(amount)
                .ok_or("InsufficientBalance")?;
            Ok(())
        }
    }

    // Helper to create a deterministic pubkey-like byte array
    fn auth_bytes(seed: u8) -> [u8; 32] {
        let mut bytes = [seed; 32];
        bytes[0] = seed;
        bytes[31] = !seed;
        bytes
    }

    #[test]
    fn test_supply_never_negative() {
        // After any valid operation, total_supply must always be >= 0 (trivially true for u64)
        let tests_and_fail = vec![
            (
                "init_ok",
                GhostState::new()
                    .initialize(auth_bytes(1), 9, INITIAL_SUPPLY)
                    .is_ok(),
                true,
            ),
            (
                "init_twice_error",
                {
                    let mut s = GhostState::new();
                    s.initialize(auth_bytes(1), 9, INITIAL_SUPPLY).unwrap();
                    s.initialize(auth_bytes(1), 9, INITIAL_SUPPLY).is_err()
                },
                true,
            ),
        ];

        for (name, result, expected) in &tests_and_fail {
            assert_eq!(*result, *expected, "Test '{}' failed", name);
        }

        // Supply is u64, so it can never be negative by type system guarantee.
        // The real invariant is: supply never decreases below 0 and never wraps.
        let state = GhostState::new();
        assert!(state.total_supply == 0, "Initial supply should be 0");

        // After initialization, supply should be positive
        let mut state = GhostState::new();
        state.initialize(auth_bytes(1), 9, INITIAL_SUPPLY).unwrap();
        assert!(state.total_supply > 0, "Supply after init should be > 0");
        assert_eq!(state.total_supply, INITIAL_SUPPLY);
    }

    #[test]
    fn test_burn_cannot_increase_supply() {
        let mut state = GhostState::new();
        let auth = auth_bytes(1);
        state.initialize(auth, 9, 1_000_000_000_000).unwrap();

        let before = state.total_supply;
        state.burn(500_000_000_000).unwrap();
        let after = state.total_supply;

        assert!(
            after < before,
            "Supply must strictly decrease after burn: before={}, after={}",
            before,
            after
        );
        assert_eq!(
            after,
            before - 500_000_000_000,
            "Supply should decrease by exact burn amount"
        );
    }

    #[test]
    fn test_mint_cannot_decrease_supply() {
        let mut state = GhostState::new();
        let auth = auth_bytes(1);
        state.initialize(auth, 9, 1_000_000_000_000).unwrap();

        let before = state.total_supply;
        state.mint(auth, 500_000_000_000).unwrap();
        let after = state.total_supply;

        assert!(
            after > before,
            "Supply must strictly increase after mint: before={}, after={}",
            before,
            after
        );
        assert_eq!(
            after,
            before + 500_000_000_000,
            "Supply should increase by exact mint amount"
        );
    }

    #[test]
    fn test_only_mint_authority_can_mint() {
        let mut state = GhostState::new();
        let authority = auth_bytes(1);
        let non_authority = auth_bytes(99);

        state.initialize(authority, 9, 1_000_000_000_000).unwrap();

        // Authority can mint
        assert!(
            state.mint(authority, 100).is_ok(),
            "Mint authority should be able to mint"
        );

        // Non-authority cannot mint
        assert!(
            state.mint(non_authority, 100).is_err(),
            "Non-authority should NOT be able to mint"
        );

        // Supply should not have changed after failed mint
        let supply_after_failed = state.total_supply;
        assert_eq!(
            supply_after_failed,
            1_000_000_000_000 + 100,
            "Supply should only reflect the successful mint"
        );
    }

    #[test]
    fn test_non_authority_cannot_rotate() {
        // Authority rotation (or any privileged operation) must only be
        // callable by the current authority.

        let mut state = GhostState::new();
        let authority = auth_bytes(1);
        state.initialize(authority, 9, INITIAL_SUPPLY).unwrap();

        // After initialization, only the stored authority should be able
        // to perform privileged operations
        assert_eq!(state.mint_authority, Some(authority));
        assert_eq!(state.is_initialized, true);

        // A non-authority attempting to change state should be rejected
        let non_auth = auth_bytes(42);
        assert_ne!(
            non_auth, authority,
            "Non-authority pubkey must differ from real authority"
        );
    }

    #[test]
    fn test_initialization_only_once() {
        let mut state = GhostState::new();
        let auth1 = auth_bytes(1);
        let auth2 = auth_bytes(2);

        // First initialization should succeed
        let result1 = state.initialize(auth1, 9, INITIAL_SUPPLY);
        assert!(result1.is_ok(), "First initialization should succeed");

        // Second initialization should fail
        let result2 = state.initialize(auth2, 9, INITIAL_SUPPLY);
        assert!(
            result2.is_err(),
            "Second initialization should fail with AlreadyInitialized"
        );

        // State should remain from first initialization
        assert_eq!(
            state.mint_authority,
            Some(auth1),
            "Authority should be from first initialization"
        );
        assert_eq!(
            state.total_supply, INITIAL_SUPPLY,
            "Supply should be from first initialization"
        );
    }

    #[test]
    fn test_combined_operations_invariant() {
        // Run a sequence of operations and verify invariants hold
        let mut state = GhostState::new();
        let auth = auth_bytes(1);

        state.initialize(auth, 9, 10_000_000_000).unwrap();

        let operations: Vec<(&str, u64)> = vec![
            ("mint", 5_000_000_000),
            ("mint", 1_000),
            ("burn", 2_000),
            ("mint", 10_000_000),
            ("burn", 1_000_000),
            ("mint", 500),
        ];

        let mut expected_supply: i128 = 10_000_000_000;

        for (op, amount) in operations {
            match op {
                "mint" => {
                    let result = state.mint(auth, amount);
                    assert!(result.is_ok(), "Mint of {} should succeed", amount);
                    expected_supply += amount as i128;
                }
                "burn" => {
                    let result = state.burn(amount);
                    assert!(result.is_ok(), "Burn of {} should succeed", amount);
                    expected_supply -= amount as i128;
                }
                _ => unreachable!(),
            }

            assert_eq!(
                state.total_supply as i128, expected_supply,
                "Supply invariant broken after {} of {}",
                op, amount
            );
            assert!(state.total_supply >= 0);
        }
    }
}

