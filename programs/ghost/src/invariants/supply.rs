use anchor_lang::prelude::*;

use crate::errors::GhostError;
use crate::math::supply::*;
use crate::state::{GlobalConfig, SupplyOperation};

/// INVARIANT_1: Only the configured mint authority can mint tokens.
/// This is enforced at the instruction entry point by checking the signer.
pub const INVARIANT_1_MSG: &str = "INVARIANT_1: Only configured mint authority can mint";

/// INVARIANT_2: No burn operation can increase the total supply.
pub const INVARIANT_2_MSG: &str = "INVARIANT_2: No burn can increase supply";

/// INVARIANT_3: No mint operation can decrease the total supply.
pub const INVARIANT_3_MSG: &str = "INVARIANT_3: No mint can decrease supply";

/// Checks all supply invariants for a given operation.
///
/// # Arguments
/// * `config` - The global configuration (unused here, reserved for future checks).
/// * `operation` - The supply operation being performed (Mint, Burn, Transfer).
/// * `pre` - Total supply before the operation.
/// * `post` - Total supply after the operation.
/// * `amount` - The amount of tokens involved in the operation.
pub fn check_all_supply_invariants(
    _config: &GlobalConfig,
    operation: SupplyOperation,
    pre: u64,
    post: u64,
    amount: u64,
) -> Result<()> {
    match operation {
        SupplyOperation::Mint => {
            // INVARIANT_3: Mint must increase supply
            if post < pre {
                msg!("{}", INVARIANT_3_MSG);
                return Err(GhostError::SupplyInvariantViolated.into());
            }
            verify_supply_invariant_mint(pre, amount, post)?;
        }
        SupplyOperation::Burn => {
            // INVARIANT_2: Burn must decrease supply
            if post > pre {
                msg!("{}", INVARIANT_2_MSG);
                return Err(GhostError::SupplyInvariantViolated.into());
            }
            verify_supply_invariant_burn(pre, amount, post)?;
        }
        SupplyOperation::Transfer => {
            // Transfers must not change total supply
            if pre != post {
                return Err(GhostError::SupplyInvariantViolated.into());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    fn dummy_config() -> GlobalConfig {
        GlobalConfig {
            mint: Pubkey::new_unique(),
            ..Default::default()
        }
    }

    #[test]
    fn test_mint_invariant_passes() {
        let cfg = dummy_config();
        assert!(check_all_supply_invariants(&cfg, SupplyOperation::Mint, 100, 150, 50).is_ok());
    }

    #[test]
    fn test_mint_invariant_violated_decrease() {
        let cfg = dummy_config();
        assert!(check_all_supply_invariants(&cfg, SupplyOperation::Mint, 100, 50, 50).is_err());
    }

    #[test]
    fn test_mint_invariant_wrong_math() {
        let cfg = dummy_config();
        assert!(check_all_supply_invariants(&cfg, SupplyOperation::Mint, 100, 200, 50).is_err());
    }

    #[test]
    fn test_burn_invariant_passes() {
        let cfg = dummy_config();
        assert!(check_all_supply_invariants(&cfg, SupplyOperation::Burn, 100, 50, 50).is_ok());
    }

    #[test]
    fn test_burn_invariant_violated_increase() {
        let cfg = dummy_config();
        assert!(check_all_supply_invariants(&cfg, SupplyOperation::Burn, 100, 150, 50).is_err());
    }

    #[test]
    fn test_transfer_invariant() {
        let cfg = dummy_config();
        assert!(check_all_supply_invariants(&cfg, SupplyOperation::Transfer, 100, 100, 50).is_ok());
        assert!(
            check_all_supply_invariants(&cfg, SupplyOperation::Transfer, 100, 200, 50).is_err()
        );
    }
}
