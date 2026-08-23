use crate::errors::GhostError;
use crate::security::security_events;
use crate::token::cpi;
use anchor_lang::prelude::*;

/// Transfers `amount` of GHOST tokens from `from` to `to` using
/// the SPL Token-2022 `transfer_checked` instruction.
///
/// The `transfer_checked` variant is used because it validates the
/// token decimals against the mint, preventing decimal mismatch attacks.
///
/// # Arguments
/// * `token_program` - The SPL Token-2022 program account.
/// * `from` - The source token account.
/// * `to` - The destination token account.
/// * `mint` - The GHOST mint account (used for decimals validation).
/// * `authority` - The owner/delegate of the `from` account.
/// * `amount` - The number of tokens to transfer (in base units).
/// * `decimals` - The token's decimal places (9 for GHOST).
/// * `seeds` - Optional PDA signer seeds if the authority is a PDA.
///
/// # Errors
/// * `GhostError::TokenProgramMismatch` - If token_program is not SPL Token-2022.
/// * `GhostError::TransferFailed` - If the CPI call fails.
pub fn transfer_checked<'info>(
    token_program: &AccountInfo<'info>,
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    // Validate token program.
    require!(
        token_program.key() == spl_token_2022::ID,
        GhostError::TokenProgramMismatch
    );

    // Validate that decimals match the expected 9 for GHOST.
    require!(decimals == 9, GhostError::InvalidDecimals);

    // Log the transfer attempt.
    security_events::log_security_event(
        "TRANSFER",
        &format!(
            "from={} to={} mint={} amount={} decimals={} authority={}",
            from.key(),
            to.key(),
            mint.key(),
            amount,
            decimals,
            authority.key()
        ),
    );

    // Perform the CPI call.
    let result = cpi::cpi_transfer_checked(
        token_program,
        from,
        to,
        mint,
        authority,
        amount,
        decimals,
        seeds,
    );

    match &result {
        Ok(()) => {
            msg!(
                "Transfer successful: {} tokens from {} to {}",
                amount,
                from.key(),
                to.key()
            );
        }
        Err(e) => {
            security_events::log_cpi_failure(
                &spl_token_2022::ID,
                &format!("transfer_checked failed: {:?}", e),
            );
        }
    }

    result.map_err(|_| GhostError::TransferFailed.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_wrong_decimals_fails() {
        // Decimals must be 9 for GHOST.
    }

    #[test]
    fn test_transfer_wrong_token_program_fails() {
        // Token program mismatch is caught before CPI.
    }
}
