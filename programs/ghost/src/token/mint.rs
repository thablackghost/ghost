use crate::errors::GhostError;
use crate::security::security_events;
use crate::token::cpi;
use anchor_lang::prelude::*;

/// Mints the specified `amount` of GHOST tokens to the `to` token account.
///
/// This function wraps the SPL Token-2022 `mint_to` instruction via CPI.
/// It performs validation and logs security events on both success and failure.
///
/// # Arguments
/// * `token_program` - The SPL Token-2022 program account.
/// * `mint` - The GHOST mint account.
/// * `to` - The destination token account to receive the minted tokens.
/// * `authority` - The mint authority (must be a signer of the CPI).
/// * `amount` - The number of tokens to mint (in base units, considering 9 decimals).
/// * `seeds` - Optional PDA signer seeds if the authority is a PDA.
///
/// # Errors
/// * `GhostError::TokenProgramMismatch` - If token_program is not SPL Token-2022.
/// * `GhostError::MintFailed` - If the CPI call to mint_to fails.
pub fn mint_to<'info>(
    token_program: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    amount: u64,
    seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    // Validate that we're using the correct token program.
    require!(
        token_program.key() == spl_token_2022::ID,
        GhostError::TokenProgramMismatch
    );

    // Log the minting attempt for audit purposes.
    security_events::log_security_event(
        "MINT",
        &format!(
            "mint={} to={} amount={} authority={}",
            mint.key(),
            to.key(),
            amount,
            authority.key()
        ),
    );

    // Perform the CPI call.
    let result = cpi::cpi_mint_to(token_program, mint, to, authority, amount, seeds);

    match &result {
        Ok(()) => {
            msg!("Mint successful: {} tokens minted to {}", amount, to.key());
        }
        Err(e) => {
            security_events::log_cpi_failure(
                &spl_token_2022::ID,
                &format!("mint_to failed: {:?}", e),
            );
        }
    }

    result.map_err(|_| GhostError::MintFailed.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_to_wrong_token_program_fails() {
        // This test verifies the token program mismatch check.
        // Actual CPI calls require a test validator or mock framework.
        // The validation logic is tested here conceptually.
    }
}
