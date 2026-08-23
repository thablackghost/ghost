use anchor_lang::prelude::*;
use crate::errors::GhostError;

/// Validates that the given account is owned by the expected program/owner.
/// This is a critical security check to prevent fake account attacks.
pub fn validate_account_owner(account: &AccountInfo, expected_owner: &Pubkey) -> Result<()> {
    require!(
        account.owner == expected_owner,
        GhostError::InvalidAccountOwner
    );
    Ok(())
}

/// Validates a token account by checking its owner matches the expected owner
/// and that it belongs to the expected mint. This is three-fold:
/// 1. The account is owned by the SPL Token-2022 program.
/// 2. The account's mint field matches the expected mint.
/// 3. The account's owner field matches the expected token owner.
///
/// Safety: this performs deserialization of the token account data, which
/// is safe because we have already verified the account is owned by the
/// Token-2022 program.
pub fn validate_token_account(
    account: &AccountInfo,
    expected_mint: &Pubkey,
    expected_owner: &Pubkey,
) -> Result<()> {
    // Verify ownership by SPL Token-2022 program.
    require!(
        account.owner == &spl_token_2022::ID,
        GhostError::InvalidTokenAccountOwner
    );

    // Deserialize the token account data to inspect mint and owner.
    let token_account_data = account.data.borrow();
    let token_account = spl_token_2022::extension::StateWithExtensions::<
        spl_token_2022::state::Account,
    >::unpack(&token_account_data)
        .map_err(|_| GhostError::InvalidTokenAccountData)?;

    require!(
        &token_account.base.mint == expected_mint,
        GhostError::TokenAccountMintMismatch
    );

    require!(
        &token_account.base.owner == expected_owner,
        GhostError::TokenAccountOwnerMismatch
    );

    Ok(())
}

/// Validates that a mint account belongs to the expected mint pubkey
/// and is owned by the SPL Token-2022 program.
pub fn validate_mint(account: &AccountInfo, expected_mint: &Pubkey) -> Result<()> {
    require!(
        account.owner == &spl_token_2022::ID,
        GhostError::InvalidMintOwner
    );
    require!(
        account.key() == *expected_mint,
        GhostError::MintMismatch
    );
    Ok(())
}

/// Validates that the account's key matches the expected PDA.
/// This prevents passing in a different account that happens to have
/// the same owner/data layout.
pub fn validate_pda(account: &AccountInfo, expected_pda: &Pubkey) -> Result<()> {
    require!(
        account.key() == *expected_pda,
        GhostError::PdaMismatch
    );
    Ok(())
}

/// Validates that a program account has the expected program ID.
/// Used to verify CPI target programs are the legitimate SPL Token-2022
/// or other expected programs rather than a malicious look-alike.
pub fn validate_program_id(program: &AccountInfo, expected: &Pubkey) -> Result<()> {
    require!(
        program.key() == *expected,
        GhostError::InvalidProgramId
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_validate_pda_matches() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![0; 32];
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

        assert!(validate_pda(&account_info, &key).is_ok());
    }

    #[test]
    fn test_validate_pda_mismatch() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![0; 32];
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

        assert!(validate_pda(&account_info, &Pubkey::new_unique()).is_err());
    }
}
