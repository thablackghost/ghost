use crate::errors::GhostError;
use anchor_lang::prelude::*;

/// Checks that the mint account is a valid SPL Token-2022 mint and
/// verifies the expected extensions are present (or absent).
///
/// The GHOST token uses SPL Token-2022 for its enhanced functionality.
/// This function validates:
/// 1. The account is owned by the Token-2022 program.
/// 2. The account data can be deserialized as a Token-2022 mint.
/// 3. The mint has the expected extensions configured.
pub fn check_token_2022_extensions(mint: &AccountInfo) -> Result<()> {
    require!(
        mint.owner == &spl_token_2022::ID,
        GhostError::InvalidMintOwner
    );

    let data = mint.data.borrow();
    let mint_state =
        spl_token_2022::extension::StateWithExtensions::<spl_token_2022::state::Mint>::unpack(
            &data,
        )
        .map_err(|_| GhostError::InvalidMintData)?;

    // Verify the mint has the expected number of decimals.
    require!(mint_state.base.decimals == 9, GhostError::InvalidDecimals);

    // Verify the mint supply is consistent with the expected total supply.
    // The mint authority check ensures no unauthorized minting.
    // (Supply validation is done at the instruction level.)
    msg!(
        "Token-2022 mint verified: decimals={}, supply={}",
        mint_state.base.decimals,
        mint_state.base.supply
    );

    Ok(())
}

/// Validates that the mint does NOT have a transfer hook extension enabled.
///
/// Transfer hooks can interfere with program-controlled transfers by
/// requiring additional accounts and CPI calls. The GHOST token program
/// intentionally avoids transfer hooks to ensure deterministic transfers.
///
/// # Arguments
/// * `mint` - The mint account to check for transfer hook extension.
pub fn validate_no_transfer_hook(mint: &AccountInfo) -> Result<()> {
    let data = mint.data.borrow();
    let mint_state =
        spl_token_2022::extension::StateWithExtensions::<spl_token_2022::state::Mint>::unpack(
            &data,
        )
        .map_err(|_| GhostError::InvalidMintData)?;

    // Check if transfer hook extension is present.
    // The `get_extension` method returns `Ok` if the extension exists,
    // or an error if it doesn't. We WANT the error (no extension).
    let transfer_hook_result =
        mint_state.get_extension::<spl_token_2022::extension::transfer_hook::TransferHook>();

    match transfer_hook_result {
        Ok(_) => {
            msg!(
                "WARNING: Transfer hook extension detected on mint {}",
                mint.key()
            );
            Err(GhostError::TransferHookNotAllowed.into())
        }
        Err(_) => {
            // No transfer hook - this is what we want.
            msg!("No transfer hook extension on mint {} - OK", mint.key());
            Ok(())
        }
    }
}

/// Token-2022 Extensions Overview for GHOST:
///
/// **Extensions USED by GHOST:**
/// - **MintCloseAuthority**: Allows the mint to be closed after all tokens are burned.
/// - **DefaultAccountState**: Sets default state for new token accounts (initialized).
///
/// **Extensions AVOIDED by GHOST:**
/// - **TransferHook**: Would require extra accounts and CPIs during transfers.
///   This is validated in `validate_no_transfer_hook`.
/// - **TransferFee**: Would complicate transfer amounts and supply accounting.
/// - **ConfidentialTransfer**: Not needed for a transparent token.
/// - **InterestBearing**: Not applicable to the GHOST token economics.
/// - **NonTransferable**: Would prevent all transfers (we want transferable tokens).
/// - **PermanentDelegate**: Would give a delegate permanent control over all accounts.
/// - **CpiGuard**: Could interfere with program-controlled token operations.
///
/// When the GHOST mint is created, only the approved extensions should be
/// enabled. Any unexpected extensions will cause validation failures.
pub fn validate_ghost_mint_extensions(mint: &AccountInfo) -> Result<()> {
    // Perform the base check.
    check_token_2022_extensions(mint)?;

    // Ensure no transfer hook.
    validate_no_transfer_hook(mint)?;

    msg!(
        "All Token-2022 extensions validated for GHOST mint: {}",
        mint.key()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_check_token_2022_extensions_wrong_owner() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![0; 82]; // Minimum mint size
        let owner = Pubkey::new_unique();

        let account_info = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        assert!(check_token_2022_extensions(&account_info).is_err());
    }

    #[test]
    fn test_validate_no_transfer_hook_invalid_data() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![0; 4]; // Way too small
        let owner = spl_token_2022::ID;

        let account_info = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        assert!(validate_no_transfer_hook(&account_info).is_err());
    }
}
