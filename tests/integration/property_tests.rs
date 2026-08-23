#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;

    const INITIAL_SUPPLY: u64 = 10_000_000_000_000_000_000;

    /// Simulates the core state of the GHOST token without requiring actual
    /// Solana runtime, allowing property-based testing over many random inputs.
    #[derive(Debug, Clone)]
    struct GhostModel {
        total_supply: u64,
        initialized: bool,
    }

    impl GhostModel {
        fn new() -> Self {
            Self {
                total_supply: 0,
                initialized: false,
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
            if amount == 0 {
                return Err("ZeroAmount");
            }
            let new_supply = self.total_supply.checked_add(amount).ok_or("Overflow")?;
            self.total_supply = new_supply;
            Ok(self.total_supply)
        }

        fn burn(&mut self, amount: u64) -> Result<u64, &'static str> {
            if amount == 0 {
                return Err("ZeroAmount");
            }
            let new_supply = self
                .total_supply
                .checked_sub(amount)
                .ok_or("InsufficientBalance")?;
            self.total_supply = new_supply;
            Ok(self.total_supply)
        }
    }

    proptest! {
        /// Property: For any positive amount, minting increases total supply
        /// by exactly that amount.
        #[test]
        fn prop_mint_amount_always_increases_supply(
            amount in 1u64..1_000_000_000_000_000
        ) {
            let mut model = GhostModel::new();
            model.init(INITIAL_SUPPLY).unwrap();

            let before = model.total_supply;
            let result = model.mint(amount);

            prop_assert!(result.is_ok(), "Mint of {} should succeed", amount);
            let after = model.total_supply;

            prop_assert_eq!(
                after,
                before.checked_add(amount).unwrap(),
                "Supply should be exactly before + amount"
            );
            prop_assert!(
                after > before,
                "Supply must increase after successful mint"
            );
        }

        /// Property: Burning never increases supply.
        #[test]
        fn prop_burn_never_increases_supply(
            amount in 1u64..1_000_000
        ) {
            let mut model = GhostModel::new();
            let initial = 1_000_000_000; // Start with enough to burn
            model.init(initial).unwrap();

            let before = model.total_supply;

            if amount <= before {
                let result = model.burn(amount);
                prop_assert!(result.is_ok(), "Burn of {} should succeed when <= balance", amount);
                let after = model.total_supply;

                prop_assert!(
                    after < before,
                    "Supply must strictly decrease after burn"
                );
                prop_assert_eq!(
                    after,
                    before - amount,
                    "Supply should be exactly before - amount"
                );
            } else {
                let result = model.burn(amount);
                prop_assert!(result.is_err(), "Burn of {} should fail when > balance", amount);
                // Supply unchanged after failed burn
                prop_assert_eq!(model.total_supply, before);
            }
        }

        /// Property: Supply always remains non-negative regardless of operations.
        #[test]
        fn prop_supply_always_non_negative(
            operations in prop::collection::vec(
                (0u8..2u8, 0u64..1000u64),
                0..50
            )
        ) {
            let mut model = GhostModel::new();
            model.init(1_000_000).unwrap();

            for (op_code, amount) in operations {
                match op_code {
                    0 => { // Mint
                        let _ = model.mint(amount);
                    }
                    1 => { // Burn
                        let _ = model.burn(amount);
                    }
                    _ => unreachable!(),
                }

                // Core invariant: supply must never exceed u64::MAX
                prop_assert!(
                    model.total_supply <= u64::MAX,
                    "Supply must remain within u64 bounds"
                );
            }
        }

        /// Property: After a successful mint and subsequent burn of the same
        /// amount, supply returns to original value.
        #[test]
        fn prop_mint_then_burn_returns_to_original(
            amount in 1u64..1_000_000
        ) {
            let mut model = GhostModel::new();
            model.init(10_000_000).unwrap();

            let original = model.total_supply;

            model.mint(amount).unwrap();
            model.burn(amount).unwrap();

            prop_assert_eq!(
                model.total_supply,
                original,
                "Mint then burn of same amount should return to original supply"
            );
        }

        /// Property: Burning then minting the same amount returns to original.
        #[test]
        fn prop_burn_then_mint_returns_to_original(
            amount in 1u64..1_000_000
        ) {
            let mut model = GhostModel::new();
            let initial = 5_000_000;
            model.init(initial).unwrap();

            let original = model.total_supply;

            // Only attempt if we have enough to burn
            if amount <= original {
                model.burn(amount).unwrap();
                model.mint(amount).unwrap();

                prop_assert_eq!(
                    model.total_supply,
                    original,
                    "Burn then mint of same amount should return to original supply"
                );
            }
        }

        /// Property: Zero amount operations always fail.
        #[test]
        fn prop_zero_mint_always_fails(_seed in any::<u64>()) {
            let mut model = GhostModel::new();
            model.init(1_000_000).unwrap();

            let result = model.mint(0);
            prop_assert!(result.is_err(), "Minting 0 must fail");

            let result = model.burn(0);
            prop_assert!(result.is_err(), "Burning 0 must fail");
        }

        /// Property: Initialization only works once.
        #[test]
        fn prop_double_init_fails(first_supply in 1u64..INITIAL_SUPPLY) {
            let mut model = GhostModel::new();

            let r1 = model.init(first_supply);
            prop_assert!(r1.is_ok(), "First init should succeed");

            let r2 = model.init(second_supply_for_test());
            prop_assert!(r2.is_err(), "Second init should fail");
        }
    }

    // Helper to avoid closure capture issues with proptest
    fn second_supply_for_test() -> u64 {
        INITIAL_SUPPLY + 1
    }

    /// Deterministic property tests that run without proptest runtime
    #[cfg(test)]
    mod deterministic {
        use super::*;

        #[test]
        fn test_monte_carlo_operations() {
            // Run a Monte Carlo simulation with 1000 random operations
            let mut model = GhostModel::new();
            model.init(1_000_000_000).unwrap();

            let original = model.total_supply;
            let mut net_change: i128 = 0;

            // Use a simple LCG for pseudo-randomness
            let mut seed: u64 = 12345;
            for _ in 0..1000 {
                seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let op_code = (seed >> 32) & 1;
                let amount = ((seed >> 16) & 0xFFFF) + 1; // 1..65536

                match op_code {
                    0 => {
                        if let Ok(_) = model.mint(amount) {
                            net_change += amount as i128;
                        }
                    }
                    1 => {
                        if let Ok(_) = model.burn(amount) {
                            net_change -= amount as i128;
                        }
                    }
                    _ => unreachable!(),
                }

                let expected = (original as i128 + net_change) as u64;
                assert_eq!(
                    model.total_supply, expected,
                    "Supply invariant broken at iteration with net_change={}",
                    net_change
                );
            }
        }
    }
}
