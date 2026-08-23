use crate::errors::GhostError;
use anchor_lang::prelude::*;

/// Validates that an account is owned by the specified program.
/// This is a fundamental ownership check used throughout the program.
pub fn validate_ownership(account: &AccountInfo, owner: &Pubkey) -> Result<()> {
    require!(account.owner == owner, GhostError::InvalidAccountOwner);
    Ok(())
}

/// Validates that an account is owned by the System Program.
/// Used to verify that new accounts being initialized are system-owned
/// before we take ownership of them.
pub fn validate_system_owned(account: &AccountInfo) -> Result<()> {
    require!(
        account.owner == &anchor_lang::solana_program::system_program::ID,
        GhostError::AccountNotSystemOwned
    );
    Ok(())
}

/// Validates that an account is owned by the SPL Token-2022 program.
/// Used to ensure token accounts passed to our program are legitimate
/// token accounts rather than arbitrarily crafted accounts.
pub fn validate_token_program_owned(account: &AccountInfo) -> Result<()> {
    require!(
        account.owner == &spl_token_2022::ID,
        GhostError::AccountNotTokenProgramOwned
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;
    use anchor_lang::solana_program::system_program;

    #[test]
    fn test_validate_system_owned_passes() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![0; 32];
        let owner = system_program::ID;

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

        assert!(validate_system_owned(&account_info).is_ok());
    }

    #[test]
    fn test_validate_system_owned_fails() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![0; 32];
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

        assert!(validate_system_owned(&account_info).is_err());
    }

    #[test]
    fn test_validate_token_program_owned_passes() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![0; 165]; // Minimum token account size
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

        assert!(validate_token_program_owned(&account_info).is_ok());
    }

    #[test]
    fn test_validate_token_program_owned_fails() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![0; 32];
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

        assert!(validate_token_program_owned(&account_info).is_err());
    }
}
