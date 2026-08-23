use anchor_lang::prelude::*;
use crate::errors::GhostError;

/// Validates that a CPI program account matches the expected program ID.
/// This is essential to prevent malicious programs from being passed in
/// during cross-program invocations.
pub fn validate_cpi_program(program: &AccountInfo, expected: &Pubkey) -> Result<()> {
    require!(
        program.key() == *expected,
        GhostError::InvalidCpiProgram
    );
    Ok(())
}

/// Validates that a mint account used in a CPI call matches the expected mint
/// and is owned by the SPL Token-2022 program.
pub fn validate_cpi_mint(mint: &AccountInfo, expected: &Pubkey) -> Result<()> {
    require!(
        mint.key() == *expected,
        GhostError::CpiMintMismatch
    );
    require!(
        mint.owner == &spl_token_2022::ID,
        GhostError::InvalidMintOwner
    );
    Ok(())
}

/// Validates a token account used in a CPI call.
/// Checks that the token account:
/// 1. Is owned by SPL Token-2022.
/// 2. Belongs to the expected mint.
/// 3. Is owned by the expected token owner.
pub fn validate_cpi_token_account(
    ta: &AccountInfo,
    expected_mint: &Pubkey,
    expected_owner: &Pubkey,
) -> Result<()> {
    require!(
        ta.owner == &spl_token_2022::ID,
        GhostError::InvalidTokenAccountOwner
    );

    let ta_data = ta.data.borrow();
    let token_account = spl_token_2022::extension::StateWithExtensions::<
        spl_token_2022::state::Account,
    >::unpack(&ta_data)
        .map_err(|_| GhostError::InvalidTokenAccountData)?;

    require!(
        &token_account.base.mint == expected_mint,
        GhostError::CpiTokenAccountMintMismatch
    );

    require!(
        &token_account.base.owner == expected_owner,
        GhostError::CpiTokenAccountOwnerMismatch
    );

    Ok(())
}

/// Validates that the CPI authority account's key matches the expected pubkey.
/// Used to ensure the correct authority is being passed for a CPI operation.
pub fn validate_cpi_authority(authority: &AccountInfo, expected: &Pubkey) -> Result<()> {
    require!(
        authority.key() == *expected,
        GhostError::CpiAuthorityMismatch
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_validate_cpi_program_matches() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![];
        let owner = Pubkey::default();

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

        assert!(validate_cpi_program(&account_info, &key).is_ok());
    }

    #[test]
    fn test_validate_cpi_program_mismatch() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![];
        let owner = Pubkey::default();

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

        assert!(validate_cpi_program(&account_info, &Pubkey::new_unique()).is_err());
    }
}
