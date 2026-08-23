use crate::errors::GhostError;
use anchor_lang::prelude::*;

/// Validates that the given account has signed the transaction.
/// This is a fundamental check to ensure the transaction was authorized
/// by the owner of the account.
pub fn validate_signer(account: &AccountInfo) -> Result<()> {
    require!(account.is_signer, GhostError::MissingSignature);
    Ok(())
}

/// Validates that a `Signer` account's key matches the expected pubkey.
/// This combines the signer check with an identity check in one call.
pub fn validate_signer_is(expected_signer: &Signer, expected_pubkey: &Pubkey) -> Result<()> {
    require!(
        expected_signer.key() == *expected_pubkey,
        GhostError::SignerMismatch
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_validate_signer_passes_when_signer() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![];
        let owner = Pubkey::default();

        let account_info = AccountInfo::new(
            &key,
            true, // is_signer
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        assert!(validate_signer(&account_info).is_ok());
    }

    #[test]
    fn test_validate_signer_fails_when_not_signer() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![];
        let owner = Pubkey::default();

        let account_info = AccountInfo::new(
            &key,
            false, // not a signer
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        assert!(validate_signer(&account_info).is_err());
    }
}
