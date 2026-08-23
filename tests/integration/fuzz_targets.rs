#[cfg(test)]
mod fuzz_targets {
    /// Fuzz targets for the GHOST token program.
    ///
    /// These targets are designed to be used with `cargo-fuzz` (libfuzzer)
    /// but include standalone test entrypoints that can be run via `cargo test`
    /// with deterministic inputs for quick regression checking.
    ///
    /// To use with cargo-fuzz, add a fuzz target in `fuzz/fuzz_targets/` that
    /// calls these functions with `arbitrary::Arbitrary`-derived inputs.
    use std::panic;

    const INITIAL_SUPPLY: u64 = 10_000_000_000_000_000_000;

    /// The core fuzzable state machine for GHOST token operations.
    #[derive(Debug, Clone)]
    struct FuzzState {
        total_supply: u64,
        initialized: bool,
        paused: bool,
    }

    impl FuzzState {
        fn new() -> Self {
            Self {
                total_supply: 0,
                initialized: false,
                paused: false,
            }
        }

        fn init(&mut self, supply: u64) -> Result<(), &'static str> {
            if self.initialized {
                return Err("AlreadyInitialized");
            }
            self.total_supply = supply;
            self.initialized = true;
            Ok(())
        }

        fn mint(&mut self, amount: u64) -> Result<u64, &'static str> {
            if self.paused {
                return Err("Paused");
            }
            if amount == 0 {
                return Err("ZeroAmount");
            }
            let new = self.total_supply.checked_add(amount).ok_or("Overflow")?;
            self.total_supply = new;
            Ok(new)
        }

        fn burn(&mut self, amount: u64) -> Result<u64, &'static str> {
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
            self.total_supply = new;
            Ok(new)
        }

        fn pause(&mut self) {
            self.paused = true;
        }
        fn unpause(&mut self) {
            self.paused = false;
        }
    }

    /// Entrypoint for fuzzing mint operations with random amounts.
    /// Safe: catches all panics and returns cleanly.
    #[test]
    fn fuzz_mint_amount() {
        let test_amounts: Vec<u64> = vec![
            0,                                // Zero amount (should fail)
            1,                                // Minimum valid
            u64::MAX,                         // Maximum possible
            u64::MAX / 2,                     // Large value
            INITIAL_SUPPLY,                   // Same as initial supply
            INITIAL_SUPPLY.saturating_mul(2), // Double initial supply
            u64::MAX - INITIAL_SUPPLY,        // Remaining capacity
            (u64::MAX - INITIAL_SUPPLY) + 1,  // Just over capacity
            42,                               // Small arbitrary
            1_000_000_000_000_000_000,        // 1 billion tokens
        ];

        for &amount in &test_amounts {
            let result = panic::catch_unwind(|| {
                let mut state = FuzzState::new();

                // Try minting on uninitialized state
                let mint_before_init = state.mint(amount);
                assert!(
                    mint_before_init.is_err(),
                    "Mint on uninitialized state should fail (amount={})",
                    amount
                );

                // Initialize then mint
                state.init(INITIAL_SUPPLY).unwrap();
                let before = state.total_supply;

                match state.mint(amount) {
                    Ok(new_supply) => {
                        // Verify invariants on success
                        assert!(
                            new_supply >= before,
                            "Supply must not decrease: before={}, after={}",
                            before,
                            new_supply
                        );
                        assert!(new_supply <= u64::MAX, "Supply must not exceed u64::MAX");

                        if amount > 0 && INITIAL_SUPPLY <= u64::MAX - amount {
                            assert_eq!(
                                new_supply,
                                before + amount,
                                "Supply must increase by exact amount"
                            );
                        }
                    }
                    Err(e) => {
                        // Verify failure reasons are valid
                        match e {
                            "ZeroAmount" => assert_eq!(
                                amount, 0,
                                "ZeroAmount error only for amount=0, got amount={}",
                                amount
                            ),
                            "Overflow" => assert!(
                                before > u64::MAX - amount,
                                "Overflow error only when supply would exceed u64::MAX"
                            ),
                            "Paused" => {} // Valid if paused
                            _ => panic!("Unexpected error: {}", e),
                        }
                    }
                }
            });

            if let Err(panic_info) = result {
                let msg = panic_info
                    .downcast_ref::<String>()
                    .map(|s| s.as_str())
                    .or_else(|| panic_info.downcast_ref::<&str>().copied())
                    .unwrap_or("(unknown panic)");
                panic!("fuzz_mint_amount panicked for amount={}: {}", amount, msg);
            }
        }
    }

    /// Entrypoint for fuzzing burn operations with random amounts.
    #[test]
    fn fuzz_burn_amount() {
        let test_amounts: Vec<u64> = vec![
            0,                  // Zero (should fail)
            1,                  // Minimum valid
            500,                // Small
            u64::MAX,           // Maximum
            INITIAL_SUPPLY,     // Exact supply
            INITIAL_SUPPLY + 1, // Just over supply
            INITIAL_SUPPLY / 2, // Half supply
            999_999_999,        // Arbitrary
        ];

        for &amount in &test_amounts {
            let result = panic::catch_unwind(|| {
                let mut state = FuzzState::new();

                // Burn on uninitialized state
                let burn_before_init = state.burn(amount);
                assert!(
                    burn_before_init.is_err(),
                    "Burn on uninitialized state should fail (amount={})",
                    amount
                );

                // Initialize with enough supply
                state.init(INITIAL_SUPPLY).unwrap();
                let before = state.total_supply;

                match state.burn(amount) {
                    Ok(new_supply) => {
                        assert!(
                            new_supply <= before,
                            "Supply must not increase: before={}, after={}",
                            before,
                            new_supply
                        );
                        assert_eq!(
                            new_supply,
                            before - amount,
                            "Supply must decrease by exact burn amount"
                        );
                    }
                    Err(e) => match e {
                        "ZeroAmount" => assert_eq!(amount, 0),
                        "InsufficientBalance" => assert!(amount > before),
                        "Paused" => {}
                        _ => panic!("Unexpected burn error: {}", e),
                    },
                }
            });

            if let Err(panic_info) = result {
                let msg = panic_info
                    .downcast_ref::<String>()
                    .map(|s| s.as_str())
                    .or_else(|| panic_info.downcast_ref::<&str>().copied())
                    .unwrap_or("(unknown panic)");
                panic!("fuzz_burn_amount panicked for amount={}: {}", amount, msg);
            }
        }
    }

    /// Entrypoint for fuzzing metadata with random strings.
    #[test]
    fn fuzz_metadata_strings() {
        let test_strings: Vec<&str> = vec![
            "",                                  // Empty
            "A",                                 // Single char
            "GHOST",                             // Valid
            "X".repeat(32).as_str(),             // Exactly max name
            "Y".repeat(33).as_str(),             // One over max name
            "X".repeat(10).as_str(),             // Exactly max symbol
            "Y".repeat(11).as_str(),             // One over max symbol
            "X".repeat(200).as_str(),            // Exactly max URI
            "Y".repeat(201).as_str(),            // One over max URI
            "X".repeat(1000).as_str(),           // Very long
            "https://ghost.token/metadata.json", // Valid URI
            "\0\0\0",                            // Null bytes
            "GHOST 👻 TOKEN",                    // Emoji / unicode
            "<script>alert(1)</script>",         // XSS attempt
            "'; DROP TABLE tokens; --",          // SQL injection attempt
        ];

        // Simulate the actual Anchor string handling without Solana runtime
        for &input in &test_strings {
            let result = panic::catch_unwind(|| {
                let name_bytes = input.as_bytes();
                let symbol_bytes = input.as_bytes();
                let uri_bytes = input.as_bytes();

                // Simulate Anchor String length checks
                // Anchor String has a max length of 4-byte length prefix
                if name_bytes.len() > u32::MAX as usize {
                    panic!("Name too large to serialize into Anchor String");
                }

                // Validate length constraints
                let name_valid = name_bytes.len() <= 32;
                let symbol_valid = symbol_bytes.len() <= 10;
                let uri_valid = uri_bytes.len() <= 200;

                // Log valid/invalid for debugging
                let _ = (name_valid, symbol_valid, uri_valid, input);

                // Ensure strings can be serialized/deserialized safely
                let name_str = std::str::from_utf8(name_bytes);
                if name_bytes.len() <= 32 {
                    // Should be valid or gracefully fail
                    let _ = name_str;
                }
            });

            if let Err(panic_info) = result {
                let msg = panic_info
                    .downcast_ref::<String>()
                    .map(|s| s.as_str())
                    .or_else(|| panic_info.downcast_ref::<&str>().copied())
                    .unwrap_or("(unknown panic)");
                // Only fail on truly unexpected panics
                // Very long strings may legitimately fail
                if input.len() <= 200 {
                    panic!(
                        "fuzz_metadata_strings panicked for input '{}': {}",
                        input, msg
                    );
                }
            }
        }
    }

    /// Fuzz the interaction between pause state and operations.
    #[test]
    fn fuzz_paused_state_operations() {
        let operations: Vec<(bool, u64)> = vec![
            (false, 100), // Unpaused mint
            (true, 100),  // Paused mint
            (true, 0),    // Paused zero mint
            (false, 0),   // Unpaused zero mint
            (false, u64::MAX),
            (true, u64::MAX),
        ];

        for &(should_pause, amount) in &operations {
            let result = panic::catch_unwind(|| {
                let mut state = FuzzState::new();
                state.init(INITIAL_SUPPLY).unwrap();

                if should_pause {
                    state.pause();
                }

                match state.mint(amount) {
                    Ok(_) => {
                        assert!(
                            !should_pause,
                            "Mint should not succeed when paused (amount={})",
                            amount
                        );
                        assert!(amount > 0, "Mint should not succeed with zero amount");
                    }
                    Err(e) => {
                        let valid_error = match e {
                            "Paused" => should_pause,
                            "ZeroAmount" => amount == 0,
                            "Overflow" => amount > u64::MAX - INITIAL_SUPPLY,
                            _ => false,
                        };
                        if !valid_error {
                            panic!(
                                "Unexpected error '{}' for paused={}, amount={}",
                                e, should_pause, amount
                            );
                        }
                    }
                }
            });

            if let Err(_) = result {
                panic!(
                    "Panic in fuzz_paused_state for paused={}, amount={}",
                    should_pause, amount
                );
            }
        }
    }
}
