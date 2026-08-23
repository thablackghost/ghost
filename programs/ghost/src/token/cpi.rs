use crate::errors::GhostError;
use anchor_lang::prelude::*;

/// CPI wrapper for SPL Token-2022 `mint_to`.
///
/// Performs a cross-program invocation to mint tokens.
/// The token program MUST be verified as the SPL Token-2022 program
/// BEFORE calling this function (validation is done at the caller level).
///
/// # Safety
/// The caller MUST validate that `token_program.key() == spl_token_2022::ID`.
/// This function trusts the caller's validation.
pub fn cpi_mint_to<'info>(
    token_program: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    amount: u64,
    seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    // Defense-in-depth: re-validate token program ID.
    require!(
        token_program.key() == spl_token_2022::ID,
        GhostError::TokenProgramMismatch
    );

    let ix = spl_token_2022::instruction::mint_to(
        &spl_token_2022::ID,
        &mint.key(),
        &to.key(),
        &authority.key(),
        &[],
        amount,
    )
    .map_err(|_| GhostError::CpiInstructionBuildFailed)?;

    let account_infos = vec![
        mint.clone(),
        to.clone(),
        authority.clone(),
        token_program.clone(),
    ];

    match seeds {
        Some(signer_seeds) => {
            solana_program::program::invoke_signed(&ix, &account_infos, signer_seeds)?;
        }
        None => {
            solana_program::program::invoke(&ix, &account_infos)?;
        }
    }

    Ok(())
}

/// CPI wrapper for SPL Token-2022 `burn`.
///
/// Performs a cross-program invocation to burn tokens from a token account.
/// The token program MUST be verified as the SPL Token-2022 program
/// BEFORE calling this function.
pub fn cpi_burn<'info>(
    token_program: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    from: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    amount: u64,
    seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    require!(
        token_program.key() == spl_token_2022::ID,
        GhostError::TokenProgramMismatch
    );

    let ix = spl_token_2022::instruction::burn(
        &spl_token_2022::ID,
        &from.key(),
        &mint.key(),
        &authority.key(),
        &[],
        amount,
    )
    .map_err(|_| GhostError::CpiInstructionBuildFailed)?;

    let account_infos = vec![
        from.clone(),
        mint.clone(),
        authority.clone(),
        token_program.clone(),
    ];

    match seeds {
        Some(signer_seeds) => {
            solana_program::program::invoke_signed(&ix, &account_infos, signer_seeds)?;
        }
        None => {
            solana_program::program::invoke(&ix, &account_infos)?;
        }
    }

    Ok(())
}

/// CPI wrapper for SPL Token-2022 `transfer_checked`.
///
/// Performs a cross-program invocation to transfer tokens with decimal checking.
/// This is preferred over `transfer` because it validates the token's decimals
/// against the mint, preventing decimal mismatch attacks.
pub fn cpi_transfer_checked<'info>(
    token_program: &AccountInfo<'info>,
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    require!(
        token_program.key() == spl_token_2022::ID,
        GhostError::TokenProgramMismatch
    );

    let ix = spl_token_2022::instruction::transfer_checked(
        &spl_token_2022::ID,
        &from.key(),
        &mint.key(),
        &to.key(),
        &authority.key(),
        &[],
        amount,
        decimals,
    )
    .map_err(|_| GhostError::CpiInstructionBuildFailed)?;

    let account_infos = vec![
        from.clone(),
        mint.clone(),
        to.clone(),
        authority.clone(),
        token_program.clone(),
    ];

    match seeds {
        Some(signer_seeds) => {
            solana_program::program::invoke_signed(&ix, &account_infos, signer_seeds)?;
        }
        None => {
            solana_program::program::invoke(&ix, &account_infos)?;
        }
    }

    Ok(())
}

/// CPI wrapper for SPL Token-2022 `approve`.
///
/// Approves a delegate to spend up to `amount` tokens from the `source` account.
/// The delegate can then transfer or burn tokens on behalf of the owner.
pub fn cpi_approve<'info>(
    token_program: &AccountInfo<'info>,
    source: &AccountInfo<'info>,
    delegate: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    amount: u64,
    seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    require!(
        token_program.key() == spl_token_2022::ID,
        GhostError::TokenProgramMismatch
    );

    let ix = spl_token_2022::instruction::approve(
        &spl_token_2022::ID,
        &source.key(),
        &delegate.key(),
        &authority.key(),
        &[],
        amount,
    )
    .map_err(|_| GhostError::CpiInstructionBuildFailed)?;

    let account_infos = vec![
        source.clone(),
        delegate.clone(),
        authority.clone(),
        token_program.clone(),
    ];

    match seeds {
        Some(signer_seeds) => {
            solana_program::program::invoke_signed(&ix, &account_infos, signer_seeds)?;
        }
        None => {
            solana_program::program::invoke(&ix, &account_infos)?;
        }
    }

    Ok(())
}

/// CPI wrapper for SPL Token-2022 `revoke`.
///
/// Revokes a previously granted delegate authority on the `source` account.
/// After revocation, the delegate can no longer spend tokens on behalf of the owner.
pub fn cpi_revoke<'info>(
    token_program: &AccountInfo<'info>,
    source: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    require!(
        token_program.key() == spl_token_2022::ID,
        GhostError::TokenProgramMismatch
    );

    let ix = spl_token_2022::instruction::revoke(
        &spl_token_2022::ID,
        &source.key(),
        &authority.key(),
        &[],
    )
    .map_err(|_| GhostError::CpiInstructionBuildFailed)?;

    let account_infos = vec![source.clone(), authority.clone(), token_program.clone()];

    match seeds {
        Some(signer_seeds) => {
            solana_program::program::invoke_signed(&ix, &account_infos, signer_seeds)?;
        }
        None => {
            solana_program::program::invoke(&ix, &account_infos)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_cpi_mint_to_wrong_program() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![];
        let owner = Pubkey::default();

        let token_program = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        let mint = AccountInfo::new(
            &Pubkey::new_unique(),
            false,
            true,
            &mut lamports,
            &mut vec![],
            &spl_token_2022::ID,
            false,
            0,
        );

        let to = AccountInfo::new(
            &Pubkey::new_unique(),
            false,
            true,
            &mut lamports,
            &mut vec![],
            &spl_token_2022::ID,
            false,
            0,
        );

        let authority = AccountInfo::new(
            &Pubkey::new_unique(),
            true,
            false,
            &mut lamports,
            &mut vec![],
            &owner,
            false,
            0,
        );

        // Wrong token program should fail the token_program check.
        assert!(cpi_mint_to(&token_program, &mint, &to, &authority, 100, None).is_err());
    }
}
