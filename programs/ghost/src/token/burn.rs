use crate::errors::GhostError;
use crate::security::security_events;
use crate::token::cpi;
use anchor_lang::prelude::*;

/// Burns the specified `amount` of GHOST tokens from the `from` token account.
///
/// This function wraps the SPL Token-2022 `burn` instruction via CPI.
/// The burned tokens are permanently removed from circulation, decreasing
/// the total supply.
///
/// # Arguments
/// * `token_program` - The SPL Token-2022 program account.
/// * `mint` - The GHOST mint account (supply is decreased on burn).
/// * `from` - The token account to burn tokens from.
/// * `authority` - The owner/delegate authority of the `from` account.
/// * `amount` - The number of tokens to burn (in base units).
/// * `seeds` - Optional PDA signer seeds if the authority is a PDA.
///
/// # Errors
/// * `GhostError::TokenProgramMismatch` - If token_program is not SPL Token-2022.
/// * `GhostError::BurnFailed` - If the CPI call to burn fails (e.g., insufficient balance).
pub fn burn<'info>(
    token_program: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    from: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    amount: u64,
    seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    // Validate token program.
    require!(
        token_program.key() == spl_token_2022::ID,
        GhostError::TokenProgramMismatch
    );

    // Log the burn attempt.
    security_events::log_security_event(
        "BURN",
        &format!(
            "mint={} from={} amount={} authority={}",
            mint.key(),
            from.key(),
            amount,
            authority.key()
        ),
    );

    // Perform the CPI call.
    let result = cpi::cpi_burn(token_program, mint, from, authority, amount, seeds);

    match &result {
        Ok(()) => {
            msg!(
                "Burn successful: {} tokens burned from {}",
                amount,
                from.key()
            );
        }
        Err(e) => {
            security_events::log_cpi_failure(&spl_token_2022::ID, &format!("burn failed: {:?}", e));
        }
    }

    result.map_err(|_| GhostError::BurnFailed.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burn_wrong_token_program_fails() {
        // Token program mismatch is caught before CPI.
    }
}
